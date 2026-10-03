use reqwest::{Method, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::LineClient;
use crate::client::{Host, RETRY_KEY_HEADER};
use crate::error::Result;
use crate::message::Message;

impl LineClient {
    /// Sends a reply message.
    /// <https://developers.line.biz/en/reference/messaging-api/#send-reply-message>
    pub async fn reply_message(
        &self,
        request: &ReplyMessageRequest,
    ) -> Result<ReplyMessageResponse> {
        let rb = self
            .request(Method::POST, Host::Api, &["v2", "bot", "message", "reply"])
            .json(request);
        self.send_json(rb).await
    }

    /// Sends a push message.
    /// <https://developers.line.biz/en/reference/messaging-api/#send-push-message>
    pub async fn push_message(&self, request: &PushMessageRequest) -> Result<PushMessageResponse> {
        let rb = self
            .request(Method::POST, Host::Api, &["v2", "bot", "message", "push"])
            .json(request);
        let (mut response, request_id): (PushMessageResponse, _) = self
            .send_json_with_request_id(with_retry_key(rb, &request.retry_key))
            .await?;
        response.request_id = request_id;
        Ok(response)
    }

    /// Sends a multicast message.
    /// <https://developers.line.biz/en/reference/messaging-api/#send-multicast-message>
    pub async fn multicast(&self, request: &MulticastRequest) -> Result<MulticastResponse> {
        let rb = self
            .request(
                Method::POST,
                Host::Api,
                &["v2", "bot", "message", "multicast"],
            )
            .json(request);
        let (mut response, request_id): (MulticastResponse, _) = self
            .send_json_with_request_id(with_retry_key(rb, &request.retry_key))
            .await?;
        response.request_id = request_id;
        Ok(response)
    }

    /// Sends a narrowcast message.
    /// <https://developers.line.biz/en/reference/messaging-api/#send-narrowcast-message>
    pub async fn narrowcast(&self, request: &NarrowcastRequest) -> Result<NarrowcastResponse> {
        let rb = self
            .request(
                Method::POST,
                Host::Api,
                &["v2", "bot", "message", "narrowcast"],
            )
            .json(request);
        let (mut response, request_id): (NarrowcastResponse, _) = self
            .send_json_with_request_id(with_retry_key(rb, &request.retry_key))
            .await?;
        response.request_id = request_id;
        Ok(response)
    }

    /// Gets the status of a narrowcast message.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-narrowcast-progress-status>
    pub async fn get_narrowcast_progress(
        &self,
        request_id: &str,
    ) -> Result<NarrowcastProgressResponse> {
        let rb = self
            .request(
                Method::GET,
                Host::Api,
                &["v2", "bot", "message", "progress", "narrowcast"],
            )
            .query(&[("requestId", request_id)]);
        self.send_json(rb).await
    }

    /// Sends a broadcast message.
    /// <https://developers.line.biz/en/reference/messaging-api/#send-broadcast-message>
    pub async fn broadcast(&self, request: &BroadcastRequest) -> Result<BroadcastResponse> {
        let rb = self
            .request(
                Method::POST,
                Host::Api,
                &["v2", "bot", "message", "broadcast"],
            )
            .json(request);
        let (mut response, request_id): (BroadcastResponse, _) = self
            .send_json_with_request_id(with_retry_key(rb, &request.retry_key))
            .await?;
        response.request_id = request_id;
        Ok(response)
    }

    /// Gets the target limit for sending messages this month.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-quota>
    pub async fn get_message_quota(&self) -> Result<MessageQuotaResponse> {
        let rb = self.request(Method::GET, Host::Api, &["v2", "bot", "message", "quota"]);
        self.send_json(rb).await
    }

    /// Gets the number of messages sent this month.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-consumption>
    pub async fn get_message_quota_consumption(&self) -> Result<QuotaConsumptionResponse> {
        let rb = self.request(
            Method::GET,
            Host::Api,
            &["v2", "bot", "message", "quota", "consumption"],
        );
        self.send_json(rb).await
    }

    /// Gets the number of sent reply messages on `date` (`yyyyMMdd`, UTC+9).
    /// <https://developers.line.biz/en/reference/messaging-api/#get-number-of-reply-messages>
    pub async fn get_number_of_sent_reply_messages(
        &self,
        date: &str,
    ) -> Result<NumberOfMessagesResponse> {
        self.get_number_of_sent_messages("reply", date).await
    }

    /// Gets the number of sent push messages on `date` (`yyyyMMdd`, UTC+9).
    /// <https://developers.line.biz/en/reference/messaging-api/#get-number-of-push-messages>
    pub async fn get_number_of_sent_push_messages(
        &self,
        date: &str,
    ) -> Result<NumberOfMessagesResponse> {
        self.get_number_of_sent_messages("push", date).await
    }

    /// Gets the number of sent multicast messages on `date` (`yyyyMMdd`, UTC+9).
    /// <https://developers.line.biz/en/reference/messaging-api/#get-number-of-multicast-messages>
    pub async fn get_number_of_sent_multicast_messages(
        &self,
        date: &str,
    ) -> Result<NumberOfMessagesResponse> {
        self.get_number_of_sent_messages("multicast", date).await
    }

    /// Gets the number of sent broadcast messages on `date` (`yyyyMMdd`, UTC+9).
    /// <https://developers.line.biz/en/reference/messaging-api/#get-number-of-broadcast-messages>
    pub async fn get_number_of_sent_broadcast_messages(
        &self,
        date: &str,
    ) -> Result<NumberOfMessagesResponse> {
        self.get_number_of_sent_messages("broadcast", date).await
    }

    async fn get_number_of_sent_messages(
        &self,
        kind: &str,
        date: &str,
    ) -> Result<NumberOfMessagesResponse> {
        let rb = self
            .request(
                Method::GET,
                Host::Api,
                &["v2", "bot", "message", "delivery", kind],
            )
            .query(&[("date", date)]);
        self.send_json(rb).await
    }

    /// Validates message objects of a reply message.
    /// <https://developers.line.biz/en/reference/messaging-api/#validate-message-objects-of-reply-message>
    pub async fn validate_reply(&self, request: &ValidateMessageRequest) -> Result<()> {
        self.validate("reply", request).await
    }

    /// Validates message objects of a push message.
    /// <https://developers.line.biz/en/reference/messaging-api/#validate-message-objects-of-push-message>
    pub async fn validate_push(&self, request: &ValidateMessageRequest) -> Result<()> {
        self.validate("push", request).await
    }

    /// Validates message objects of a multicast message.
    /// <https://developers.line.biz/en/reference/messaging-api/#validate-message-objects-of-multicast-message>
    pub async fn validate_multicast(&self, request: &ValidateMessageRequest) -> Result<()> {
        self.validate("multicast", request).await
    }

    /// Validates message objects of a narrowcast message.
    /// <https://developers.line.biz/en/reference/messaging-api/#validate-message-objects-of-narrowcast-message>
    pub async fn validate_narrowcast(&self, request: &ValidateMessageRequest) -> Result<()> {
        self.validate("narrowcast", request).await
    }

    /// Validates message objects of a broadcast message.
    /// <https://developers.line.biz/en/reference/messaging-api/#validate-message-objects-of-broadcast-message>
    pub async fn validate_broadcast(&self, request: &ValidateMessageRequest) -> Result<()> {
        self.validate("broadcast", request).await
    }

    async fn validate(&self, kind: &str, request: &ValidateMessageRequest) -> Result<()> {
        let rb = self
            .request(
                Method::POST,
                Host::Api,
                &["v2", "bot", "message", "validate", kind],
            )
            .json(request);
        self.send_empty(rb).await
    }

    /// Gets the number of units used this month.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-number-of-units-used-this-month>
    pub async fn get_aggregation_unit_usage(&self) -> Result<GetAggregationUnitUsageResponse> {
        let rb = self.request(
            Method::GET,
            Host::Api,
            &["v2", "bot", "message", "aggregation", "info"],
        );
        self.send_json(rb).await
    }

    /// Gets the name list of units used this month.
    /// `start` is the `next` value of the previous page.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-name-list-of-units-used-this-month>
    pub async fn get_aggregation_unit_name_list(
        &self,
        limit: Option<&str>,
        start: Option<&str>,
    ) -> Result<GetAggregationUnitNameListResponse> {
        let rb = self
            .request(
                Method::GET,
                Host::Api,
                &["v2", "bot", "message", "aggregation", "list"],
            )
            .query(&[("limit", limit), ("start", start)]);
        self.send_json(rb).await
    }

    /// Marks messages from a user as read (partner feature).
    /// <https://developers.line.biz/en/reference/partner-docs/#mark-messages-from-users-as-read>
    pub async fn mark_messages_as_read(&self, request: &MarkMessagesAsReadRequest) -> Result<()> {
        let rb = self
            .request(
                Method::POST,
                Host::Api,
                &["v2", "bot", "message", "markAsRead"],
            )
            .json(request);
        self.send_empty(rb).await
    }

    /// Displays a loading animation in a one-on-one chat.
    /// <https://developers.line.biz/en/reference/messaging-api/#display-a-loading-indicator>
    pub async fn show_loading_animation(
        &self,
        request: &ShowLoadingAnimationRequest,
    ) -> Result<()> {
        let rb = self
            .request(
                Method::POST,
                Host::Api,
                &["v2", "bot", "chat", "loading", "start"],
            )
            .json(request);
        self.send_empty(rb).await
    }

    /// Marks messages as read using the token from a webhook message event.
    /// <https://developers.line.biz/en/reference/messaging-api/#mark-as-read>
    pub async fn mark_messages_as_read_by_token(
        &self,
        request: &MarkMessagesAsReadByTokenRequest,
    ) -> Result<()> {
        let rb = self
            .request(
                Method::POST,
                Host::Api,
                &["v2", "bot", "chat", "markAsRead"],
            )
            .json(request);
        self.send_empty(rb).await
    }
}

fn with_retry_key(rb: RequestBuilder, retry_key: &Option<String>) -> RequestBuilder {
    match retry_key {
        Some(key) => rb.header(RETRY_KEY_HEADER, key),
        None => rb,
    }
}

fn messages(messages: impl IntoIterator<Item = impl Into<Message>>) -> Vec<Message> {
    messages.into_iter().map(Into::into).collect()
}

/// A message that was sent.
///
/// <https://developers.line.biz/en/reference/messaging-api/#send-push-message-response>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentMessage {
    /// ID of the sent message.
    pub id: String,
    /// Quote token of the message. Only included when a message that can be a quote target was sent as a push or reply message.
    pub quote_token: Option<String>,
}

