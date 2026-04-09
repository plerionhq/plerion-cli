use plerion::output::table::{colorize_severity, colorize_status};

#[test]
fn test_colorize_severity_no_color() {
    assert_eq!(colorize_severity("CRITICAL", true), "CRITICAL");
    assert_eq!(colorize_severity("HIGH", true), "HIGH");
    assert_eq!(colorize_severity("MEDIUM", true), "MEDIUM");
    assert_eq!(colorize_severity("LOW", true), "LOW");
    assert_eq!(colorize_severity("INFORMATIONAL", true), "INFORMATIONAL");
    assert_eq!(colorize_severity("UNKNOWN", true), "UNKNOWN");
}

#[test]
fn test_colorize_severity_returns_plain_text() {
    // colorize_severity now returns plain text; Cell::fg() handles coloring
    assert_eq!(colorize_severity("CRITICAL", false), "CRITICAL");
    assert_eq!(colorize_severity("HIGH", false), "HIGH");
    assert_eq!(colorize_severity("MEDIUM", false), "MEDIUM");
    assert_eq!(colorize_severity("LOW", false), "LOW");
    assert_eq!(colorize_severity("INFORMATIONAL", false), "INFORMATIONAL");
}

#[test]
fn test_colorize_severity_unknown_value() {
    assert_eq!(colorize_severity("SOMETHING_ELSE", false), "SOMETHING_ELSE");
}

#[test]
fn test_colorize_severity_case_insensitive() {
    let result = colorize_severity("critical", false);
    assert_eq!(result, "critical");
}

#[test]
fn test_colorize_status_no_color() {
    assert_eq!(colorize_status("FAILED", true), "FAILED");
    assert_eq!(colorize_status("OPEN", true), "OPEN");
    assert_eq!(colorize_status("PASSED", true), "PASSED");
    assert_eq!(colorize_status("RESOLVED", true), "RESOLVED");
    assert_eq!(colorize_status("ACTIVE", true), "ACTIVE");
}

#[test]
fn test_colorize_status_returns_plain_text() {
    // colorize_status now returns plain text; Cell::fg() handles coloring
    assert_eq!(colorize_status("FAILED", false), "FAILED");
    assert_eq!(colorize_status("PASSED", false), "PASSED");
    assert_eq!(colorize_status("OPEN", false), "OPEN");
    assert_eq!(colorize_status("RESOLVED", false), "RESOLVED");
    assert_eq!(colorize_status("ACTIVE", false), "ACTIVE");
    assert_eq!(colorize_status("DISMISSED_ACCEPTED", false), "DISMISSED_ACCEPTED");
    assert_eq!(colorize_status("DISMISSED_NOT_A_RISK", false), "DISMISSED_NOT_A_RISK");
}

#[test]
fn test_colorize_status_delete_states() {
    assert_eq!(colorize_status("DELETE FAILED", false), "DELETE FAILED");
    assert_eq!(colorize_status("DELETE IN PROGRESS", false), "DELETE IN PROGRESS");
}

#[test]
fn test_colorize_status_unknown_value() {
    assert_eq!(colorize_status("OTHER", false), "OTHER");
}
