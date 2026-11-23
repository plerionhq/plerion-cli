#![cfg(unix)]

use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Stdio};
use tempfile::TempDir;

fn configure(home: &std::path::Path) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_plerion"))
        .arg("configure")
        .env("HOME", home)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .expect("failed to execute plerion binary");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"\ndummy-key\n\n\n")
        .unwrap();
    assert!(child.wait().unwrap().success());
}

fn mode(path: &std::path::Path) -> u32 {
    fs::metadata(path).unwrap().permissions().mode() & 0o777
}

#[test]
fn test_configure_creates_credentials_readable_only_by_owner() {
    let home = TempDir::new().unwrap();
    configure(home.path());
    assert_eq!(mode(&home.path().join(".plerion/credentials")), 0o600);
}

#[test]
fn test_configure_tightens_existing_credentials_file() {
    let home = TempDir::new().unwrap();
    let dir = home.path().join(".plerion");
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("credentials");
    fs::write(&path, "[old]\napi_key = old-key\n").unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();

    configure(home.path());

    assert_eq!(mode(&path), 0o600);
    let content = fs::read_to_string(&path).unwrap();
    assert!(content.contains("old-key"));
    assert!(content.contains("dummy-key"));
}
