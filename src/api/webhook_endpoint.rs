use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::LineClient;
use crate::client::Host;
use crate::error::Result;

impl LineClient {
    /// Gets the webhook endpoint URL and whether webhooks are enabled.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-webhook-endpoint-information>
    pub async fn get_webhook_endpoint(&self) -> Result<GetWebhookEndpointResponse> {
        let rb = self.request(
            Method::GET,
            Host::Api,
            &["v2", "bot", "channel", "webhook", "endpoint"],
        );
        self.send_json(rb).await
    }

    /// Sets the webhook endpoint URL.
    /// <https://developers.line.biz/en/reference/messaging-api/#set-webhook-endpoint-url>
    pub async fn set_webhook_endpoint(&self, request: &SetWebhookEndpointRequest) -> Result<()> {
        let rb = self
            .request(
                Method::PUT,
                Host::Api,
                &["v2", "bot", "channel", "webhook", "endpoint"],
            )
            .json(request);
        self.send_empty(rb).await
    }

    /// Tests a webhook endpoint and returns the result (success, status code, reason).
    /// <https://developers.line.biz/en/reference/messaging-api/#test-webhook-endpoint>
    pub async fn test_webhook_endpoint(
        &self,
        request: &TestWebhookEndpointRequest,
    ) -> Result<TestWebhookEndpointResponse> {
        let rb = self
            .request(
                Method::POST,
                Host::Api,
                &["v2", "bot", "channel", "webhook", "test"],
            )
            .json(request);
        self.send_json(rb).await
    }
}

/// Webhook endpoint information.
///
/// <https://developers.line.biz/en/reference/messaging-api/#get-webhook-endpoint-information>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWebhookEndpointResponse {
    /// Webhook URL.
    pub endpoint: String,
    /// Webhook usage status. Webhook events are sent to the webhook URL only when `true`.
    pub active: bool,
}

/// Request body of [`LineClient::set_webhook_endpoint`](crate::LineClient::set_webhook_endpoint).
///
/// <https://developers.line.biz/en/reference/messaging-api/#set-webhook-endpoint-url>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetWebhookEndpointRequest {
    /// A valid webhook URL (max 500 characters).
    pub endpoint: String,
}

impl SetWebhookEndpointRequest {
    /// Creates a request that sets the webhook URL to `endpoint`.
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
        }
    }
}

/// Request body of [`LineClient::test_webhook_endpoint`](crate::LineClient::test_webhook_endpoint).
///
/// <https://developers.line.biz/en/reference/messaging-api/#test-webhook-endpoint>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestWebhookEndpointRequest {
    /// A webhook URL to be validated (max 500 characters).
    pub endpoint: Option<String>,
}

impl TestWebhookEndpointRequest {
    /// Sets the webhook URL to be validated.
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
}

/// Result of a webhook endpoint test.
///
/// <https://developers.line.biz/en/reference/messaging-api/#test-webhook-endpoint>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestWebhookEndpointResponse {
    /// Result of the communication from the LINE platform to the webhook URL.
    pub success: Option<bool>,
    /// Time of the event.
    pub timestamp: String,
    /// The HTTP status code. Zero or a negative number if the webhook response isn't received.
    pub status_code: i32,
    /// Reason for the response.
    pub reason: String,
    /// Details of the response.
    pub detail: String,
}