/// Request body of [`LineClient::reply_message`](crate::LineClient::reply_message).
///
/// <https://developers.line.biz/en/reference/messaging-api/#send-reply-message>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplyMessageRequest {
    /// `replyToken` received via webhook.
    pub reply_token: String,
    /// Messages to send (1 to 5).
    pub messages: Vec<Message>,
    /// Whether to suppress the push notification on the user's device. Defaults to `false`.
    pub notification_disabled: Option<bool>,
}

impl ReplyMessageRequest {
    /// Creates a request that replies to `reply_token` with `messages` (1 to 5).
    pub fn new(
        reply_token: impl Into<String>,
        messages: impl IntoIterator<Item = impl Into<Message>>,
    ) -> Self {
        Self {
            reply_token: reply_token.into(),
            messages: self::messages(messages),
            notification_disabled: None,
        }
    }

    /// Sets whether to suppress the push notification on the user's device.
    pub fn notification_disabled(mut self, notification_disabled: bool) -> Self {
        self.notification_disabled = Some(notification_disabled);
        self
    }
}

/// Response of [`LineClient::reply_message`](crate::LineClient::reply_message).
///
/// <https://developers.line.biz/en/reference/messaging-api/#send-reply-message-response>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplyMessageResponse {
    /// Sent messages (1 to 5).
    pub sent_messages: Vec<SentMessage>,
}

/// Request body of [`LineClient::push_message`](crate::LineClient::push_message).
///
/// <https://developers.line.biz/en/reference/messaging-api/#send-push-message>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushMessageRequest {
    /// ID of the receiver.
    pub to: String,
    /// Messages to send (1 to 5).
    pub messages: Vec<Message>,
    /// Whether to suppress the push notification on the user's device. Defaults to `false`.
    pub notification_disabled: Option<bool>,
    /// Names of aggregation units (case-sensitive). Only available to corporate users who have submitted the required applications.
    pub custom_aggregation_units: Option<Vec<String>>,
    /// Retry key (a UUID you generate), sent as the `X-Line-Retry-Key` header.
    #[serde(skip)]
    pub retry_key: Option<String>,
}

