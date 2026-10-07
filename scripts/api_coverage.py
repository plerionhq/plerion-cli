#!/usr/bin/env python3
"""Report customer API operations in the published API reference that the CLI does not call.

Usage:
  python3 scripts/api_coverage.py                 # fetch the live spec
  python3 scripts/api_coverage.py --spec FILE     # use a local copy
  python3 scripts/api_coverage.py --list-covered  # print what the CLI calls

Exit codes: 0 all covered, 1 operations uncovered, 2 the check itself failed
(spec unreadable, or a client call whose path the extractor cannot resolve).
"""

import argparse
import re
import sys
import urllib.request
from pathlib import Path

SPEC_URL = "https://docs.plerion.com/api-reference/openapi.yaml"
PATH_PREFIX = "/v1/tenant"
METHODS = ("get", "post", "put", "patch", "delete")
ROOT = Path(__file__).resolve().parent.parent
ENDPOINTS_DIR = ROOT / "src" / "api" / "endpoints"
# Requests that need special headers (the IaC zip upload) are built in the client itself.
CLIENT_FILE = ROOT / "src" / "api" / "client.rs"
BASE_URL_ARG = "{}{}"
IGNORE_FILE = ROOT / "api-coverage-ignore.txt"


def normalise(path):
    path = path.split("?", 1)[0]
    return re.sub(r"\{[^{}]*\}", "{}", path)


# --- Spec parsing -----------------------------------------------------------
# The spec is YAML but only a narrow, regular slice is needed: path keys at
# indent 2 under `paths:`, method keys at indent 4, and scalar fields at
# indent 6 under each method. Block scalars are always indented deeper than
# their key, so they never match these exact indents.

def _scalar(value):
    value = value.strip()
    if len(value) >= 2 and value[0] == value[-1] and value[0] in "'\"":
        value = value[1:-1]
    return value


def parse_spec(text):
    ops = []
    in_paths = False
    path = None
    op = None
    for line in text.splitlines():
        if not line.strip() or line.lstrip().startswith("#"):
            continue
        indent = len(line) - len(line.lstrip(" "))
        if indent == 0:
            in_paths = line.rstrip() == "paths:"
            path = op = None
            continue
        if not in_paths:
            continue
        if indent == 2:
            m = re.match(r"""\s*['"]?(/[^'"]*?)['"]?:\s*$""", line)
            path = m.group(1) if m else None
            op = None
        elif indent == 4 and path:
            key = line.strip().split(":", 1)[0]
            if key in METHODS:
                op = {"method": key.upper(), "path": path, "operationId": "", "summary": ""}
                ops.append(op)
            else:
                op = None
        elif indent == 6 and op is not None:
            key, _, value = line.strip().partition(":")
            if key in ("operationId", "summary"):
                op[key] = _scalar(value)
    if not ops:
        print("error: no operations found in the spec's paths block", file=sys.stderr)
        sys.exit(2)
    return ops


# --- CLI call extraction ----------------------------------------------------

STR = r'"((?:[^"\\]|\\.)*)"'
CALL_RE = re.compile(r"\b(\w+)\s*\.(get|post|put|patch|delete)\(\s*&?\s*")
CONST_RE = re.compile(r"\bconst\s+(\w+)\s*:\s*&(?:'static\s+)?str\s*=\s*" + STR)
# A helper that builds a path: `fn name(...) -> ... { Ok(format!("..."` or `format!("..."`.
FN_RE = re.compile(r"\bfn\s+(\w+)\s*\([^)]*\)\s*->[^{]*\{\s*(?:Ok\(\s*)?format!\(\s*" + STR)
LET_RE = re.compile(r"\blet\s+(?:mut\s+)?(\w+)\s*(?::[^=]+)?=\s*")


def strip_comments(src):
    out = []
    for line in src.splitlines():
        # Drop `//` comments that are not inside a string literal.
        in_str = False
        i = 0
        while i < len(line):
            c = line[i]
            if c == "\\" and in_str:
                i += 2
                continue
            if c == '"':
                in_str = not in_str
            elif not in_str and line.startswith("//", i):
                line = line[:i]
                break
            i += 1
        out.append(line)
    return "\n".join(out)


def _literal_at(src, pos, consts, fns):
    """Resolve the path expression starting at pos without following variables."""
    rest = src[pos:]
    m = re.match(STR, rest)
    if m:
        return m.group(1)
    m = re.match(r"format!\(\s*" + STR, rest)
    if m:
        return m.group(1)
    m = re.match(r"(\w+)\s*\(", rest)
    if m and m.group(1) in fns:
        return fns[m.group(1)]
    m = re.match(r"(\w+)\b", rest)
    if m and m.group(1) in consts:
        return consts[m.group(1)]
    return None


