use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};

use super::TableRenderable;

/// Severity color mapping — returns ANSI-colored string if color enabled.
pub fn colorize_severity(s: &str, no_color: bool) -> String {
    if no_color {
        return s.to_string();
    }
    match s.to_uppercase().as_str() {
        "CRITICAL" => s.to_string(),
        "HIGH" => s.to_string(),
        "MEDIUM" => s.to_string(),
        "LOW" => s.to_string(),
        "INFORMATIONAL" => s.to_string(),
        _ => s.to_string(),
    }
}

/// Status color mapping.
pub fn colorize_status(s: &str, no_color: bool) -> String {
    if no_color {
        return s.to_string();
    }
    match s.to_uppercase().as_str() {
        "FAILED" | "OPEN" | "DELETE FAILED" | "DELETE IN PROGRESS" => s.to_string(),
        "PASSED" | "RESOLVED" | "ACTIVE" => s.to_string(),
        "DISMISSED_ACCEPTED" | "DISMISSED_NOT_A_RISK" => s.to_string(),
        _ => s.to_string(),
    }
}

/// Apply color to a cell based on its text content (severity or status).
fn colorize_cell(cell: Cell, text: &str, no_color: bool) -> Cell {
    if no_color {
        return cell;
    }
    match text.to_uppercase().as_str() {
        // Severity colors
        "CRITICAL" => cell.fg(Color::Red).add_attribute(Attribute::Bold),
        "HIGH" => cell.fg(Color::Red),
        "MEDIUM" => cell.fg(Color::Yellow),
        "LOW" => cell.fg(Color::Cyan),
        "INFORMATIONAL" => cell.fg(Color::Blue),
        // Status colors
        "FAILED" | "OPEN" | "DELETE FAILED" | "DELETE IN PROGRESS" => cell.fg(Color::Red),
        "PASSED" | "RESOLVED" | "ACTIVE" => cell.fg(Color::Green),
        "DISMISSED_ACCEPTED" | "DISMISSED_NOT_A_RISK" => cell.fg(Color::Yellow),
        _ => cell,
    }
}

pub fn render<T: TableRenderable>(item: &T, no_color: bool) {
    render_list(std::slice::from_ref(item), no_color);
}

pub fn render_list<T: TableRenderable>(items: &[T], no_color: bool) {
    if items.is_empty() {
        println!("No results.");
        return;
    }

    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);

    // Header row
    let headers: Vec<Cell> = T::headers()
        .iter()
        .map(|h| {
            if no_color {
                Cell::new(h)
            } else {
                Cell::new(h).add_attribute(Attribute::Bold).fg(Color::White)
            }
        })
        .collect();
    table.set_header(headers);

    for item in items {
        let row: Vec<Cell> = item
            .row()
            .into_iter()
            .map(|text| {
                let cell = Cell::new(&text);
                colorize_cell(cell, &text, no_color)
            })
            .collect();
        table.add_row(row);
    }

    println!("{table}");
    println!("{} result(s)", items.len());
}