impl PushMessageRequest {
    /// Creates a request that pushes `messages` (1 to 5) to the receiver `to`.
    pub fn new(
        to: impl Into<String>,
        messages: impl IntoIterator<Item = impl Into<Message>>,
    ) -> Self {
        Self {
            to: to.into(),
            messages: self::messages(messages),
            notification_disabled: None,
            custom_aggregation_units: None,
            retry_key: None,
        }
    }

    /// Sets whether to suppress the push notification on the user's device.
    pub fn notification_disabled(mut self, notification_disabled: bool) -> Self {
        self.notification_disabled = Some(notification_disabled);
        self
    }

    /// Sets the names of aggregation units (case-sensitive).
    pub fn custom_aggregation_units(
        mut self,
        units: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.custom_aggregation_units = Some(units.into_iter().map(Into::into).collect());
        self
    }

    /// Sets the retry key, sent as the `X-Line-Retry-Key` header.
    ///
    /// The retry key is a UUID in hexadecimal format (e.g. `123e4567-e89b-12d3-a456-426614174000`)
    /// that you generate yourself; LINE does not generate it.
    pub fn retry_key(mut self, retry_key: impl Into<String>) -> Self {
        self.retry_key = Some(retry_key.into());
        self
    }
}

/// Response of [`LineClient::push_message`](crate::LineClient::push_message).
///
/// <https://developers.line.biz/en/reference/messaging-api/#send-push-message-response>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushMessageResponse {
    /// Sent messages (1 to 5).
    pub sent_messages: Vec<SentMessage>,
    /// Value of the `x-line-request-id` response header.
    #[serde(skip)]
    pub request_id: Option<String>,
}

/// Request body of [`LineClient::multicast`](crate::LineClient::multicast).
///
/// <https://developers.line.biz/en/reference/messaging-api/#send-multicast-message>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MulticastRequest {
    /// Messages to send (1 to 5).
    pub messages: Vec<Message>,
    /// User IDs of the recipients (1 to 500). Use `userId` values returned in webhook event objects, not LINE IDs found on LINE.
    pub to: Vec<String>,
    /// Whether to suppress the push notification on the user's device. Defaults to `false`.
    pub notification_disabled: Option<bool>,
    /// Name of the aggregation unit (case-sensitive, max 1).
    pub custom_aggregation_units: Option<Vec<String>>,
    /// Retry key (a UUID you generate), sent as the `X-Line-Retry-Key` header.
    #[serde(skip)]
    pub retry_key: Option<String>,
}

impl MulticastRequest {
    /// Creates a request that sends `messages` (1 to 5) to the user IDs in `to` (1 to 500).
    pub fn new(
        to: impl IntoIterator<Item = impl Into<String>>,
        messages: impl IntoIterator<Item = impl Into<Message>>,
    ) -> Self {
        Self {
            messages: self::messages(messages),
            to: to.into_iter().map(Into::into).collect(),
            notification_disabled: None,
            custom_aggregation_units: None,
            retry_key: None,
        }
    }

    /// Sets whether to suppress the push notification on the user's device.
    pub fn notification_disabled(mut self, notification_disabled: bool) -> Self {
        self.notification_disabled = Some(notification_disabled);
        self
    }

    /// Sets the name of the aggregation unit (case-sensitive, max 1).
    pub fn custom_aggregation_units(
        mut self,
        units: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.custom_aggregation_units = Some(units.into_iter().map(Into::into).collect());
        self
    }

    /// Sets the retry key, sent as the `X-Line-Retry-Key` header.
    ///
    /// The retry key is a UUID in hexadecimal format (e.g. `123e4567-e89b-12d3-a456-426614174000`)
    /// that you generate yourself; LINE does not generate it.
    pub fn retry_key(mut self, retry_key: impl Into<String>) -> Self {
        self.retry_key = Some(retry_key.into());
        self
    }
}

/// Response of [`LineClient::multicast`](crate::LineClient::multicast).
///
/// <https://developers.line.biz/en/reference/messaging-api/#send-multicast-response>
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MulticastResponse {
    /// Value of the `x-line-request-id` response header.
    #[serde(skip)]
    pub request_id: Option<String>,
}

/// Request body of [`LineClient::narrowcast`](crate::LineClient::narrowcast).
///
/// <https://developers.line.biz/en/reference/messaging-api/#send-narrowcast-message>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NarrowcastRequest {
    /// Messages to send (1 to 5).
    pub messages: Vec<Message>,
    /// Recipients specified by audiences or by the request ID of an earlier message.
    pub recipient: Option<Recipient>,
    /// Demographic filter for the recipients.
    pub filter: Option<Filter>,
    /// Limit on the number of recipients.
    pub limit: Option<Limit>,
    /// Whether to suppress the push notification on the user's device. Defaults to `false`.
    pub notification_disabled: Option<bool>,
    /// Retry key (a UUID you generate), sent as the `X-Line-Retry-Key` header.
    #[serde(skip)]
    pub retry_key: Option<String>,
}

impl NarrowcastRequest {
    /// Creates a request that sends `messages` (1 to 5).
    pub fn new(messages: impl IntoIterator<Item = impl Into<Message>>) -> Self {
        Self {
            messages: self::messages(messages),
            recipient: None,
            filter: None,
            limit: None,
            notification_disabled: None,
            retry_key: None,
        }
    }

    /// Sets the recipients.
    pub fn recipient(mut self, recipient: impl Into<Recipient>) -> Self {
        self.recipient = Some(recipient.into());
        self
    }

    /// Sets the demographic filter.
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }

    /// Sets the limit on the number of recipients.
    pub fn limit(mut self, limit: Limit) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets whether to suppress the push notification on the user's device.
    pub fn notification_disabled(mut self, notification_disabled: bool) -> Self {
        self.notification_disabled = Some(notification_disabled);
        self
    }

    /// Sets the retry key, sent as the `X-Line-Retry-Key` header.
    ///
    /// The retry key is a UUID in hexadecimal format (e.g. `123e4567-e89b-12d3-a456-426614174000`)
    /// that you generate yourself; LINE does not generate it.
    pub fn retry_key(mut self, retry_key: impl Into<String>) -> Self {
        self.retry_key = Some(retry_key.into());
        self
    }
}

