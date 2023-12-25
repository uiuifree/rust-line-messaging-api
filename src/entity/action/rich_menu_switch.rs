use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineMessageActionRichMenuSwitch {
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "label")]
    label: Option<String>,
    #[serde(rename = "richMenuAliasId")]
    rich_menu_alias_id: String,
    data: String,
}
