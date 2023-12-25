use crate::{LineApiResponse, LineClient};
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};

impl LineClient {
    /// https://developers.line.biz/ja/reference/messaging-api/#get-number-of-delivery-messages
    pub async fn insight_message_delivery(
        &self,
        date: &str,
    ) -> LineApiResponse<LineApiInsightMessageDeliveryResponse> {
        self.http_get(
            format!(
                "https://api.line.me/v2/bot/insight/message/delivery?date={}",
                date
            )
            .as_str(),
            &json!({}),
        )
        .await
    }
    /// https://developers.line.biz/ja/reference/messaging-api/#get-number-of-followers
    pub async fn insight_followers(
        &self,
        date: &str,
    ) -> LineApiResponse<LineApiInsightFollowersResponse> {
        self.http_get(
            format!("https://api.line.me/v2/bot/insight/followers?date={}", date).as_str(),
            &json!({}),
        )
        .await
    }
    /// https://developers.line.biz/ja/reference/messaging-api/#get-demographic
    pub async fn insight_demographic(&self) -> LineApiResponse<LineApiInsightDemographicResponse> {
        self.http_get("https://api.line.me/v2/bot/insight/demographic", &json!({}))
            .await
    }
    /// https://developers.line.biz/ja/reference/messaging-api/#get-demographic
    pub async fn insight_message_event(&self, request_id: &str) -> LineApiResponse<Value> {
        self.http_get(
            format!(
                "https://api.line.me/v2/bot/insight/message/event?requestId={}",
                request_id
            )
            .as_str(),
            &json!({}),
        )
        .await
    }
    /// https://developers.line.biz/ja/reference/messaging-api/#get-statistics-per-unit
    pub async fn insight_message_event_aggregation(
        &self,
        custom_aggregation_unit: &str,
        from: &str,
        to: &str,
    ) -> LineApiResponse<Value> {
        self.http_get(format!("https://api.line.me/v2/bot/insight/message/event/aggregation?customAggregationUnit={}&from={}&to={}", custom_aggregation_unit, from, to).as_str(), &json!({})).await
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiInsightMessageDeliveryResponse {
    pub status: String,
    pub broadcast: Option<u32>,
    pub targeting: Option<u32>,
    #[serde(rename = "autoResponse")]
    pub auto_response: Option<u32>,
    #[serde(rename = "welcomeResponse")]
    pub welcome_response: Option<u32>,
    #[serde(rename = "chat")]
    pub chat: Option<u32>,
    #[serde(rename = "apiBroadcast")]
    pub api_broadcast: Option<u32>,
    #[serde(rename = "apiPush")]
    pub api_push: Option<u32>,
    #[serde(rename = "apiMulticast")]
    pub api_multicast: Option<u32>,
    #[serde(rename = "apiNarrowcast")]
    pub api_narrowcast: Option<u32>,
    #[serde(rename = "apiReply")]
    pub api_reply: Option<u32>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiInsightFollowersResponse {
    pub status: String,
    pub followers: Option<u32>,
    #[serde(rename = "targetedReaches")]
    pub targeted_reaches: Option<u32>,
    pub blocks: Option<u32>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiInsightDemographicResponse {
    pub available: bool,
    pub genders: Vec<LineApiInsightDemographicGender>,
    pub ages: Vec<LineApiInsightDemographicAge>,
    pub areas: Vec<LineApiInsightDemographicArea>,
    #[serde(rename = "appTypes")]
    pub app_types: Vec<LineApiInsightDemographicAppType>,
    #[serde(rename = "subscriptionPeriods")]
    pub subscription_periods: Vec<LineApiInsightDemographicSubscriptionPeriods>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiInsightDemographicGender {
    gender: String,
    percentage: f32,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiInsightDemographicAge {
    age: String,
    percentage: f32,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiInsightDemographicArea {
    area: String,
    percentage: f32,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiInsightDemographicAppType {
    #[serde(rename = "appType")]
    app_type: String,
    percentage: f32,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiInsightDemographicSubscriptionPeriods {
    #[serde(rename = "subscriptionPeriod")]
    subscription_period: String,
    percentage: f32,
}