/// Response of [`LineClient::narrowcast`](crate::LineClient::narrowcast).
///
/// <https://developers.line.biz/en/reference/messaging-api/#send-narrowcast-response>
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct NarrowcastResponse {
    /// Value of the `x-line-request-id` response header.
    /// Pass it to [`LineClient::get_narrowcast_progress`].
    #[serde(skip)]
    pub request_id: Option<String>,
}

/// Recipient of a narrowcast message.
/// <https://developers.line.biz/en/reference/messaging-api/#narrowcast-recipient>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Recipient {
    /// Logical operation on other recipients.
    #[serde(rename = "operator")]
    Operator(OperatorRecipient),
    /// Users in an audience.
    #[serde(rename = "audience")]
    Audience(AudienceRecipient),
    /// Users who received an earlier message.
    #[serde(rename = "redelivery")]
    Redelivery(RedeliveryRecipient),
}

/// Combines recipients with `and`, `or` and `not`.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperatorRecipient {
    /// Logical conjunction (AND) of the recipients.
    pub and: Option<Vec<Recipient>>,
    /// Logical disjunction (OR) of the recipients.
    pub or: Option<Vec<Recipient>>,
    /// Logical negation (NOT) of the recipient.
    pub not: Option<Box<Recipient>>,
}

impl OperatorRecipient {
    /// Sets the recipients to combine with AND.
    pub fn and(mut self, recipients: impl IntoIterator<Item = impl Into<Recipient>>) -> Self {
        self.and = Some(recipients.into_iter().map(Into::into).collect());
        self
    }

    /// Sets the recipients to combine with OR.
    pub fn or(mut self, recipients: impl IntoIterator<Item = impl Into<Recipient>>) -> Self {
        self.or = Some(recipients.into_iter().map(Into::into).collect());
        self
    }

    /// Sets the recipient to negate.
    pub fn not(mut self, recipient: impl Into<Recipient>) -> Self {
        self.not = Some(Box::new(recipient.into()));
        self
    }
}

impl From<OperatorRecipient> for Recipient {
    fn from(value: OperatorRecipient) -> Self {
        Recipient::Operator(value)
    }
}

/// Users in an audience.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudienceRecipient {
    /// Audience ID.
    pub audience_group_id: Option<i64>,
}

impl AudienceRecipient {
    /// Sets the audience ID.
    pub fn audience_group_id(mut self, audience_group_id: i64) -> Self {
        self.audience_group_id = Some(audience_group_id);
        self
    }
}

impl From<AudienceRecipient> for Recipient {
    fn from(value: AudienceRecipient) -> Self {
        Recipient::Audience(value)
    }
}

/// Users who received an earlier narrowcast or broadcast message.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedeliveryRecipient {
    /// Request ID of the earlier narrowcast or broadcast message.
    pub request_id: Option<String>,
}

impl RedeliveryRecipient {
    /// Sets the request ID of the earlier narrowcast or broadcast message.
    pub fn request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
}

impl From<RedeliveryRecipient> for Recipient {
    fn from(value: RedeliveryRecipient) -> Self {
        Recipient::Redelivery(value)
    }
}

/// Filter for narrowcast.
///
/// <https://developers.line.biz/en/reference/messaging-api/#narrowcast-demographic-filter>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    /// Demographic filter.
    pub demographic: Option<DemographicFilter>,
}

impl Filter {
    /// Sets the demographic filter.
    pub fn demographic(mut self, demographic: impl Into<DemographicFilter>) -> Self {
        self.demographic = Some(demographic.into());
        self
    }
}

/// Demographic filter of a narrowcast message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DemographicFilter {
    /// Filter by age.
    #[serde(rename = "age")]
    Age(AgeDemographicFilter),
    /// Filter by OS of the user's device.
    #[serde(rename = "appType")]
    AppType(AppTypeDemographicFilter),
    /// Filter by area.
    #[serde(rename = "area")]
    Area(AreaDemographicFilter),
    /// Filter by gender.
    #[serde(rename = "gender")]
    Gender(GenderDemographicFilter),
    /// Logical operation on other demographic filters.
    #[serde(rename = "operator")]
    Operator(OperatorDemographicFilter),
    /// Filter by how long the user has been a friend.
    #[serde(rename = "subscriptionPeriod")]
    SubscriptionPeriod(SubscriptionPeriodDemographicFilter),
}

/// Filter by age.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgeDemographicFilter {
    /// Lower bound of the age range (inclusive).
    pub gte: Option<AgeDemographic>,
    /// Upper bound of the age range (exclusive).
    pub lt: Option<AgeDemographic>,
}

impl AgeDemographicFilter {
    /// Sets the lower bound of the age range (inclusive).
    pub fn gte(mut self, gte: AgeDemographic) -> Self {
        self.gte = Some(gte);
        self
    }

    /// Sets the upper bound of the age range (exclusive).
    pub fn lt(mut self, lt: AgeDemographic) -> Self {
        self.lt = Some(lt);
        self
    }
}

impl From<AgeDemographicFilter> for DemographicFilter {
    fn from(value: AgeDemographicFilter) -> Self {
        DemographicFilter::Age(value)
    }
}

/// Age used in [`AgeDemographicFilter`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgeDemographic {
    /// `age_15`: 15 years old.
    #[serde(rename = "age_15")]
    Age15,
    /// `age_20`: 20 years old.
    #[serde(rename = "age_20")]
    Age20,
    /// `age_25`: 25 years old.
    #[serde(rename = "age_25")]
    Age25,
    /// `age_30`: 30 years old.
    #[serde(rename = "age_30")]
    Age30,
    /// `age_35`: 35 years old.
    #[serde(rename = "age_35")]
    Age35,
    /// `age_40`: 40 years old.
    #[serde(rename = "age_40")]
    Age40,
    /// `age_45`: 45 years old.
    #[serde(rename = "age_45")]
    Age45,
    /// `age_50`: 50 years old.
    #[serde(rename = "age_50")]
    Age50,
    /// `age_55`: 55 years old.
    #[serde(rename = "age_55")]
    Age55,
    /// `age_60`: 60 years old.
    #[serde(rename = "age_60")]
    Age60,
    /// `age_65`: 65 years old.
    #[serde(rename = "age_65")]
    Age65,
    /// `age_70`: 70 years old.
    #[serde(rename = "age_70")]
    Age70,
}

