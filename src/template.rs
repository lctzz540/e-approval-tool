use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    pub individual: Vec<Individual>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Individual {
    pub name: String,
    pub value: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: i64,
    pub position: i64,
    pub display: Option<bool>,
    pub conditions: Option<Conditions>,
    pub additional_display_class: Option<String>,
    pub static_content: Option<String>,
    pub valuefeeder: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conditions {
    pub required: bool,
    #[serde(default)]
    pub data: Vec<Daum>,
    #[serde(rename = "min-length")]
    pub min_length: i64,
    #[serde(rename = "max-length")]
    pub max_length: i64,
    pub multiple: Option<bool>,
    pub location_related: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    pub text: String,
    pub value: String,
}

pub fn create_individual(
    name: &str,
    id: i64,
    datatype: &str,
    is_required: bool,
    _data: Vec<Daum>,
    position: i64,
) -> Individual {
    let mut individual = Individual::default();

    individual.name = name.to_string();
    individual.type_field = datatype.to_string();
    individual.value = Some("".to_string());
    individual.display = Some(true);
    individual.additional_display_class = Some("left-info".to_string());
    individual.id = id;
    individual.position = position;

    let data = _data;

    individual.conditions = Some(Conditions {
        data,
        required: is_required,
        min_length: 1,
        max_length: 1000,
        ..Conditions::default()
    });

    individual
}

fn remove_lines_with_null(json_string: &str) -> String {
    let lines: Vec<&str> = json_string
        .lines()
        .filter(|line| !line.contains("null"))
        .collect();
    lines.join("\n")
}

pub fn template_to_json(template: &Template) -> Result<String, serde_json::Error> {
    let json_string = serde_json::to_string_pretty(template)?;

    Ok(remove_lines_with_null(json_string.as_str()))
}

pub fn read_json_file(filename: &str) -> Option<Template> {
    let file = File::open(filename).ok()?;
    let mut reader = BufReader::new(file);

    let mut json_string = String::new();
    reader.read_to_string(&mut json_string).ok()?;

    let root: Template = serde_json::from_str(&json_string).ok()?;

    Some(root)
}
