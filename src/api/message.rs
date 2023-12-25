use crate::message::{
    LineApiMessageBroadcastRequest, LineApiMessageMulticastRequest, LineApiMessagePushRequest,
};
use crate::{LineApiResponse, LineClient};
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};

impl LineClient {
    //
    // エンドポイント一覧
    // POST /v2/bot/message/reply
    // POST /v2/bot/message/push
    /// https://developers.line.biz/ja/reference/messaging-api/#send-push-message
    pub async fn message_send_push(
        &self,
        request: &LineApiMessagePushRequest,
    ) -> LineApiResponse<LineApiMessagePushResponse> {
        self.http_post("https://api.line.me/v2/bot/message/push", &request)
            .await
    }
    /// https://developers.line.biz/ja/reference/messaging-api/#send-multicast-message
    // POST /v2/bot/message/multicast
    pub async fn message_send_multicast(
        &self,
        request: &LineApiMessageMulticastRequest,
    ) -> LineApiResponse<LineApiMessageSendMulticastResponse> {
        self.http_post("https://api.line.me/v2/bot/message/multicast", &request)
            .await
    }
    // POST /v2/bot/message/narrowcast
    // GET  /v2/bot/message/progress/narrowcast
    // POST /v2/bot/message/broadcast
    pub async fn message_send_broadcast(
        &self,
        request: &LineApiMessageBroadcastRequest,
    ) -> LineApiResponse<LineApiMessageSendBroadcastResponse> {
        self.http_post("https://api.line.me/v2/bot/message/broadcast", &request)
            .await
    }
    /// https://developers.line.biz/ja/reference/messaging-api/#get-quota
    // GET  /v2/bot/message/quota
    pub async fn message_get_quota(&self) -> LineApiResponse<LineApiMessageGetQuotaResponse> {
        self.http_get("https://api.line.me/v2/bot/message/quota", &json!({}))
            .await
    }
    /// https://developers.line.biz/ja/reference/messaging-api/#get-consumption
    // GET  /v2/bot/message/quota/consumption
    pub async fn message_get_quota_consumption(
        &self,
    ) -> LineApiResponse<LineApiMessageGetQuotaConsumptionResponse> {
        self.http_get(
            "https://api.line.me/v2/bot/message/quota/consumption ",
            &json!({}),
        )
        .await
    }
    // GET  /v2/bot/message/delivery/reply
    /// https://developers.line.biz/ja/reference/messaging-api/#get-number-of-reply-messages
    pub async fn message_get_number_of_reply_messages(
        &self,
        date: &str,
    ) -> LineApiResponse<LineApiMessageGetNumberOfReplyMessageResponse> {
        self.http_get(
            format!(
                "https://api.line.me/v2/bot/message/delivery/reply?date={}",
                date
            )
            .as_str(),
            &json!({}),
        )
        .await
    }

    ///
    // GET  /v2/bot/message/delivery/push
    /// https://developers.line.biz/ja/reference/messaging-api/#get-number-of-push-messages

    pub async fn message_get_number_of_push_messages(
        &self,
        date: &str,
    ) -> LineApiResponse<LineApiMessageGetNumberOfPushMessageResponse> {
        self.http_get(
            format!(
                "https://api.line.me/v2/bot/message/delivery/push?date={}",
                date
            )
            .as_str(),
            &json!({}),
        )
        .await
    }

    // GET  /v2/bot/message/delivery/multicast
    /// https://developers.line.biz/ja/reference/messaging-api/#get-number-of-push-messages
    pub async fn message_get_number_of_multicast_messages(
        &self,
        date: &str,
    ) -> LineApiResponse<LineApiMessageGetNumberOfMulticastMessageResponse> {
        self.http_get(
            format!(
                "https://api.line.me/v2/bot/message/delivery/multicast?date={}",
                date
            )
            .as_str(),
            &json!({}),
        )
        .await
    }
    // GET  /v2/bot/message/delivery/broadcast
    /// https://developers.line.biz/ja/reference/messaging-api/#get-number-of-broadcast-messages
    pub async fn message_get_number_of_broadcast_messages(
        &self,
        date: &str,
    ) -> LineApiResponse<LineApiMessageGetNumberOfBroadcastMessageResponse> {
        self.http_get(
            format!(
                "https://api.line.me/v2/bot/message/delivery/broadcast?date={}",
                date
            )
            .as_str(),
            &json!({}),
        )
        .await
    }
    // POST /v2/bot/message/validate/reply
    /// https://developers.line.biz/ja/reference/messaging-api/#validate-message-objects-of-reply-message
    pub async fn message_validate_reply(
        &self,
        data: &Vec<Value>,
    ) -> LineApiResponse<LineApiMessageValidateMessageObjectsOfReplyMessage> {
        self.http_post(
            "https://api.line.me/v2/bot/message/validate/reply",
            &json!({
                "messages":data
            }),
        )
        .await
    }