/// Filter by OS of the user's device.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppTypeDemographicFilter {
    /// OS types to match.
    pub one_of: Option<Vec<AppTypeDemographic>>,
}

impl AppTypeDemographicFilter {
    /// Sets the OS types to match.
    pub fn one_of(mut self, one_of: impl IntoIterator<Item = AppTypeDemographic>) -> Self {
        self.one_of = Some(one_of.into_iter().collect());
        self
    }
}

impl From<AppTypeDemographicFilter> for DemographicFilter {
    fn from(value: AppTypeDemographicFilter) -> Self {
        DemographicFilter::AppType(value)
    }
}

/// OS type used in [`AppTypeDemographicFilter`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AppTypeDemographic {
    /// iOS.
    Ios,
    /// Android.
    Android,
}

/// Filter by area.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AreaDemographicFilter {
    /// Areas to match.
    pub one_of: Option<Vec<AreaDemographic>>,
}

impl AreaDemographicFilter {
    /// Sets the areas to match.
    pub fn one_of(mut self, one_of: impl IntoIterator<Item = AreaDemographic>) -> Self {
        self.one_of = Some(one_of.into_iter().collect());
        self
    }
}

impl From<AreaDemographicFilter> for DemographicFilter {
    fn from(value: AreaDemographicFilter) -> Self {
        DemographicFilter::Area(value)
    }
}

