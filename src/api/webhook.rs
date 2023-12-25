use crate::{LineApiResponse, LineClient};
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};

impl LineClient {
    /// https://developers.line.biz/ja/reference/messaging-api/#set-webhook-endpoint-url
    pub async fn webhook_endpoint_put(&self, endpoint: &str) -> LineApiResponse<Value> {
        self.http_put(
            "https://api.line.me/v2/bot/channel/webhook/endpoint",
            &json!({
                "endpoint": endpoint
            }),
        )
        .await
    }
    /// https://developers.line.biz/ja/reference/messaging-api/#get-webhook-endpoint-information
    pub async fn webhook_endpoint_get(&self) -> LineApiResponse<LineApiWebhookEndpointGetResponse> {
        self.http_get(
            "https://api.line.me/v2/bot/channel/webhook/endpoint",
            &json!({}),
        )
        .await
    }
    /// https://developers.line.biz/ja/reference/messaging-api/#get-webhook-endpoint-information
    pub async fn webhook_test(
        &self,
        request: &LineApiWebhookTestRequest,
    ) -> LineApiResponse<LineApiWebhookTestResponse> {
        self.http_post("https://api.line.me/v2/bot/channel/webhook/test", request)
            .await
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LineApiWebhookEndpointGetResponse {
    pub endpoint: String,
    pub active: bool,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LineApiWebhookTestRequest {
    pub endpoint: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LineApiWebhookTestResponse {
    pub success: bool,
    pub timestamp: String,
    #[serde(rename = "statusCode")]
    pub status_code: u32,
    pub reason: String,
    pub detail: String,
}
