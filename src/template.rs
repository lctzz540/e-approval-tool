use serde_derive::{Deserialize, Serialize};
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
    #[serde(rename = "dataFrom")]
    pub datafrom: Option<i64>,
    pub display: Option<bool>,
    pub conditions: Option<Conditions>,
    #[serde(rename = "additionalDisplayClass")]
    pub additional_display_class: Option<String>,
    pub static_content: Option<String>,
    pub valuefeeder: Option<String>,
}
fn default_min_length() -> i64 {
    1
}
fn default_max_length() -> i64 {
    1000
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conditions {
    pub required: Option<bool>,
    #[serde(default)]
    pub data: Vec<Daum>,
    #[serde(rename = "minLength")]
    #[serde(default = "default_min_length")]
    pub min_length: i64,
    #[serde(rename = "maxLength")]
    #[serde(default = "default_max_length")]
    pub max_length: i64,
    pub multiple: Option<bool>,
    pub location_related: Option<bool>,
    #[serde(rename = "displayCondition")]
    pub display_condition: Option<DisplayCondition>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    pub text: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayCondition {
    #[serde(rename = "idCondition")]
    pub id_condition: Option<i64>,
    #[serde(rename = "valueCondition")]
    pub value_condition: Option<String>,
    #[serde(rename = "dataCondition")]
    pub data_condition: Option<String>,
}

pub fn create_individual(
    name: &str,
    id: i64,
    datatype: &str,
    datafrom: i64,
    is_required: bool,
    _data: Vec<Daum>,
    position: i64,
    displaycondition: &str,
    displayconditionid: i64,
    staticcontent: &str,
) -> Individual {
    let mut individual = Individual::default();

    individual.name = name.to_string();
    individual.type_field = datatype.to_string();
    individual.value = Some("".to_string());
    individual.display = Some(true);
    individual.additional_display_class = Some("left-info".to_string());
    individual.id = id;
    individual.position = position;
    individual.static_content = Some(staticcontent.to_string());
    if datafrom != 0 {
        individual.datafrom = Some(datafrom);
    }

    let data = _data;

    let display_condition_object = DisplayCondition {
        id_condition: Some(displayconditionid),
        value_condition: Some(displaycondition.to_string()),
        data_condition: Some("INDIVIDUAL".to_string()),
    };
    if displayconditionid == 0 {
        individual.conditions = Some(Conditions {
            required: Some(is_required),
            min_length: 1,
            max_length: 1000,
            data,
            ..Conditions::default()
        });
    } else {
        individual.conditions = Some(Conditions {
            required: Some(is_required),
            min_length: 1,
            max_length: 1000,
            data,
            display_condition: Some(display_condition_object),
            ..Conditions::default()
        });
    }

    individual
}

pub fn make_splitter(name: &str, id: i64, datatype: &str) -> Individual {
    let mut individual = Individual::default();

    individual.name = name.to_string();
    individual.type_field = datatype.to_string();
    individual.id = id;
    individual.position = id;
    individual
}

pub fn read_json_file_to_template(filename: &str) -> Option<Template> {
    let file = File::open(filename).ok()?;
    let mut reader = BufReader::new(file);

    let mut json_string = String::new();
    reader.read_to_string(&mut json_string).ok()?;

    let root: Template = match serde_json::from_str(&json_string) {
        Ok(root) => root,
        Err(err) => {
            eprintln!("Error parsing JSON file: {}", err);
            return None;
        }
    };

    Some(root)
}
