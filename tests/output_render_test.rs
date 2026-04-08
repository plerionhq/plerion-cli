use plerion::output::{self, OutputFormat, TableRenderable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: String,
    name: String,
    value: i32,
}

impl TableRenderable for Item {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "NAME", "VALUE"]
    }
    fn row(&self) -> Vec<String> {
        vec![self.id.clone(), self.name.clone(), self.value.to_string()]
    }
}

fn sample_items() -> Vec<Item> {
    vec![
        Item { id: "1".to_string(), name: "alpha".to_string(), value: 100 },
        Item { id: "2".to_string(), name: "beta".to_string(), value: 200 },
    ]
}

fn single_item() -> Item {
    Item { id: "42".to_string(), name: "test".to_string(), value: 999 }
}

// --- render_list tests ---

#[test]
fn test_render_list_json() {
    let items = sample_items();
    // Should not panic
    output::render_list(&items, OutputFormat::Json, None, false).unwrap();
}

#[test]
fn test_render_list_yaml() {
    let items = sample_items();
    output::render_list(&items, OutputFormat::Yaml, None, false).unwrap();
}

#[test]
fn test_render_list_table() {
    let items = sample_items();
    output::render_list(&items, OutputFormat::Table, None, false).unwrap();
}

#[test]
fn test_render_list_table_no_color() {
    let items = sample_items();
    output::render_list(&items, OutputFormat::Table, None, true).unwrap();
}

#[test]
fn test_render_list_text() {
    let items = sample_items();
    output::render_list(&items, OutputFormat::Text, None, false).unwrap();
}

#[test]
fn test_render_list_empty_table() {
    let items: Vec<Item> = vec![];
    output::render_list(&items, OutputFormat::Table, None, false).unwrap();
}

#[test]
fn test_render_list_with_query() {
    let items = sample_items();
    output::render_list(&items, OutputFormat::Json, Some("[0].name"), false).unwrap();
}

// --- render (single item) tests ---

#[test]
fn test_render_single_json() {
    let item = single_item();
    output::render(&item, OutputFormat::Json, None, false).unwrap();
}

#[test]
fn test_render_single_yaml() {
    let item = single_item();
    output::render(&item, OutputFormat::Yaml, None, false).unwrap();
}

#[test]
fn test_render_single_table() {
    let item = single_item();
    output::render(&item, OutputFormat::Table, None, false).unwrap();
}

#[test]
fn test_render_single_table_no_color() {
    let item = single_item();
    output::render(&item, OutputFormat::Table, None, true).unwrap();
}

#[test]
fn test_render_single_text() {
    let item = single_item();
    output::render(&item, OutputFormat::Text, None, false).unwrap();
}

#[test]
fn test_render_single_with_query() {
    let item = single_item();
    output::render(&item, OutputFormat::Json, Some("name"), false).unwrap();
}

// --- render_json_value tests ---

#[test]
fn test_render_json_value_json() {
    let val = serde_json::json!({"key": "value", "count": 42});
    output::render_json_value(&val, OutputFormat::Json, None).unwrap();
}

#[test]
fn test_render_json_value_yaml() {
    let val = serde_json::json!({"key": "value"});
    output::render_json_value(&val, OutputFormat::Yaml, None).unwrap();
}

#[test]
fn test_render_json_value_table_falls_through_to_json() {
    let val = serde_json::json!({"nested": {"data": [1, 2, 3]}});
    output::render_json_value(&val, OutputFormat::Table, None).unwrap();
}

#[test]
fn test_render_json_value_text_falls_through_to_json() {
    let val = serde_json::json!("plain string");
    output::render_json_value(&val, OutputFormat::Text, None).unwrap();
}

#[test]
fn test_render_json_value_with_query() {
    let val = serde_json::json!({"data": [{"id": 1}, {"id": 2}]});
    output::render_json_value(&val, OutputFormat::Json, Some("data[0].id")).unwrap();
}

// --- apply_query edge cases ---

#[test]
fn test_apply_query_returns_null_for_missing_path() {
    let val = serde_json::json!({"data": "hello"});
    let result = output::apply_query(val, "nonexistent").unwrap();
    assert_eq!(result, serde_json::Value::Null);
}

#[test]
fn test_apply_query_complex_filter() {
    let val = serde_json::json!({
        "data": [
            {"name": "a", "score": 10},
            {"name": "b", "score": 20},
            {"name": "c", "score": 5}
        ]
    });
    let result = output::apply_query(val, "data[?score > `10`].name").unwrap();
    assert_eq!(result, serde_json::json!(["b"]));
}

#[test]
fn test_apply_query_length_function() {
    let val = serde_json::json!({"items": [1, 2, 3, 4, 5]});
    let result = output::apply_query(val, "length(items)").unwrap();
    assert_eq!(result, serde_json::json!(5));
}

// --- text_row default impl ---

#[test]
fn test_text_row_default_delegates_to_row() {
    let item = single_item();
    assert_eq!(item.text_row(), item.row());
}

// --- OutputFormat edge cases ---

#[test]
fn test_output_format_case_insensitive() {
    use std::str::FromStr;
    assert_eq!(OutputFormat::from_str("TABLE").unwrap(), OutputFormat::Table);
    assert_eq!(OutputFormat::from_str("JSON").unwrap(), OutputFormat::Json);
    assert_eq!(OutputFormat::from_str("Yaml").unwrap(), OutputFormat::Yaml);
    assert_eq!(OutputFormat::from_str("Text").unwrap(), OutputFormat::Text);
}

#[test]
fn test_output_format_invalid() {
    use std::str::FromStr;
    let result = OutputFormat::from_str("csv");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unknown output format"));
}
