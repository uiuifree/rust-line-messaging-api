use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum LineWebhookSource {
    User(LineWebhookSourceUser),
    Group(LineWebhookSourceGroup),
    Room(LineWebhookSourceRoom),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LineWebhookSourceUser {
    #[serde(rename = "type")]
    pub source_type: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LineWebhookSourceGroup {
    #[serde(rename = "type")]
    pub source_type: String,
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LineWebhookSourceRoom {
    #[serde(rename = "type")]
    pub source_type: String,
    #[serde(rename = "roomId")]
    pub room_id: String,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
}
