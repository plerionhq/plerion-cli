use std::process::Command;

/// Subprocess tests for `plerion integrations add aws`. Only offline paths
/// (--dry-run, flag validation) run here; the orchestration itself is covered
/// at the lib level in onboard_test.rs where AWS is mockable.
fn run_plerion(args: &[&str]) -> std::process::Output {
    let binary = env!("CARGO_BIN_EXE_plerion");
    Command::new(binary)
        .args(args)
        .env("PLERION_API_KEY", "test-key")
        .env("PLERION_REGION", "au")
        .env("NO_COLOR", "1")
        .output()
        .expect("failed to execute plerion binary")
}

#[test]
fn test_cli_add_aws_dry_run_is_offline_and_redacts_token() {
    let output = run_plerion(&[
        "integrations",
        "add",
        "aws",
        "--dry-run",
        "--service-account-id",
        "222222222222",
    ]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout.contains("dry run"), "{stdout}");
    assert!(stdout.contains("Plerion-Integration"), "{stdout}");
    assert!(stdout.contains("PlerionManagedServiceAccount"), "{stdout}");
    assert!(stdout.contains("222222222222"), "{stdout}");
    assert!(stdout.contains("redacted"), "{stdout}");
    // The real token value can never appear: dry-run makes no network calls.
    assert!(!stdout.contains("tmp-"), "{stdout}");
    assert!(stdout.contains("au.api.plerion.com"), "{stdout}");
}

#[test]
fn test_cli_add_aws_dry_run_no_auto_update() {
    let output = run_plerion(&[
        "integrations",
        "add",
        "aws",
        "--dry-run",
        "--no-auto-update",
        "--stack-name",
        "CustomStack",
    ]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    assert!(stdout.contains("EnableAutoUpdate      false"), "{stdout}");
    assert!(stdout.contains("CustomStack"), "{stdout}");
}

#[test]
fn test_cli_add_aws_rejects_bad_kms_mode() {
    let output = run_plerion(&[
        "integrations",
        "add",
        "aws",
        "--dry-run",
        "--kms-key-access-mode",
        "SOME_KEYS",
    ]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("SOME_KEYS"), "{stderr}");
}

#[test]
fn test_cli_add_aws_stack_name_prefix_rule_exit_code() {
    // Not dry-run: option validation runs first in onboard::run and must exit 1
    // before any network access is attempted... except validation happens after
    // client construction, which is offline. Use --validate-only with a bad
    // stack name; the InvalidOptions error maps to exit code 1.
    let output = run_plerion(&[
        "integrations",
        "add",
        "aws",
        "--validate-only",
        "--stack-name",
        "NotPlerion",
        "--service-account-id",
        "222222222222",
        "--endpoint-url",
        "http://127.0.0.1:1", // unreachable; must fail before any request
    ]);
    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Plerion-"), "{stderr}");
}

#[test]
fn test_cli_add_aws_help_lists_flags() {
    let output = run_plerion(&["integrations", "add", "aws", "--help"]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    for flag in [
        "--aws-profile",
        "--aws-region",
        "--expect-account-id",
        "--validate-only",
        "--dry-run",
        "--allow-existing",
        "--strict-preflight",
        "--no-auto-update",
    ] {
        assert!(stdout.contains(flag), "missing {flag} in help: {stdout}");
    }
    // Hidden escape hatch stays hidden.
    assert!(!stdout.contains("--plerion-account-id"), "{stdout}");
}
