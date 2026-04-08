pub mod json;
pub mod table;
pub mod text;
pub mod yaml;

use crate::error::PlerionError;
use serde::Serialize;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Table,
    Json,
    Yaml,
    Text,
}

impl FromStr for OutputFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "table" => Ok(Self::Table),
            "json" => Ok(Self::Json),
            "yaml" => Ok(Self::Yaml),
            "text" => Ok(Self::Text),
            other => Err(format!("Unknown output format: {other}")),
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Table => write!(f, "table"),
            Self::Json => write!(f, "json"),
            Self::Yaml => write!(f, "yaml"),
            Self::Text => write!(f, "text"),
        }
    }
}

/// Trait for types that can render themselves as a table.
pub trait TableRenderable {
    fn headers() -> Vec<&'static str>;
    fn row(&self) -> Vec<String>;
    fn text_row(&self) -> Vec<String> {
        self.row()
    }
}

/// Apply a JMESPath query to a JSON value.
pub fn apply_query(value: serde_json::Value, query: &str) -> Result<serde_json::Value, PlerionError> {
    let expr = jmespatch::compile(query)
        .map_err(|e| PlerionError::ParseError(format!("Invalid JMESPath query: {e}")))?;

    // Convert serde_json::Value → jmespatch::Variable via JSON string
    let json_str = serde_json::to_string(&value)
        .map_err(|e| PlerionError::ParseError(e.to_string()))?;
    let data = jmespatch::Variable::from_json(&json_str)
        .map_err(|e| PlerionError::ParseError(format!("Failed to parse for JMESPath: {e}")))?;

    let result = expr
        .search(data)
        .map_err(|e| PlerionError::ParseError(format!("JMESPath search failed: {e}")))?;

    // Convert result Rc<Variable> back to serde_json::Value via Display (JSON repr)
    let result_str = format!("{result}");
    serde_json::from_str(&result_str)
        .map_err(|e| PlerionError::ParseError(format!("Failed to parse JMESPath result: {e}: {result_str}")))
}

/// Render a serializable value in the requested format.
pub fn render<T>(value: &T, format: OutputFormat, query: Option<&str>, no_color: bool) -> Result<(), PlerionError>
where
    T: Serialize + TableRenderable,
{
    let json_value = serde_json::to_value(value)
        .map_err(|e| PlerionError::ParseError(e.to_string()))?;

    if let Some(q) = query {
        let filtered = apply_query(json_value.clone(), q)?;
        println!("{}", serde_json::to_string_pretty(&filtered).unwrap());
        return Ok(());
    }

    match format {
        OutputFormat::Json => json::render(&json_value),
        OutputFormat::Yaml => yaml::render(&json_value),
        OutputFormat::Table => table::render(value, no_color),
        OutputFormat::Text => text::render(value),
    }
    Ok(())
}

/// Render a list of serializable items.
pub fn render_list<T>(items: &[T], format: OutputFormat, query: Option<&str>, no_color: bool) -> Result<(), PlerionError>
where
    T: Serialize + TableRenderable,
{
    let json_value = serde_json::to_value(items)
        .map_err(|e| PlerionError::ParseError(e.to_string()))?;

    if let Some(q) = query {
        let filtered = apply_query(json_value.clone(), q)?;
        println!("{}", serde_json::to_string_pretty(&filtered).unwrap());
        return Ok(());
    }

    match format {
        OutputFormat::Json => json::render(&json_value),
        OutputFormat::Yaml => yaml::render(&json_value),
        OutputFormat::Table => table::render_list(items, no_color),
        OutputFormat::Text => text::render_list(items),
    }
    Ok(())
}

/// Render arbitrary JSON (for endpoints that return complex/nested structures).
pub fn render_json_value(value: &serde_json::Value, format: OutputFormat, query: Option<&str>) -> Result<(), PlerionError> {
    let value = if let Some(q) = query {
        apply_query(value.clone(), q)?
    } else {
        value.clone()
    };

    match format {
        OutputFormat::Json | OutputFormat::Table | OutputFormat::Text => json::render(&value),
        OutputFormat::Yaml => yaml::render(&value),
    }
    Ok(())
}
