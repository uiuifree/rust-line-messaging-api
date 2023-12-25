use crate::{LineApiResponse, LineClient};
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};

impl LineClient {
    /// https://developers.line.biz/ja/reference/messaging-api/#get-profile
    pub async fn profile(&self, user_id: &str) -> LineApiResponse<LineApiProfileResponse> {
        self.http_get(
            format!("https://api.line.me/v2/bot/profile/{}", user_id).as_str(),
            &json!({}),
        )
        .await
    }
    /// https://developers.line.biz/ja/reference/messaging-api/#get-follower-ids
    pub async fn followers_ids(
        &self,
        request: &LineApiFollowersIdsRequest,
    ) -> LineApiResponse<Value> {
        self.http_get("https://api.line.me/v2/bot/followers/ids", request)
            .await
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LineApiFollowersIdsRequest {
    pub limit: Option<u32>,
    pub start: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LineApiProfileResponse {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "pictureUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_url: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}
