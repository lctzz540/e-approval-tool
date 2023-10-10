use crate::{template, workflow};
use polars::prelude::AnyValue;
use std::fs::File;
use std::io::Write;

pub fn to_i64(value: AnyValue) -> i64 {
    match value {
        AnyValue::Int64(i) => i,
        AnyValue::Float64(f) => f as i64,
        _ => 0,
    }
}

pub fn clean_value(value: AnyValue) -> String {
    match value {
        AnyValue::Int64(i) => i.to_string(),
        AnyValue::Float64(f) => f.to_string(),
        AnyValue::Utf8(s) => {
            let cleaned_value = s
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
                .to_lowercase();
            cleaned_value
        }
        _ => String::new(),
    }
}

pub fn remove_lines_with_null(json_string: &str) -> String {
    let lines: Vec<&str> = json_string
        .lines()
        .filter(|line| !line.contains("null"))
        .collect();
    lines.join("\n")
}

pub fn template_to_json(template: &template::Template) -> Result<String, serde_json::Error> {
    let json_string = serde_json::to_string_pretty(template)?;

    Ok(remove_lines_with_null(json_string.as_str()))
}

pub fn write_json_to_file(filename: &str, json_string: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}

pub fn workflow_to_json(workflow: &workflow::Root) -> Result<String, serde_json::Error> {
    let json_string = serde_json::to_string_pretty(workflow)?;

    Ok(remove_lines_with_null(json_string.as_str()))
}
