pub fn render(value: &serde_json::Value) {
    match serde_yaml::to_string(value) {
        Ok(s) => print!("{s}"),
        Err(e) => eprintln!("YAML serialization error: {e}"),
    }
}
