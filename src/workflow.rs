use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub relatives: Vec<Rela>,
    pub relationships: Vec<Relationship>,
    #[serde(rename = "notification_config")]
    pub notification_config: NotificationConfig,
    pub display: Vec<Display>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rela {
    pub owner: Option<String>,
    pub decisions: Option<Decisions>,
    pub id: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub is_done: bool,
    #[serde(rename = "role_config")]
    pub role_config: Option<RoleConfig>,
    pub status: String,
    pub summary: Option<String>,
    pub roles: Option<Roles>,
    pub autocreate: Option<bool>,
    pub sla: Option<Sla>,
    pub phase_type: Option<String>,
    pub is_notify: Option<bool>,
    pub auto_finish: Option<bool>,
    pub prevent_feeder: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Decisions {
    pub text: String,
    pub actions: Vec<Action>,
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub workflow: Workflow,
    pub name: String,
    pub id: i64,
    pub conditions: Conditions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workflow {
    pub relationships: Vec<i64>,
    pub relatives: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conditions {
    #[serde(rename = "right-element")]
    pub right_element: Option<RightElement>,
    #[serde(rename = "compare-type")]
    pub compare_type: Option<String>,
    pub operation: String,
    #[serde(rename = "left-element")]
    pub left_element: Option<LeftElement>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RightElement {
    pub dataset: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeftElement {
    pub input_id: i64,
    pub info_id: i64,
    pub phase_id: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub dataset: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleConfig {
    #[serde(default)]
    pub assigner: Vec<Value>,
    #[serde(default)]
    pub implementer: Vec<Value>,
    #[serde(default)]
    pub functions: Vec<Function>,
    #[serde(rename = "provider_id")]
    #[serde(default)]
    pub provider_id: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Function {
    #[serde(rename = "role_type_to_add")]
    pub role_type_to_add: String,
    pub id: i64,
    pub params: Vec<Param>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Param {
    pub info_id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub dataset: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Roles {
    pub assigner: Vec<Value>,
    pub implementer: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sla {
    pub res: f64,
    pub fix: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    pub autocreate: bool,
    pub from: i64,
    pub id: i64,
    pub to: i64,
    pub position: Option<i64>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub status: String,
    pub display: Option<bool>,
    pub button_config: Option<ButtonConfig>,
    pub detail: Option<Detail>,
    pub related_to: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonConfig {
    pub cancel: Cancel,
    pub approve: Approve,
    pub request_update: RequestUpdate,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cancel {
    pub enable: bool,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Approve {
    pub enable: bool,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestUpdate {
    pub enable: bool,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Detail {
    pub individual: Vec<Individual>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Individual {
    pub static_content: String,
    pub display: bool,
    pub name: String,
    pub additional_display_class: String,
    pub id: i64,
    pub position: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub conditions: Conditions2,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conditions2 {
    #[serde(rename = "max-length")]
    pub max_length: i64,
    pub required: bool,
    #[serde(rename = "min-length")]
    pub min_length: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationConfig {
    #[serde(rename = "approvers_pending_mail_template")]
    pub approvers_pending_mail_template: i64,
    #[serde(rename = "ticket_response_mail_template")]
    pub ticket_response_mail_template: i64,
    #[serde(rename = "comment_mail_template")]
    pub comment_mail_template: i64,
    #[serde(rename = "viewers_assignment_mail_template")]
    pub viewers_assignment_mail_template: i64,
    #[serde(rename = "watchers_ticket_rejection_mail_template")]
    pub watchers_ticket_rejection_mail_template: i64,
    #[serde(rename = "ticket_result_reject_mail_template")]
    pub ticket_result_reject_mail_template: i64,
    #[serde(rename = "new_performer_phase_reassignment_mail_template")]
    pub new_performer_phase_reassignment_mail_template: i64,
    #[serde(rename = "creator_rejected_mail_template")]
    pub creator_rejected_mail_template: i64,
    #[serde(rename = "ticket_share_mail_template")]
    pub ticket_share_mail_template: i64,
    #[serde(rename = "ticket_creation_mail_template")]
    pub ticket_creation_mail_template: i64,
    #[serde(rename = "watchers_ticket_updated_mail_template")]
    pub watchers_ticket_updated_mail_template: i64,
    #[serde(rename = "ticket_finish_inform_mail_template")]
    pub ticket_finish_inform_mail_template: i64,
    #[serde(rename = "relationship_result_reject_mail_template")]
    pub relationship_result_reject_mail_template: i64,
    #[serde(rename = "creator_approved_mail_template")]
    pub creator_approved_mail_template: i64,
    #[serde(rename = "mention_mail_template")]
    pub mention_mail_template: i64,
    #[serde(rename = "co_execute_rejection_mail_template")]
    pub co_execute_rejection_mail_template: i64,
    #[serde(rename = "implementation_reminder_mail_template")]
    pub implementation_reminder_mail_template: i64,
    #[serde(rename = "ticket_cancellation_mail_template")]
    pub ticket_cancellation_mail_template: i64,
    #[serde(rename = "creator_requested_update_mail_template")]
    pub creator_requested_update_mail_template: i64,
    #[serde(rename = "approval_input_finish_mail_template")]
    pub approval_input_finish_mail_template: i64,
    #[serde(rename = "co_execute_request_to_update_mail_template")]
    pub co_execute_request_to_update_mail_template: i64,
    #[serde(rename = "old_performer_phase_reassignment_mail_template")]
    pub old_performer_phase_reassignment_mail_template: i64,
    #[serde(rename = "watchers_ticket_auto_closing_mail_template")]
    pub watchers_ticket_auto_closing_mail_template: i64,
    #[serde(rename = "approval_reminder_no_token_mail_template")]
    pub approval_reminder_no_token_mail_template: i64,
    #[serde(rename = "watchers_ticket_creation_mail_template")]
    pub watchers_ticket_creation_mail_template: i64,
    #[serde(rename = "watchers_ticket_cancellation_mail_template")]
    pub watchers_ticket_cancellation_mail_template: i64,
    #[serde(rename = "creator_requested_update_reminder_mail_template")]
    pub creator_requested_update_reminder_mail_template: i64,
    #[serde(rename = "ticket_updated_mail_template")]
    pub ticket_updated_mail_template: i64,
    #[serde(rename = "approval_reminder_mail_template")]
    pub approval_reminder_mail_template: i64,
    #[serde(rename = "confirmation_reminder_mail_template")]
    pub confirmation_reminder_mail_template: i64,
    #[serde(rename = "phase_input_finish_mail_template")]
    pub phase_input_finish_mail_template: i64,
    #[serde(rename = "phase_finished_mail_template")]
    pub phase_finished_mail_template: i64,
    #[serde(rename = "watchers_ticket_closing_mail_template")]
    pub watchers_ticket_closing_mail_template: i64,
    #[serde(rename = "watchers_ticket_response_mail_template")]
    pub watchers_ticket_response_mail_template: i64,
    #[serde(rename = "phase_approve_finished_mail_template")]
    pub phase_approve_finished_mail_template: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Display {
    pub positions: Vec<i64>,
    pub id: i64,
    pub image: Option<String>,
}

pub fn read_json_file_to_notification_config(
    filename: &str,
) -> Result<NotificationConfig, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut json_string = String::new();
    reader.read_to_string(&mut json_string)?;

    let parsed_json: serde_json::Value = serde_json::from_str(&json_string)?;

    let root: NotificationConfig = serde_json::from_value(parsed_json)?;

    Ok(root)
}