/// Area code of a narrowcast demographic filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AreaDemographic {
    /// jp_01: 北海道 // Hokkaido
    #[serde(rename = "jp_01")]
    Hokkaido,
    /// jp_02: 青森県 // Aomori
    #[serde(rename = "jp_02")]
    Aomori,
    /// jp_03: 岩手県 // Iwate
    #[serde(rename = "jp_03")]
    Iwate,
    /// jp_04: 宮城県 // Miyagi
    #[serde(rename = "jp_04")]
    Miyagi,
    /// jp_05: 秋田県 // Akita
    #[serde(rename = "jp_05")]
    Akita,
    /// jp_06: 山形県 // Yamagata
    #[serde(rename = "jp_06")]
    Yamagata,
    /// jp_07: 福島県 // Fukushima
    #[serde(rename = "jp_07")]
    Fukushima,
    /// jp_08: 茨城県 // Ibaraki
    #[serde(rename = "jp_08")]
    Ibaraki,
    /// jp_09: 栃木県 // Tochigi
    #[serde(rename = "jp_09")]
    Tochigi,
    /// jp_10: 群馬県 // Gunma
    #[serde(rename = "jp_10")]
    Gunma,
    /// jp_11: 埼玉県 // Saitama
    #[serde(rename = "jp_11")]
    Saitama,
    /// jp_12: 千葉県 // Chiba
    #[serde(rename = "jp_12")]
    Chiba,
    /// jp_13: 東京都 // Tokyo
    #[serde(rename = "jp_13")]
    Tokyo,
    /// jp_14: 神奈川県 // Kanagawa
    #[serde(rename = "jp_14")]
    Kanagawa,
    /// jp_15: 新潟県 // Niigata
    #[serde(rename = "jp_15")]
    Niigata,
    /// jp_16: 富山県 // Toyama
    #[serde(rename = "jp_16")]
    Toyama,
    /// jp_17: 石川県 // Ishikawa
    #[serde(rename = "jp_17")]
    Ishikawa,
    /// jp_18: 福井県 // Fukui
    #[serde(rename = "jp_18")]
    Fukui,
    /// jp_19: 山梨県 // Yamanashi
    #[serde(rename = "jp_19")]
    Yamanashi,
    /// jp_20: 長野県 // Nagano
    #[serde(rename = "jp_20")]
    Nagano,
    /// jp_21: 岐阜県 // Gifu
    #[serde(rename = "jp_21")]
    Gifu,
    /// jp_22: 静岡県 // Shizuoka
    #[serde(rename = "jp_22")]
    Shizuoka,
    /// jp_23: 愛知県 // Aichi
    #[serde(rename = "jp_23")]
    Aichi,
    /// jp_24: 三重県 // Mie
    #[serde(rename = "jp_24")]
    Mie,
    /// jp_25: 滋賀県 // Shiga
    #[serde(rename = "jp_25")]
    Shiga,
    /// jp_26: 京都府 // Kyoto
    #[serde(rename = "jp_26")]
    Kyoto,
    /// jp_27: 大阪府 // Osaka
    #[serde(rename = "jp_27")]
    Osaka,
    /// jp_28: 兵庫県 // Hyougo
    #[serde(rename = "jp_28")]
    Hyougo,
    /// jp_29: 奈良県 // Nara
    #[serde(rename = "jp_29")]
    Nara,
    /// jp_30: 和歌山県 // Wakayama
    #[serde(rename = "jp_30")]
    Wakayama,
    /// jp_31: 鳥取県 // Tottori
    #[serde(rename = "jp_31")]
    Tottori,
    /// jp_32: 島根県 // Shimane
    #[serde(rename = "jp_32")]
    Shimane,
    /// jp_33: 岡山県 // Okayama
    #[serde(rename = "jp_33")]
    Okayama,
    /// jp_34: 広島県 // Hiroshima
    #[serde(rename = "jp_34")]
    Hiroshima,
    /// jp_35: 山口県 // Yamaguchi
    #[serde(rename = "jp_35")]
    Yamaguchi,
    /// jp_36: 徳島県 // Tokushima
    #[serde(rename = "jp_36")]
    Tokushima,
    /// jp_37: 香川県 // Kagawa
    #[serde(rename = "jp_37")]
    Kagawa,
    /// jp_38: 愛媛県 // Ehime
    #[serde(rename = "jp_38")]
    Ehime,
    /// jp_39: 高知県 // Kouchi
    #[serde(rename = "jp_39")]
    Kouchi,
    /// jp_40: 福岡県 // Fukuoka
    #[serde(rename = "jp_40")]
    Fukuoka,
    /// jp_41: 佐賀県 // Saga
    #[serde(rename = "jp_41")]
    Saga,
    /// jp_42: 長崎県 // Nagasaki
    #[serde(rename = "jp_42")]
    Nagasaki,
    /// jp_43: 熊本県 // Kumamoto
    #[serde(rename = "jp_43")]
    Kumamoto,
    /// jp_44: 大分県 // Oita
    #[serde(rename = "jp_44")]
    Oita,
    /// jp_45: 宮崎県 // Miyazaki
    #[serde(rename = "jp_45")]
    Miyazaki,
    /// jp_46: 鹿児島県 // Kagoshima
    #[serde(rename = "jp_46")]
    Kagoshima,
    /// jp_47: 沖縄県 // Okinawa
    #[serde(rename = "jp_47")]
    Okinawa,
    /// tw_01: 台北市 // Taipei City
    #[serde(rename = "tw_01")]
    TaipeiCity,
    /// tw_02: 新北市 // New Taipei City
    #[serde(rename = "tw_02")]
    NewTaipeiCity,
    /// tw_03: 桃園市 // Taoyuan City
    #[serde(rename = "tw_03")]
    TaoyuanCity,
    /// tw_04: 台中市 // Taichung City
    #[serde(rename = "tw_04")]
    TaichungCity,
    /// tw_05: 台南市 // Tainan City
    #[serde(rename = "tw_05")]
    TainanCity,
    /// tw_06: 高雄市 // Kaohsiung City
    #[serde(rename = "tw_06")]
    KaohsiungCity,
    /// tw_07: 基隆市 // Keelung City
    #[serde(rename = "tw_07")]
    KeelungCity,
    /// tw_08: 新竹市 // Hsinchu City
    #[serde(rename = "tw_08")]
    HsinchuCity,
    /// tw_09: 嘉義市 // Chiayi City
    #[serde(rename = "tw_09")]
    ChiayiCity,
    /// tw_10: 新竹県 // Hsinchu County
    #[serde(rename = "tw_10")]
    HsinchuCounty,
    /// tw_11: 苗栗県 // Miaoli County
    #[serde(rename = "tw_11")]
    MiaoliCounty,
    /// tw_12: 彰化県 // Changhua County
    #[serde(rename = "tw_12")]
    ChanghuaCounty,
    /// tw_13: 南投県 // Nantou County
    #[serde(rename = "tw_13")]
    NantouCounty,
    /// tw_14: 雲林県 // Yunlin County
    #[serde(rename = "tw_14")]
    YunlinCounty,
    /// tw_15: 嘉義県 // Chiayi County
    #[serde(rename = "tw_15")]
    ChiayiCounty,
    /// tw_16: 屏東県 // Pingtung County
    #[serde(rename = "tw_16")]
    PingtungCounty,
    /// tw_17: 宜蘭県 // Yilan County
    #[serde(rename = "tw_17")]
    YilanCounty,
    /// tw_18: 花蓮県 // Hualien County
    #[serde(rename = "tw_18")]
    HualienCounty,
    /// tw_19: 台東県 // Taitung County
    #[serde(rename = "tw_19")]
    TaitungCounty,
    /// tw_20: 澎湖県 // Penghu County
    #[serde(rename = "tw_20")]
    PenghuCounty,
    /// tw_21: 金門県 // Kinmen County
    #[serde(rename = "tw_21")]
    KinmenCounty,
    /// tw_22: 連江県 // Lienchiang County
    #[serde(rename = "tw_22")]
    LienchiangCounty,
    /// th_01: バンコク // Bangkok
    #[serde(rename = "th_01")]
    Bangkok,
    /// th_02: パタヤ // Pattaya
    #[serde(rename = "th_02")]
    Pattaya,
    /// th_03: 北部 // Northern
    #[serde(rename = "th_03")]
    Northern,
    /// th_04: 中央部 // Central
    #[serde(rename = "th_04")]
    Central,
    /// th_05: 南部 // Southern
    #[serde(rename = "th_05")]
    Southern,
    /// th_06: 東部 // Eastern
    #[serde(rename = "th_06")]
    Eastern,
    /// th_07: 東北部 // NorthEastern
    #[serde(rename = "th_07")]
    Northeastern,
    /// th_08: 西部 // Western
    #[serde(rename = "th_08")]
    Western,
    /// id_01: バリ // Bali
    #[serde(rename = "id_01")]
    Bali,
    /// id_02: バンドン // Bandung
    #[serde(rename = "id_02")]
    Bandung,
    /// id_03: バンジャルマシン // Banjarmasin
    #[serde(rename = "id_03")]
    Banjarmasin,
    /// id_04: ジャボデタベック (ジャカルタ首都圏) // Jabodetabek
    #[serde(rename = "id_04")]
    Jabodetabek,
    /// id_05: その他のエリア // Lainnya
    #[serde(rename = "id_05")]
    Lainnya,
    /// id_06: マカッサル // Makassar
    #[serde(rename = "id_06")]
    Makassar,
    /// id_07: メダン // Medan
    #[serde(rename = "id_07")]
    Medan,
    /// id_08: パレンバン // Palembang
    #[serde(rename = "id_08")]
    Palembang,
    /// id_09: サマリンダ // Samarinda
    #[serde(rename = "id_09")]
    Samarinda,
    /// id_10: スマラン // Semarang
    #[serde(rename = "id_10")]
    Semarang,
    /// id_11: スラバヤ // Surabaya
    #[serde(rename = "id_11")]
    Surabaya,
    /// id_12: ジョグジャカルタ // Yogyakarta
    #[serde(rename = "id_12")]
    Yogyakarta,
}

/// Filter by gender.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenderDemographicFilter {
    /// Genders to match.
    pub one_of: Option<Vec<GenderDemographic>>,
}

impl GenderDemographicFilter {
    /// Sets the genders to match.
    pub fn one_of(mut self, one_of: impl IntoIterator<Item = GenderDemographic>) -> Self {
        self.one_of = Some(one_of.into_iter().collect());
        self
    }
}

impl From<GenderDemographicFilter> for DemographicFilter {
    fn from(value: GenderDemographicFilter) -> Self {
        DemographicFilter::Gender(value)
    }
}

/// Gender used in [`GenderDemographicFilter`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GenderDemographic {
    /// Male.
    Male,
    /// Female.
    Female,
}

/// Combines demographic filters with `and`, `or` and `not`.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperatorDemographicFilter {
    /// Logical conjunction (AND) of the demographic filters.
    pub and: Option<Vec<DemographicFilter>>,
    /// Logical disjunction (OR) of the demographic filters.
    pub or: Option<Vec<DemographicFilter>>,
    /// Logical negation (NOT) of the demographic filter.
    pub not: Option<Box<DemographicFilter>>,
}

