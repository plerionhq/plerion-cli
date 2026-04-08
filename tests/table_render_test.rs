use plerion::output::table::{colorize_severity, colorize_status, render_list};
use plerion::output::TableRenderable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct TestRow {
    col_a: String,
    col_b: String,
}

impl TableRenderable for TestRow {
    fn headers() -> Vec<&'static str> {
        vec!["COL_A", "COL_B"]
    }
    fn row(&self) -> Vec<String> {
        vec![self.col_a.clone(), self.col_b.clone()]
    }
}

#[test]
fn test_render_list_prints_table() {
    let items = vec![
        TestRow { col_a: "hello".to_string(), col_b: "world".to_string() },
        TestRow { col_a: "foo".to_string(), col_b: "bar".to_string() },
    ];
    // Should not panic
    render_list(&items, false);
}

#[test]
fn test_render_list_no_color() {
    let items = vec![
        TestRow { col_a: "a".to_string(), col_b: "b".to_string() },
    ];
    render_list(&items, true);
}

#[test]
fn test_render_list_empty() {
    let items: Vec<TestRow> = vec![];
    render_list(&items, false);
}

// --- Severity colorize returns plain text (color is applied via Cell::fg in renderer) ---

#[test]
fn test_colorize_severity_returns_plain_text() {
    // colorize_severity now returns plain text; actual coloring done by Cell::fg()
    assert_eq!(colorize_severity("CRITICAL", false), "CRITICAL");
    assert_eq!(colorize_severity("HIGH", false), "HIGH");
    assert_eq!(colorize_severity("MEDIUM", false), "MEDIUM");
    assert_eq!(colorize_severity("LOW", false), "LOW");
    assert_eq!(colorize_severity("INFORMATIONAL", false), "INFORMATIONAL");
}

#[test]
fn test_colorize_severity_all_values_no_color() {
    assert_eq!(colorize_severity("CRITICAL", true), "CRITICAL");
    assert_eq!(colorize_severity("HIGH", true), "HIGH");
    assert_eq!(colorize_severity("MEDIUM", true), "MEDIUM");
    assert_eq!(colorize_severity("LOW", true), "LOW");
    assert_eq!(colorize_severity("INFORMATIONAL", true), "INFORMATIONAL");
    assert_eq!(colorize_severity("UNKNOWN", true), "UNKNOWN");
}

#[test]
fn test_colorize_severity_unknown_no_ansi() {
    let result = colorize_severity("SOMETHING", false);
    // Unknown values should NOT have ANSI codes, so length equals original
    assert_eq!(result, "SOMETHING");
}

// --- Status colorize edge cases ---

#[test]
fn test_colorize_status_returns_plain_text() {
    // colorize_status now returns plain text; actual coloring done by Cell::fg()
    for status in &["FAILED", "OPEN", "DELETE FAILED", "DELETE IN PROGRESS",
                     "PASSED", "RESOLVED", "ACTIVE",
                     "DISMISSED_ACCEPTED", "DISMISSED_NOT_A_RISK", "PENDING"] {
        let result = colorize_status(status, false);
        assert_eq!(result, *status);
    }
}

#[test]
fn test_colorize_status_case_insensitive() {
    let result = colorize_status("failed", false);
    assert_eq!(result, "failed");
}
