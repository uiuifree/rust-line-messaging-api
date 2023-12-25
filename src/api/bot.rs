use crate::{LineApiResponse, LineClient};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

impl LineClient {
    /// https://developers.line.biz/ja/reference/messaging-api/#get-bot-info
    pub async fn bot_info(&self) -> LineApiResponse<LineApiBotInfoResponse> {
        self.http_get("https://api.line.me/v2/bot/info", &json!({}))
            .await
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LineApiBotInfoResponse {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "basicId")]
    pub basic_id: String,
    #[serde(rename = "premiumId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_id: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "pictureUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_url: Option<String>,
    #[serde(rename = "chatMode")]
    pub chat_mode: String,
    #[serde(rename = "markAsReadMode")]
    pub mark_as_read_mode: String,
}