impl OperatorDemographicFilter {
    /// Sets the demographic filters to combine with AND.
    pub fn and(mut self, filters: impl IntoIterator<Item = impl Into<DemographicFilter>>) -> Self {
        self.and = Some(filters.into_iter().map(Into::into).collect());
        self
    }

    /// Sets the demographic filters to combine with OR.
    pub fn or(mut self, filters: impl IntoIterator<Item = impl Into<DemographicFilter>>) -> Self {
        self.or = Some(filters.into_iter().map(Into::into).collect());
        self
    }

    /// Sets the demographic filter to negate.
    pub fn not(mut self, filter: impl Into<DemographicFilter>) -> Self {
        self.not = Some(Box::new(filter.into()));
        self
    }
}

impl From<OperatorDemographicFilter> for DemographicFilter {
    fn from(value: OperatorDemographicFilter) -> Self {
        DemographicFilter::Operator(value)
    }
}

/// Filter by how long the user has been a friend.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionPeriodDemographicFilter {
    /// Lower bound of the period (inclusive).
    pub gte: Option<SubscriptionPeriodDemographic>,
    /// Upper bound of the period (exclusive).
    pub lt: Option<SubscriptionPeriodDemographic>,
}

impl SubscriptionPeriodDemographicFilter {
    /// Sets the lower bound of the period (inclusive).
    pub fn gte(mut self, gte: SubscriptionPeriodDemographic) -> Self {
        self.gte = Some(gte);
        self
    }

    /// Sets the upper bound of the period (exclusive).
    pub fn lt(mut self, lt: SubscriptionPeriodDemographic) -> Self {
        self.lt = Some(lt);
        self
    }
}

impl From<SubscriptionPeriodDemographicFilter> for DemographicFilter {
    fn from(value: SubscriptionPeriodDemographicFilter) -> Self {
        DemographicFilter::SubscriptionPeriod(value)
    }
}

/// Friendship period used in [`SubscriptionPeriodDemographicFilter`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionPeriodDemographic {
    /// `day_7`: 7 days.
    #[serde(rename = "day_7")]
    Day7,
    /// `day_30`: 30 days.
    #[serde(rename = "day_30")]
    Day30,
    /// `day_90`: 90 days.
    #[serde(rename = "day_90")]
    Day90,
    /// `day_180`: 180 days.
    #[serde(rename = "day_180")]
    Day180,
    /// `day_365`: 365 days.
    #[serde(rename = "day_365")]
    Day365,
}

/// Maximum number of narrowcast recipients.
/// <https://developers.line.biz/en/reference/messaging-api/#send-narrowcast-message>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Limit {
    /// Maximum number of narrowcast messages to send (min 1). Recipients are chosen at random.
    pub max: Option<u32>,
    /// If `true`, the message is sent within the maximum number of deliverable messages, with targets selected at random. Defaults to `false`.
    pub up_to_remaining_quota: Option<bool>,
    /// If `true`, prevents delivery to only a subset of the target audience; the request succeeds but fails asynchronously. Can be `true` only if `up_to_remaining_quota` is `true`.
    pub forbid_partial_delivery: Option<bool>,
}

impl Limit {
    /// Sets the maximum number of narrowcast messages to send (min 1).
    pub fn max(mut self, max: u32) -> Self {
        self.max = Some(max);
        self
    }

    /// Sets whether to send within the maximum number of deliverable messages.
    pub fn up_to_remaining_quota(mut self, up_to_remaining_quota: bool) -> Self {
        self.up_to_remaining_quota = Some(up_to_remaining_quota);
        self
    }

    /// Sets whether to prevent delivery to only a subset of the target audience.
    pub fn forbid_partial_delivery(mut self, forbid_partial_delivery: bool) -> Self {
        self.forbid_partial_delivery = Some(forbid_partial_delivery);
        self
    }
}

/// Progress of a narrowcast message.
///
/// <https://developers.line.biz/en/reference/messaging-api/#get-narrowcast-progress-status>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NarrowcastProgressResponse {
    /// The current status.
    pub phase: NarrowcastPhase,
    /// The number of users who successfully received the message.
    pub success_count: Option<u64>,
    /// The number of users who failed to send the message.
    pub failure_count: Option<u64>,
    /// The number of intended recipients of the message.
    pub target_count: Option<u64>,
    /// The reason the message failed to be sent. Only included when `phase` is `failed`.
    pub failed_description: Option<String>,
    /// Error summary. Only included when `phase` is `failed`.
    ///
    /// - `1`: An internal error occurred.
    /// - `2`: There weren't enough recipients.
    /// - `3`: A request that has already been accepted was retried.
    /// - `4`: An audience of less than 50 recipients is included as a condition of sending.
    /// - `5`: Delivery was canceled to prevent delivery to only a subset of the target audience.
    pub error_code: Option<i64>,
    /// Time the request was accepted, in ISO 8601 with milliseconds (UTC), e.g. `2020-12-03T10:15:30.121Z`.
    pub accepted_time: String,
    /// Time processing of the request completed, in ISO 8601 with milliseconds (UTC). Returned when `phase` is `succeeded` or `failed`.
    pub completed_time: Option<String>,
}

/// Status of a narrowcast message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NarrowcastPhase {
    /// Messages are not yet ready to be sent. They are currently being filtered or processed.
    Waiting,
    /// Messages are currently being sent.
    Sending,
    /// Messages were sent successfully. This may not mean the messages were successfully received.
    Succeeded,
    /// Messages failed to be sent. See `failed_description` for the cause.
    Failed,
    /// A value this crate does not know about.
    #[serde(other)]
    Unknown,
}

/// Request body of [`LineClient::broadcast`](crate::LineClient::broadcast).
///
/// <https://developers.line.biz/en/reference/messaging-api/#send-broadcast-message>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastRequest {
    /// Messages to send (1 to 5).
    pub messages: Vec<Message>,
    /// Whether to suppress the push notification on the user's device. Defaults to `false`.
    pub notification_disabled: Option<bool>,
    /// Retry key (a UUID you generate), sent as the `X-Line-Retry-Key` header.
    #[serde(skip)]
    pub retry_key: Option<String>,
}

