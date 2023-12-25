use crate::{LineApiResponse, LineClient};
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};

impl LineClient {
    /// https://developers.line.biz/ja/reference/messaging-api/#create-rich-menu
    pub async fn rich_menu_create(
        &self,
        value: Value,
    ) -> LineApiResponse<LineApiRichMenuCreateResponse> {
        self.http_post("https://api.line.me/v2/bot/richmenu", &value)
            .await
    }
    /// https://developers.line.biz/ja/reference/messaging-api/#create-rich-menu
    pub async fn rich_menu_validate_object(
        &self,
        value: Value,
    ) -> LineApiResponse<LineApiRichMenuValidateObjectResponse> {
        self.http_post("https://api.line.me/v2/bot/richmenu/validate", &value)
            .await
    }
    /// https://developers.line.biz/ja/reference/messaging-api/#delete-rich-menu
    pub async fn rich_menu_content_upload(
        &self,
        rich_menu_id: &str,
        file: Vec<u8>,
    ) -> LineApiResponse<Value> {
        self.http_post_data(
            format!(
                "https://api-data.line.me/v2/bot/richmenu/{}/content",
                rich_menu_id
            )
            .as_str(),
            file,
        )
        .await
    }
    /// https://developers.line.biz/ja/reference/messaging-api/#download-rich-menu-image
    pub async fn rich_menu_content_download(&self, rich_menu_id: &str) -> LineApiResponse<Vec<u8>> {
        self.http_get_stream(
            format!(
                "https://api-data.line.me/v2/bot/richmenu/{}/content",
                rich_menu_id
            )
            .as_str(),
            &json!({}),
        )
        .await
    }
    /// https://developers.line.biz/ja/reference/messaging-api/#get-rich-menu-list
    pub async fn rich_menu_list(&self) -> LineApiResponse<LineApiRichMenuListResponse> {
        self.http_get("https://api.line.me/v2/bot/richmenu/list", &json!({}))
            .await
    }
    /// https://developers.line.biz/ja/reference/messaging-api/#delete-rich-menu
    pub async fn rich_menu_delete(
        &self,
        rich_menu_id: &str,
    ) -> LineApiResponse<LineApiRichMenuDeleteResponse> {
        self.http_delete(
            format!("https://api.line.me/v2/bot/richmenu/{}", rich_menu_id).as_str(),
            &json!({}),
        )
        .await
    }
    /// https://developers.line.biz/ja/reference/messaging-api/#delete-rich-menu
    pub async fn rich_menu_get(
        &self,
        rich_menu_id: &str,
    ) -> LineApiResponse<LineApiRichMenuGetResponse> {
        self.http_get(
            format!("https://api.line.me/v2/bot/richmenu/{}", rich_menu_id).as_str(),
            &json!({}),
        )
        .await
    }
    /// https://developers.line.biz/ja/reference/messaging-api/#get-default-rich-menu-id
    pub async fn rich_menu_set_default_menu(
        &self,
        rich_menu_id: &str,
    ) -> LineApiResponse<LineApiRichMenuSetDefaultResponse> {
        self.http_post(
            format!(
                "https://api.line.me/v2/bot/user/all/richmenu/{}",
                rich_menu_id
            )
            .as_str(),
            &json!({}),
        )
        .await
    }

    /// https://developers.line.biz/ja/reference/messaging-api/#get-default-rich-menu-id
    pub async fn rich_menu_get_default_menu_id(
        &self,
    ) -> LineApiResponse<LineApiRichMenuGetDefaultResponse> {
        self.http_get("https://api.line.me/v2/bot/user/all/richmenu", &json!({}))
            .await
    }
    /// https://developers.line.biz/ja/reference/messaging-api/#cancel-default-rich-menu
    pub async fn rich_menu_delete_default_menu(
        &self,
    ) -> LineApiResponse<LineApiRichMenuDeleteDefaultResponse> {
        self.http_delete("https://api.line.me/v2/bot/user/all/richmenu", &json!({}))
            .await
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiRichMenuCreateResponse {
    #[serde(rename = "richMenuId")]
    pub rich_menu_id: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiRichMenuValidateObjectResponse {}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SlackApiFilesUploadRequest {
    pub file: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiRichMenuListResponse {
    #[serde(rename = "richmenus")]
    pub rich_menus: Vec<RichMenu>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct RichMenu {
    #[serde(rename = "richMenuId")]
    pub rich_menu_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "size")]
    pub size: RichMenuSize,
    #[serde(rename = "chatBarText")]
    pub chat_bar_text: String,
    #[serde(rename = "selected")]
    pub selected: bool,
    #[serde(rename = "areas")]
    pub areas: Vec<RichMenuArea>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct RichMenuSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct RichMenuArea {
    pub bounds: RichMenuAreaBounds,
    pub action: RichMenuAction,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct RichMenuAreaBounds {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct RichMenuAction {
    #[serde(rename = "type")]
    pub action_type: String,
    pub data: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiRichMenuGetResponse {
    #[serde(flatten)]
    pub rich_menu: RichMenu,
}
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiRichMenuDeleteResponse {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiRichMenuSetDefaultResponse {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiRichMenuGetDefaultResponse {
    #[serde(rename = "richMenuId")]
    pub rich_menu_id: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiRichMenuDeleteDefaultResponse {}
