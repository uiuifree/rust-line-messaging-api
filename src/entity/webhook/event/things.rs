use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventThings {
    #[serde(rename = "type")]
    things_type: String,
    #[serde(rename = "deviceId")]
    device_id: String,
    result: Option<LineWebhookEventThingsResult>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventThingsResult {
    #[serde(rename = "scenarioId")]
    scenario_id: String,
    #[serde(rename = "revision")]
    revision: u64,
    #[serde(rename = "startTime")]
    start_time: u64,
    #[serde(rename = "endTime")]
    end_time: u64,
    #[serde(rename = "actionResults")]
    action_results: Option<Vec<LineWebhookEventThingsResultActionResult>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LineWebhookEventThingsResultActionResult {
    #[serde(rename = "type")]
    action_type: String,
    data: Option<String>,
    #[serde(rename = "bleNotificationPayload")]
    ble_notification_payload: Option<String>,
    #[serde(rename = "errorReason")]
    error_reason: Option<String>,
}