impl BroadcastRequest {
    /// Creates a request that broadcasts `messages` (1 to 5).
    pub fn new(messages: impl IntoIterator<Item = impl Into<Message>>) -> Self {
        Self {
            messages: self::messages(messages),
            notification_disabled: None,
            retry_key: None,
        }
    }

    /// Sets whether to suppress the push notification on the user's device.
    pub fn notification_disabled(mut self, notification_disabled: bool) -> Self {
        self.notification_disabled = Some(notification_disabled);
        self
    }

    /// Sets the retry key, sent as the `X-Line-Retry-Key` header.
    ///
    /// The retry key is a UUID in hexadecimal format (e.g. `123e4567-e89b-12d3-a456-426614174000`)
    /// that you generate yourself; LINE does not generate it.
    pub fn retry_key(mut self, retry_key: impl Into<String>) -> Self {
        self.retry_key = Some(retry_key.into());
        self
    }
}

/// Response of [`LineClient::broadcast`](crate::LineClient::broadcast).
///
/// <https://developers.line.biz/en/reference/messaging-api/#send-broadcast-response>
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BroadcastResponse {
    /// Value of the `x-line-request-id` response header.
    #[serde(skip)]
    pub request_id: Option<String>,
}

/// Target limit for sending messages in the current month.
///
/// <https://developers.line.biz/en/reference/messaging-api/#get-quota>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageQuotaResponse {
    /// Whether a target limit is set.
    pub r#type: QuotaType,
    /// The target limit for sending messages in the current month. Returned when `type` is `limited`.
    pub value: Option<u64>,
}

/// Whether a target limit is set.
///
/// <https://developers.line.biz/en/reference/messaging-api/#get-quota>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QuotaType {
    /// No target limit is set.
    None,
    /// A target limit is set.
    Limited,
    /// A value this crate does not know about.
    #[serde(other)]
    Unknown,
}

/// Number of messages sent in the current month.
///
/// <https://developers.line.biz/en/reference/messaging-api/#get-consumption>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotaConsumptionResponse {
    /// The number of sent messages in the current month.
    pub total_usage: u64,
}

/// Number of messages sent on a given date.
///
/// <https://developers.line.biz/en/reference/messaging-api/#get-number-of-reply-messages>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberOfMessagesResponse {
    /// Aggregation process status.
    pub status: NumberOfMessagesStatus,
    /// The number of messages delivered on the specified date. Only included when `status` is `ready`.
    pub success: Option<u64>,
}

/// Aggregation process status of the number of sent messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NumberOfMessagesStatus {
    /// The number of messages can be obtained.
    Ready,
    /// Calculation for the specified date hasn't finished (e.g. the delivery date or a future date was specified). Calculation usually takes about a day.
    Unready,
    /// The total number of messages on the specified day is less than 20.
    UnavailableForPrivacy,
    /// The specified date is earlier than March 31, 2018, when calculation of sent messages started.
    OutOfService,
    /// A value this crate does not know about.
    #[serde(other)]
    Unknown,
}

/// Request body of the `validate_*` endpoints.
/// <https://developers.line.biz/en/reference/messaging-api/#validate-message-objects-of-reply-message>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateMessageRequest {
    /// Message objects to validate (1 to 5).
    pub messages: Vec<Message>,
}

impl ValidateMessageRequest {
    /// Creates a request that validates `messages` (1 to 5).
    pub fn new(messages: impl IntoIterator<Item = impl Into<Message>>) -> Self {
        Self {
            messages: self::messages(messages),
        }
    }
}

/// Number of aggregation units used this month.
///
/// <https://developers.line.biz/en/reference/messaging-api/#get-number-of-units-used-this-month>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAggregationUnitUsageResponse {
    /// Number of aggregation units used this month.
    pub num_of_custom_aggregation_units: u64,
}

/// Names of aggregation units used this month.
///
/// <https://developers.line.biz/en/reference/messaging-api/#get-name-list-of-units-used-this-month>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAggregationUnitNameListResponse {
    /// Names of aggregation units used this month.
    pub custom_aggregation_units: Vec<String>,
    /// Continuation token to get the next page of unit names. Returned only when unit names remain.
    pub next: Option<String>,
}

/// Request body of [`LineClient::mark_messages_as_read`](crate::LineClient::mark_messages_as_read).
///
/// <https://developers.line.biz/en/reference/partner-docs/#mark-messages-from-users-as-read>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkMessagesAsReadRequest {
    /// The chat whose messages are marked as read.
    pub chat: ChatReference,
}

impl MarkMessagesAsReadRequest {
    /// Creates a request that marks messages in `chat` as read.
    pub fn new(chat: ChatReference) -> Self {
        Self { chat }
    }
}

/// Chat reference.
///
/// <https://developers.line.biz/en/reference/partner-docs/#mark-messages-from-users-as-read>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatReference {
    /// The target user ID.
    pub user_id: String,
}

impl ChatReference {
    /// Creates a reference to the chat with `user_id`.
    pub fn new(user_id: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
        }
    }
}

/// Request body of [`LineClient::show_loading_animation`](crate::LineClient::show_loading_animation).
///
/// <https://developers.line.biz/en/reference/messaging-api/#display-a-loading-indicator-request-body>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowLoadingAnimationRequest {
    /// User ID of the target user for whom the loading animation is displayed.
    pub chat_id: String,
    /// Number of seconds to display the loading indicator: a multiple of 5, from 5 to 60.
    pub loading_seconds: Option<u32>,
}

impl ShowLoadingAnimationRequest {
    /// Creates a request that displays the loading animation to the user `chat_id`.
    pub fn new(chat_id: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            loading_seconds: None,
        }
    }

    /// 5 to 60 seconds, in steps of 5.
    pub fn loading_seconds(mut self, loading_seconds: u32) -> Self {
        self.loading_seconds = Some(loading_seconds);
        self
    }
}

/// Request body of [`LineClient::mark_messages_as_read_by_token`](crate::LineClient::mark_messages_as_read_by_token).
///
/// <https://developers.line.biz/en/reference/messaging-api/#mark-as-read-request-body>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkMessagesAsReadByTokenRequest {
    /// Token used to mark messages as read.
    pub mark_as_read_token: String,
}

impl MarkMessagesAsReadByTokenRequest {
    /// Creates a request with the token used to mark messages as read.
    pub fn new(mark_as_read_token: impl Into<String>) -> Self {
        Self {
            mark_as_read_token: mark_as_read_token.into(),
        }
    }
}
