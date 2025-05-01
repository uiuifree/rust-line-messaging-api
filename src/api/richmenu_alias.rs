use crate::{LineApiResponse, LineClient};
use serde_derive::{Deserialize, Serialize};
use serde_json::{Value, json};

impl LineClient {
    // https://developers.line.biz/ja/reference/messaging-api/#create-rich-menu-alias
    pub async fn rich_menu_alias_create(&self, value: Value) -> LineApiResponse<Value> {
        self.http_post("https://api.line.me/v2/bot/richmenu/alias", &value)
            .await
    }
    pub async fn rich_menu_alias_delete(
        &self,
        rich_menu_alias_id: String,
    ) -> LineApiResponse<Value> {
        self.http_delete(
            &format!(
                "https://api.line.me/v2/bot/richmenu/alias/{}",
                rich_menu_alias_id
            ),
            &json!(""),
        )
        .await
    }
    pub async fn rich_menu_alias_update(
        &self,
        rich_menu_alias_id: String,
        value: Value,
    ) -> LineApiResponse<Value> {
        self.http_put(
            &format!(
                "https://api.line.me/v2/bot/richmenu/alias/{}",
                rich_menu_alias_id
            ),
            &value,
        )
        .await
    }

    // https://developers.line.biz/ja/reference/messaging-api/#get-rich-menu-alias-by-id
    pub async fn rich_menu_alias_get(
        &self,
        rich_menu_alias_id: String,
    ) -> LineApiResponse<LineApiRichMenuAliasGetResponse> {
        self.http_get(
            &format!(
                "https://api.line.me/v2/bot/richmenu/alias/{}",
                rich_menu_alias_id
            ),
            &json!(""),
        )
        .await
    }
    // https://developers.line.biz/ja/reference/messaging-api/#get-rich-menu-alias-list
    pub async fn rich_menu_alias_list(&self) -> LineApiResponse<LineApiRichMenuAliasGetResponse> {
        self.http_get("https://api.line.me/v2/bot/richmenu/alias/list", &json!(""))
            .await
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiRichMenuAliasCreateResponse {}
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiRichMenuAliasGetResponse {
    #[serde(rename = "richMenuId")]
    pub rich_menu_id: String,
    #[serde(rename = "richMenuAliasId")]
    pub rich_menu_alias_id: String,
}
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiRichMenuAliasListResponse {
    pub aliases: Vec<LineApiRichMenuAliasGetResponse>,
}
