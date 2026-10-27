"""Check the resolved TLS dependencies used by release builds."""
import json
import subprocess

metadata = json.loads(subprocess.check_output(
    ["cargo", "metadata", "--locked", "--format-version", "1"], text=True))
minimum = {"rustls-webpki": (0, 103, 16), "webpki-roots": (1, 0, 10)}
for name, floor in minimum.items():
    versions = [p["version"] for p in metadata["packages"] if p["name"] == name]
    assert versions, f"Missing {name}"
    for version in versions:
        assert "-" not in version, f"Unverified prerelease {name} {version}"
        assert tuple(map(int, version.split("."))) >= floor, f"Affected {name} {version}"
        print(f"{name} {version}")
