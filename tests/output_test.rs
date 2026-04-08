use plerion::output::TableRenderable;
use serde::{Deserialize, Serialize};

/// Simple test struct that implements TableRenderable.
#[derive(Serialize, Deserialize, Clone)]
struct TestItem {
    id: String,
    name: String,
    severity: String,
}

impl TableRenderable for TestItem {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "NAME", "SEVERITY"]
    }
    fn row(&self) -> Vec<String> {
        vec![self.id.clone(), self.name.clone(), self.severity.clone()]
    }
}

fn make_items() -> Vec<TestItem> {
    vec![
        TestItem { id: "1".to_string(), name: "bucket-acl".to_string(), severity: "CRITICAL".to_string() },
        TestItem { id: "2".to_string(), name: "sg-open".to_string(), severity: "HIGH".to_string() },
    ]
}

#[test]
fn test_json_output_is_valid() {
    let items = make_items();
    let json_val = serde_json::to_value(&items).unwrap();
    // Must be an array with 2 elements
    assert!(json_val.is_array());
    assert_eq!(json_val.as_array().unwrap().len(), 2);
}

#[test]
fn test_yaml_output_is_valid() {
    let items = make_items();
    let json_val = serde_json::to_value(&items).unwrap();
    let yaml_str = serde_yaml::to_string(&json_val).unwrap();
    // Round-trip back
    let back: serde_json::Value = serde_yaml::from_str(&yaml_str).unwrap();
    assert_eq!(back.as_array().unwrap().len(), 2);
}

#[test]
fn test_text_headers_and_rows() {
    let item = TestItem { id: "42".to_string(), name: "test".to_string(), severity: "LOW".to_string() };
    let headers = TestItem::headers();
    assert_eq!(headers, vec!["ID", "NAME", "SEVERITY"]);
    let row = item.row();
    assert_eq!(row, vec!["42", "test", "LOW"]);
}

#[test]
fn test_jmespath_query() {
    use plerion::output::apply_query;
    let value = serde_json::json!([{"id": "1", "severity": "CRITICAL"}, {"id": "2", "severity": "HIGH"}]);
    let result = apply_query(value, "[0].severity").unwrap();
    assert_eq!(result, serde_json::json!("CRITICAL"));
}

#[test]
fn test_jmespath_filter_query() {
    use plerion::output::apply_query;
    let value = serde_json::json!([
        {"id": "1", "severity": "CRITICAL"},
        {"id": "2", "severity": "HIGH"}
    ]);
    let result = apply_query(value, "[?severity=='CRITICAL'].id").unwrap();
    assert_eq!(result, serde_json::json!(["1"]));
}

#[test]
fn test_invalid_jmespath_query_returns_error() {
    use plerion::output::apply_query;
    let value = serde_json::json!({"key": "value"});
    let result = apply_query(value, "[invalid query!!!");
    assert!(result.is_err());
}
