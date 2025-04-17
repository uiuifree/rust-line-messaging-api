use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct LineWebhookEventPostback {
    #[serde(default)]
    #[serde(rename = "data")]
    pub data: String,
    #[serde(default)]
    pub params: LineWebhookEventPostbackParams,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum LineWebhookEventPostbackParams {
    Date(LineWebhookEventPostbackParamsDatetime),
    Richmenu(LineWebhookEventPostbackParamsNewRichmenu),
    #[default]
    None,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum LineWebhookEventPostbackParamsDatetime {
    #[serde(rename = "date")]
    Date(String),
    #[serde(rename = "time")]
    Time(String),
    #[serde(rename = "datetime")]
    Datetime(String),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventPostbackParamsNewRichmenu {
    pub status: String,
    #[serde(rename = "newRichMenuAliasId")]
    pub new_rich_menu_alias_id: Option<String>,
}