    // POST /v2/bot/message/validate/push
    /// https://developers.line.biz/ja/reference/messaging-api/#validate-message-objects-of-push-message
    pub async fn message_validate_push(
        &self,
        data: &Vec<Value>,
    ) -> LineApiResponse<LineApiMessageValidateMessageObjectsOfPushMessage> {
        self.http_post(
            "https://api.line.me/v2/bot/message/validate/push",
            &json!({
                "messages":data
            }),
        )
        .await
    }

    // POST /v2/bot/message/validate/multicast
    /// https://developers.line.biz/ja/reference/messaging-api/#validate-message-objects-of-multicast-message
    pub async fn message_validate_multicast(
        &self,
        data: &Vec<Value>,
    ) -> LineApiResponse<LineApiMessageValidateMessageObjectsOfMulticastMessage> {
        self.http_post(
            "https://api.line.me/v2/bot/message/validate/multicast",
            &json!({
                "messages":data
            }),
        )
        .await
    }

    // POST /v2/bot/message/validate/narrowcast
    /// https://developers.line.biz/ja/reference/messaging-api/#validate-message-objects-of-narrowcast-message
    pub async fn message_validate_narrowcast(
        &self,
        data: &Vec<Value>,
    ) -> LineApiResponse<LineApiMessageValidateMessageObjectsOfNarrowcastMessage> {
        self.http_post(
            "https://api.line.me/v2/bot/message/validate/narrowcast",
            &json!({
                "messages":data
            }),
        )
        .await
    }

    // POST /v2/bot/message/validate/broadcast
    /// https://developers.line.biz/ja/reference/messaging-api/#validate-message-objects-of-broadcast-message

    pub async fn message_validate_broadcast(
        &self,
        data: &Vec<Value>,
    ) -> LineApiResponse<LineApiMessageValidateMessageObjectsOfBroadcastMessage> {
        self.http_post(
            "https://api.line.me/v2/bot/message/validate/broadcast",
            &json!({
                "messages":data
            }),
        )
        .await
    }

    // GET  /v2/bot/message/aggregation/info
    /// https://developers.line.biz/ja/reference/messaging-api/#get-number-of-units-used-this-month

    pub async fn message_aggregation_info(
        &self,
    ) -> LineApiResponse<LineApiMessageAggregationInfoResponse> {
        self.http_get(
            "https://api.line.me/v2/bot/message/aggregation/info",
            &json!({}),
        )
        .await
    }
    // GET  /v2/bot/message/aggregation/list
    /// https://developers.line.biz/ja/reference/messaging-api/#get-name-list-of-units-used-this-month
    pub async fn message_aggregation_list(
        &self,
    ) -> LineApiResponse<LineApiMessageAggregationListResponse> {
        self.http_get(
            "https://api.line.me/v2/bot/message/aggregation/list ",
            &json!({}),
        )
        .await
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiMessagePushResponse {
    #[serde(rename = "sentMessages")]
    pub sent_messages: Vec<LineApiSendMessage>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiSendMessage {
    pub id: String,
    #[serde(rename = "quoteToken")]
    pub quote_token: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiMessageSendMulticastResponse {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiMessageSendBroadcastResponse {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiMessageGetQuotaResponse {
    #[serde(rename = "type")]
    quota_type: String,
    value: Option<u32>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiMessageGetQuotaConsumptionResponse {
    #[serde(rename = "totalUsage")]
    total_usage: u32,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiMessageGetNumberOfReplyMessageResponse {
    status: String,
    success: Option<u32>,
}
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiMessageGetNumberOfPushMessageResponse {
    status: String,
    success: Option<u32>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiMessageGetNumberOfMulticastMessageResponse {
    status: String,
    success: Option<u32>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiMessageGetNumberOfBroadcastMessageResponse {
    status: String,
    success: Option<u32>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiMessageValidateMessageObjectsOfPushMessage {}
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiMessageValidateMessageObjectsOfReplyMessage {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiMessageValidateMessageObjectsOfMulticastMessage {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiMessageValidateMessageObjectsOfNarrowcastMessage {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiMessageValidateMessageObjectsOfBroadcastMessage {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiMessageAggregationInfoResponse {
    #[serde(rename = "numOfCustomAggregationUnits")]
    num_of_custom_aggregation_units: u32,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineApiMessageAggregationListResponse {
    #[serde(rename = "customAggregationUnits")]
    custom_aggregation_units: Vec<String>,
    next: Option<String>,
}