def extract_calls(src, receiver_name="client"):
    """Return ([(METHOD, raw_path)], [unresolved call descriptions]).

    In client.rs the receiver is the inner HTTP client and paths carry a
    leading `{}` for the base URL; the generic get/post/... wrappers pass
    `"{}{}"` and are skipped.
    """
    src = strip_comments(src)
    consts = {m.group(1): m.group(2) for m in CONST_RE.finditer(src)}
    fns = {m.group(1): m.group(2) for m in FN_RE.finditer(src)}
    calls, unresolved = [], []
    for m in CALL_RE.finditer(src):
        receiver, method = m.group(1), m.group(2)
        # Method chains split over lines put `client` on the line above.
        if receiver != receiver_name:
            continue
        pos = m.end()
        path = _literal_at(src, pos, consts, fns)
        if path is None:
            var = re.match(r"(\w+)\s*\)", src[pos:])
            if var:
                # The nearest earlier `let var =` in the same file is the binding in scope.
                lets = [l for l in LET_RE.finditer(src, 0, pos) if l.group(1) == var.group(1)]
                if lets:
                    path = _literal_at(src, lets[-1].end(), consts, fns)
        if path is None:
            line = src.count("\n", 0, m.start()) + 1
            unresolved.append(f"line {line}: client.{method}({src[pos:pos + 40].splitlines()[0]}")
            continue
        if receiver_name != "client":
            if path == BASE_URL_ARG:
                continue
            path = path[2:] if path.startswith("{}") else path
        calls.append((method.upper(), path))
    return calls, unresolved


def covered_operations():
    covered = {}
    problems = []
    sources = [(f, "client") for f in sorted(ENDPOINTS_DIR.glob("*.rs"))] + [(CLIENT_FILE, "inner")]
    for f, receiver in sources:
        calls, unresolved = extract_calls(f.read_text(), receiver)
        rel = f.relative_to(ROOT)
        for method, path in calls:
            covered.setdefault((method, normalise(path)), f"{rel}: {method} {path}")
        problems += [f"{rel} {u}" for u in unresolved]
    return covered, problems


# --- Ignore list ------------------------------------------------------------

def read_ignore():
    ignored = {}
    if not IGNORE_FILE.exists():
        return ignored
    for line in IGNORE_FILE.read_text().splitlines():
        op_id, _, reason = line.partition("#")
        op_id = op_id.strip()
        if op_id:
            ignored[op_id] = reason.strip()
    return ignored


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--spec", help="local OpenAPI YAML file (default: fetch %s)" % SPEC_URL)
    ap.add_argument("--list-covered", action="store_true", help="print the operations the CLI calls and exit")
    args = ap.parse_args()

    covered, problems = covered_operations()
    if problems:
        print("error: could not resolve the path of these client calls:", file=sys.stderr)
        for p in problems:
            print("  " + p, file=sys.stderr)
        return 2

    if args.list_covered:
        for key in sorted(covered, key=lambda k: (k[1], k[0])):
            print(covered[key])
        print(f"{len(covered)} operations", file=sys.stderr)
        return 0

    source = args.spec or SPEC_URL
    try:
        if args.spec:
            text = Path(args.spec).read_text()
        else:
            with urllib.request.urlopen(SPEC_URL, timeout=60) as resp:
                text = resp.read().decode("utf-8")
    except OSError as e:
        print(f"error: could not read {source}: {e}", file=sys.stderr)
        return 2

    ops = [o for o in parse_spec(text) if o["path"] == PATH_PREFIX or o["path"].startswith(PATH_PREFIX + "/")]
    ignored = read_ignore()
    spec_ids = {o["operationId"] for o in ops}

    uncovered = [
        o for o in ops
        if (o["method"], normalise(o["path"])) not in covered and o["operationId"] not in ignored
    ]
    stale = sorted(i for i in ignored if i not in spec_ids)

    print(f"{len(ops)} customer API operations in the reference, "
          f"{len(ops) - len(uncovered)} called by the CLI or ignored, {len(uncovered)} uncovered.")
    print()
    if uncovered:
        print("| operationId | Method | Path | Summary |")
        print("|---|---|---|---|")
        for o in sorted(uncovered, key=lambda o: (o["path"], o["method"])):
            print(f"| `{o['operationId']}` | {o['method']} | `{o['path']}` | {o['summary']} |")
        print()
    if stale:
        print("Warning: these entries in `api-coverage-ignore.txt` are no longer in the reference "
              "and can be removed: " + ", ".join(f"`{s}`" for s in stale))
        print()
    return 1 if uncovered else 0


if __name__ == "__main__":
    sys.exit(main())
