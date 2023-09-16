use crate::template;
use polars::prelude::*;
use std::error::Error;

fn to_i64(value: AnyValue) -> i64 {
    match value {
        AnyValue::Int64(i) => i,
        AnyValue::Float64(f) => f as i64,
        _ => 0,
    }
}

fn clean_value(value: AnyValue) -> String {
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
pub fn process_request(file_path: &str) -> Result<(), Box<dyn Error>> {
    let df = CsvReader::from_path(file_path)?
        .finish()
        .expect("Error reading CSV file");

    let mut _id: i64 = 0;
    let mut template_out = match template::read_json_file("template.json") {
        Some(template) => template,
        None => template::Template::default(),
    };

    for i in 0..df.height() {
        let mut _datatype = String::new();
        let mut _name = String::new();
        let mut _is_masterdata = false;
        let mut _is_required = false;
        let mut _datalist: Vec<String> = Vec::new();
        let mut _is_splitter = false;
        let mut _display_condition = String::new();
        let mut _static_content = String::new();
        let mut _display_condition_id: i64 = 0;
        let mut _data_from: i64 = 0;

        for col_name in df.get_column_names() {
            if col_name == "NO" {
                let value = clean_value(df.column(col_name).unwrap().get(i).unwrap());
                if !value.parse::<i32>().is_ok() {
                    _is_splitter = true;
                }
            }
            if col_name == "TÊN TRƯỜNG" {
                let value = df.column(col_name).unwrap().get(i).unwrap().to_string();
                _name = value.trim_matches(|c| c == '"' || c == '\'').to_string();
            }

            if col_name == "LOẠI DỮ LIỆU" {
                _datatype = clean_value(df.column(col_name).unwrap().get(i).unwrap());

                if _datatype == "masterdata" {
                    _is_masterdata = true;
                }
                if _datatype == "mặcđịnh" {
                    _datatype = "".to_string()
                }
            }
            if col_name == "MASTER DATA KEY" && _is_masterdata {
                _datatype = df.column(col_name).unwrap().get(i).unwrap().to_string();
            }
            if col_name == "Json ID" {
                _id = to_i64(df.column(col_name).unwrap().get(i).unwrap());
            }

            if col_name == "dataFrom" {
                _id = to_i64(df.column(col_name).unwrap().get(i).unwrap());
            }

            if col_name == "displayConditionID" {
                _display_condition_id = to_i64(df.column(col_name).unwrap().get(i).unwrap());
            }

            if col_name == "BĂT BUỘC" {
                let value = clean_value(df.column(col_name).unwrap().get(i).unwrap());

                if value == "x" {
                    _is_required = true;
                }
            }
            if col_name == "displayCondition" {
                let value = df.column(col_name).unwrap().get(i).unwrap().to_string();
                _display_condition = value.trim_matches(|c| c == '"' || c == '\'').to_string();
            }
            if col_name == "select" {
                let data_str = df.column(col_name).unwrap().get(i).unwrap().to_string();

                let trimmed_data_str = data_str.trim();

                if !trimmed_data_str.is_empty() && trimmed_data_str != "null" {
                    _datalist = trimmed_data_str
                        .split(',')
                        .map(|s| s.trim_matches(|c| c == '"' || c == '\'').to_string())
                        .collect();
                }
            }
            if col_name == "staticContent" {
                let data_str = df.column(col_name).unwrap().get(i).unwrap().to_string();

                let trimmed_data_str = data_str.trim();

                if !trimmed_data_str.is_empty() && trimmed_data_str != "null" {
                    _static_content = trimmed_data_str.to_string()
                }
            }
        }
        if _name == "".to_string() {
            continue;
        }
        let mut _data: Vec<template::Daum> = Vec::new();

        for (index, text) in _datalist.iter().enumerate() {
            let value = (index + 1).to_string();
            let daum_instance = template::Daum {
                text: text.clone(),
                value,
            };
            _data.push(daum_instance);
        }
        if _is_splitter {
            template_out.individual.push(template::make_splitter(
                _name.as_str(),
                _id,
                _datatype.as_str(),
            ))
        } else {
            template_out.individual.push(template::create_individual(
                _name.as_str(),
                _id,
                _datatype.as_str(),
                _data_from,
                _is_required,
                _data,
                _id,
                _display_condition.as_str(),
                _display_condition_id,
                format!(
                    r#"<p style="font-size: 12px; color: blue;"><i>{}</i></p>"#,
                    _static_content
                )
                .as_str(),
            ));
        }
    }
    match template::template_to_json(&template_out) {
        Ok(json_string) => {
            if let Err(err) = template::write_json_to_file("output.json", &json_string) {
                eprintln!("Error exporting JSON to file: {}", err);
            }
        }
        Err(err) => {
            eprintln!("Error converting to JSON: {}", err);
        }
    }
    Ok(())
}
