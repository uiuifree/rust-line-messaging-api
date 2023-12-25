use crate::{LineApiResponse, LineClient};
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};

impl LineClient {
    // POST   /v2/bot/user/{userId}/richmenu/{richMenuId}
    /// https://developers.line.biz/ja/reference/messaging-api/#create-rich-menu
    pub async fn rich_menu_user_link_menu(
        &self,
        rich_menu_id: &str,
        user_id: &str,
    ) -> LineApiResponse<LineApiRichMenuUserEmptyResponse> {
        self.http_post(
            format!(
                "https://api.line.me/v2/bot/user/{}/richmenu/{}",
                user_id, rich_menu_id
            )
            .as_str(),
            &json!({}),
        )
        .await
    }
    /// https://developers.line.biz/ja/reference/messaging-api/#link-rich-menu-to-users
    // POST   /v2/bot/richmenu/bulk/link
    pub async fn rich_menu_user_link_rich_menu_bulk(
        &self,
        rich_menu_id: &str,
        user_id: Vec<String>,
    ) -> LineApiResponse<LineApiRichMenuUserEmptyResponse> {
        self.http_post(
            "https://api.line.me/v2/bot/richmenu/bulk/link",
            &json!({
                "richMenuId": rich_menu_id,
                "userIds": user_id,
            }),
        )
        .await
    }
    // GET    /v2/bot/user/{userId}/richmenu

    /// https://developers.line.biz/ja/reference/messaging-api/#create-rich-menu
    pub async fn rich_menu_user_get_user_rich_menu_id(
        &self,
        user_id: &str,
    ) -> LineApiResponse<LineApiRichMenuUserGetUserRichMenuId> {
        self.http_get(
            format!("https://api.line.me/v2/bot/user/{}/richmenu", user_id).as_str(),
            &json!({}),
        )
        .await
    }
    // DELETE /v2/bot/user/{userId}/richmenu
    pub async fn rich_menu_user_unlink_user_rich_menu(
        &self,
        user_id: &str,
    ) -> LineApiResponse<LineApiRichMenuUserEmptyResponse> {
        self.http_delete(
            format!("https://api.line.me/v2/bot/user/{}/richmenu", user_id).as_str(),
            &json!({}),
        )
        .await
    }
    // POST   /v2/bot/richmenu/bulk/unlink
    pub async fn rich_menu_user_unlink_menu_bulk(
        &self,
        user_id: Vec<String>,
    ) -> LineApiResponse<LineApiRichMenuUserEmptyResponse> {
        self.http_post(
            "https://api.line.me/v2/bot/richmenu/bulk/unlink",
            &json!({
                "userIds": user_id,
            }),
        )
        .await
    }
    // POST   /v2/bot/richmenu/batch
    // GET    /v2/bot/richmenu/progress/batch
    // POST   /v2/bot/richmenu/validate/batch
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiRichMenuUserEmptyResponse {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiRichMenuUserGetUserRichMenuId {
    #[serde(rename = "richMenuId")]
    pub rich_menu_id: String,
}
