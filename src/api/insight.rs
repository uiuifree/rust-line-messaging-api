use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::client::{Host, LineClient};
use crate::error::Result;

impl LineClient {
    /// Gets the demographic attributes of the LINE Official Account's friends.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-demographic>
    pub async fn get_friends_demographics(&self) -> Result<GetFriendsDemographicsResponse> {
        let rb = self.request(
            Method::GET,
            Host::Api,
            &["v2", "bot", "insight", "demographic"],
        );
        self.send_json(rb).await
    }

    /// Gets the number of messages sent on a date (`yyyyMMdd`, UTC+9).
    /// <https://developers.line.biz/en/reference/messaging-api/#get-number-of-delivery-messages>
    pub async fn get_number_of_message_deliveries(
        &self,
        date: &str,
    ) -> Result<GetNumberOfMessageDeliveriesResponse> {
        let rb = self
            .request(
                Method::GET,
                Host::Api,
                &["v2", "bot", "insight", "message", "delivery"],
            )
            .query(&[("date", date)]);
        self.send_json(rb).await
    }

    /// Gets the number of followers as of a date (`yyyyMMdd`, UTC+9).
    /// <https://developers.line.biz/en/reference/messaging-api/#get-number-of-followers>
    pub async fn get_number_of_followers(
        &self,
        date: Option<&str>,
    ) -> Result<GetNumberOfFollowersResponse> {
        let rb = self
            .request(
                Method::GET,
                Host::Api,
                &["v2", "bot", "insight", "followers"],
            )
            .query(&[("date", date)]);
        self.send_json(rb).await
    }

    /// Gets statistics about how users interacted with a narrowcast or broadcast message.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-message-event>
    pub async fn get_message_event(&self, request_id: &str) -> Result<GetMessageEventResponse> {
        let rb = self
            .request(
                Method::GET,
                Host::Api,
                &["v2", "bot", "insight", "message", "event"],
            )
            .query(&[("requestId", request_id)]);
        self.send_json(rb).await
    }

    /// Gets statistics per aggregation unit for a period (`yyyyMMdd`, UTC+9).
    /// <https://developers.line.biz/en/reference/messaging-api/#get-statistics-per-unit>
    pub async fn get_statistics_per_unit(
        &self,
        custom_aggregation_unit: &str,
        from: &str,
        to: &str,
    ) -> Result<GetStatisticsPerUnitResponse> {
        let rb = self
            .request(
                Method::GET,
                Host::Api,
                &["v2", "bot", "insight", "message", "event", "aggregation"],
            )
            .query(&[
                ("customAggregationUnit", custom_aggregation_unit),
                ("from", from),
                ("to", to),
            ]);
        self.send_json(rb).await
    }

    /// Gets impression and click statistics of a rich menu for a period (`yyyyMMdd`, UTC+9).
    /// <https://developers.line.biz/en/reference/messaging-api/#get-rich-menu-insight-summary>
    pub async fn get_rich_menu_insight_summary(
        &self,
        rich_menu_id: &str,
        from: &str,
        to: &str,
    ) -> Result<GetRichMenuInsightSummaryResponse> {
        let rb = self
            .request(
                Method::GET,
                Host::Api,
                &["v2", "bot", "insight", "richmenu", rich_menu_id, "summary"],
            )
            .query(&[("from", from), ("to", to)]);
        self.send_json(rb).await
    }

    /// Gets daily impression and click statistics of a rich menu for a period (`yyyyMMdd`, UTC+9).
    /// <https://developers.line.biz/en/reference/messaging-api/#get-rich-menu-insight-daily>
    pub async fn get_rich_menu_insight_daily(
        &self,
        rich_menu_id: &str,
        from: &str,
        to: &str,
    ) -> Result<GetRichMenuInsightDailyResponse> {
        let rb = self
            .request(
                Method::GET,
                Host::Api,
                &["v2", "bot", "insight", "richmenu", rich_menu_id, "daily"],
            )
            .query(&[("from", from), ("to", to)]);
        self.send_json(rb).await
    }
}

/// Friend demographics of the LINE Official Account.
/// <https://developers.line.biz/en/reference/messaging-api/#get-demographic>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFriendsDemographicsResponse {
    /// `true` if friend demographic information is available.
    pub available: Option<bool>,
    /// Percentage per gender.
    pub genders: Option<Vec<GenderTile>>,
    /// Percentage per age group.
    pub ages: Option<Vec<AgeTile>>,
    /// Percentage per area.
    pub areas: Option<Vec<AreaTile>>,
    /// Percentage by OS.
    pub app_types: Option<Vec<AppTypeTile>>,
    /// Percentage per friendship duration.
    pub subscription_periods: Option<Vec<SubscriptionPeriodTile>>,
}

/// Percentage of friends of one gender.
/// <https://developers.line.biz/en/reference/messaging-api/#get-demographic>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GenderTile {
    /// Users' gender.
    pub gender: Option<Gender>,
    /// Percentage.
    pub percentage: Option<f64>,
}

/// Users' gender.
///
/// `Unknown` is both the spec value `unknown` and the fallback for values added later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Gender {
    /// `male`.
    Male,
    /// `female`.
    Female,
    /// `unknown`, or a value not known to this crate.
    #[serde(other)]
    Unknown,
}

/// Percentage of friends in one age group.
/// <https://developers.line.biz/en/reference/messaging-api/#get-demographic>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AgeTile {
    /// Users' age group.
    pub age: Option<AgeGroup>,
    /// Percentage.
    pub percentage: Option<f64>,
}

/// Users' age group.
///
/// `Unknown` is both the spec value `unknown` and the fallback for values added later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgeGroup {
    /// `from0to14`: 0 to 14 years old.
    #[serde(rename = "from0to14")]
    From0To14,
    /// `from15to19`: 15 to 19 years old.
    #[serde(rename = "from15to19")]
    From15To19,
    /// `from20to24`: 20 to 24 years old.
    #[serde(rename = "from20to24")]
    From20To24,
    /// `from25to29`: 25 to 29 years old.
    #[serde(rename = "from25to29")]
    From25To29,
    /// `from30to34`: 30 to 34 years old.
    #[serde(rename = "from30to34")]
    From30To34,
    /// `from35to39`: 35 to 39 years old.
    #[serde(rename = "from35to39")]
    From35To39,
    /// `from40to44`: 40 to 44 years old.
    #[serde(rename = "from40to44")]
    From40To44,
    /// `from45to49`: 45 to 49 years old.
    #[serde(rename = "from45to49")]
    From45To49,
    /// `from50`: 50 years old or older.
    #[serde(rename = "from50")]
    From50,
    /// `from50to54`: 50 to 54 years old.
    #[serde(rename = "from50to54")]
    From50To54,
    /// `from55to59`: 55 to 59 years old.
    #[serde(rename = "from55to59")]
    From55To59,
    /// `from60to64`: 60 to 64 years old.
    #[serde(rename = "from60to64")]
    From60To64,
    /// `from65to69`: 65 to 69 years old.
    #[serde(rename = "from65to69")]
    From65To69,
    /// `from70`: 70 years old or older.
    #[serde(rename = "from70")]
    From70,
    /// `unknown`, or a value not known to this crate.
    #[serde(rename = "unknown", other)]
    Unknown,
}

/// Percentage of friends in one area.
/// <https://developers.line.biz/en/reference/messaging-api/#get-demographic>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AreaTile {
    /// Users' country and region.
    pub area: Option<String>,
    /// Percentage.
    pub percentage: Option<f64>,
}

/// Percentage of friends using one OS.
/// <https://developers.line.biz/en/reference/messaging-api/#get-demographic>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppTypeTile {
    /// Users' OS.
    pub app_type: Option<AppType>,
    /// Percentage.
    pub percentage: Option<f64>,
}

/// Users' OS.
///
/// `Unknown` is the fallback for values added later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AppType {
    /// `ios`.
    Ios,
    /// `android`.
    Android,
    /// `others`.
    Others,
    /// A value not known to this crate.
    #[serde(other)]
    Unknown,
}

/// Percentage of friends per friendship duration.
/// <https://developers.line.biz/en/reference/messaging-api/#get-demographic>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionPeriodTile {
    /// Subscription period.
    pub subscription_period: Option<SubscriptionPeriod>,
    /// Percentage, in the range `[0.0, 100.0]`.
    pub percentage: Option<f64>,
}

/// Subscription period (friendship duration).
///
/// `Unknown` is both the spec value `unknown` and the fallback for values added later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SubscriptionPeriod {
    /// `within7days`: less than 7 days.
    #[serde(rename = "within7days")]
    Within7Days,
    /// `within30days`: 7 days or more but less than 30 days.
    #[serde(rename = "within30days")]
    Within30Days,
    /// `within90days`: 30 days or more but less than 90 days.
    #[serde(rename = "within90days")]
    Within90Days,
    /// `within180days`: 90 days or more but less than 180 days.
    #[serde(rename = "within180days")]
    Within180Days,
    /// `within365days`: 180 days or more but less than 365 days.
    #[serde(rename = "within365days")]
    Within365Days,
    /// `over365days`: 365 days or more.
    #[serde(rename = "over365days")]
    Over365Days,
    /// `unknown`, or a value not known to this crate.
    #[serde(other)]
    Unknown,
}

/// Calculation status of an insight statistic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InsightStatus {
    /// `ready`.
    Ready,
    /// `unready`.
    Unready,
    /// `out_of_service`.
    OutOfService,
    /// A value not known to this crate.
    #[serde(other)]
    Unknown,
}

/// Number of messages sent on a date.
/// <https://developers.line.biz/en/reference/messaging-api/#get-number-of-delivery-messages>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNumberOfMessageDeliveriesResponse {
    /// Status of the counting process.
    pub status: Option<InsightStatus>,
    /// Number of messages sent to all of this LINE Official Account's friends (broadcast messages).
    pub broadcast: Option<u64>,
    /// Number of messages sent to some of this LINE Official Account's friends, based on specific attributes (targeted messages).
    pub targeting: Option<u64>,
    /// Number of auto-response messages sent.
    pub auto_response: Option<u64>,
    /// Number of greeting messages sent.
    pub welcome_response: Option<u64>,
    /// Number of messages sent from the LINE Official Account Manager Chat screen.
    pub chat: Option<u64>,
    /// Number of broadcast messages sent with the `Send broadcast message` operation.
    pub api_broadcast: Option<u64>,
    /// Number of push messages sent with the `Send push message` operation.
    pub api_push: Option<u64>,
    /// Number of multicast messages sent with the `Send multicast message` operation.
    pub api_multicast: Option<u64>,
    /// Number of narrowcast messages sent with the `Send narrowcast message` operation.
    pub api_narrowcast: Option<u64>,
    /// Number of replies sent with the `Send reply message` operation.
    pub api_reply: Option<u64>,
}

/// Number of followers as of a date.
/// <https://developers.line.biz/en/reference/messaging-api/#get-number-of-followers>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNumberOfFollowersResponse {
    /// Calculation status.
    pub status: Option<InsightStatus>,
    /// Number of times, as of the date, that a user added this account as a friend for the first time.
    ///
    /// It doesn't decrease even if a user later blocks the account or deletes their LINE account.
    pub followers: Option<u64>,
    /// Number of users, as of the date, reachable through targeted messages based on gender, age, and/or region.
    ///
    /// Only includes users who are active on LINE or LINE services and whose demographics have a high level of certainty.
    pub targeted_reaches: Option<u64>,
    /// Number of users blocking the account as of the date; decreases when a user unblocks the account.
    pub blocks: Option<u64>,
}

/// Statistics about how users interact with narrowcast or broadcast messages sent from the LINE Official Account.
/// <https://developers.line.biz/en/reference/messaging-api/#get-insight-message-event-response>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct GetMessageEventResponse {
    /// Summary of message statistics.
    pub overview: Option<GetMessageEventResponseOverview>,
    /// Information about individual message bubbles.
    pub messages: Option<Vec<GetMessageEventResponseMessage>>,
    /// Information about opened URLs in the message.
    pub clicks: Option<Vec<GetMessageEventResponseClick>>,
}

/// Summary of message statistics.
/// <https://developers.line.biz/en/reference/messaging-api/#get-insight-message-event-response>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMessageEventResponseOverview {
    /// Request ID.
    pub request_id: Option<String>,
    /// UNIX timestamp of the message delivery time, in seconds.
    pub timestamp: Option<i64>,
    /// Number of messages delivered; `None` if not all messages have been sent.
    pub delivered: Option<u64>,
    /// Number of users who opened the message, meaning they displayed at least 1 bubble.
    pub unique_impression: Option<u64>,
    /// Number of users who opened any URL in the message.
    pub unique_click: Option<u64>,
    /// Number of users who started playing any video or audio in the message.
    pub unique_media_played: Option<u64>,
    /// Number of users who played the entirety of any video or audio in the message.
    pub unique_media_played100_percent: Option<u64>,
}

/// Statistics of a message bubble.
/// <https://developers.line.biz/en/reference/messaging-api/#get-insight-message-event-response>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMessageEventResponseMessage {
    /// Bubble's serial number.
    pub seq: Option<u32>,
    /// Number of times the bubble was displayed.
    pub impression: Option<u64>,
    /// Number of times audio or video in the bubble started playing.
    pub media_played: Option<u64>,
    /// Number of times audio or video in the bubble started playing and was played 25% of the total time.
    pub media_played25_percent: Option<u64>,
    /// Number of times audio or video in the bubble started playing and was played 50% of the total time.
    pub media_played50_percent: Option<u64>,
    /// Number of times audio or video in the bubble started playing and was played 75% of the total time.
    pub media_played75_percent: Option<u64>,
    /// Number of times audio or video in the bubble started playing and was played 100% of the total time.
    pub media_played100_percent: Option<u64>,
    /// Number of users that started playing audio or video in the bubble.
    pub unique_media_played: Option<u64>,
    /// Number of users that started playing audio or video in the bubble and played 25% of the total time.
    pub unique_media_played25_percent: Option<u64>,
    /// Number of users that started playing audio or video in the bubble and played 50% of the total time.
    pub unique_media_played50_percent: Option<u64>,
    /// Number of users that started playing audio or video in the bubble and played 75% of the total time.
    pub unique_media_played75_percent: Option<u64>,
    /// Number of users that started playing audio or video in the bubble and played 100% of the total time.
    pub unique_media_played100_percent: Option<u64>,
}

/// Statistics of a URL opened from the message.
/// <https://developers.line.biz/en/reference/messaging-api/#get-insight-message-event-response>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMessageEventResponseClick {
    /// The URL's serial number.
    pub seq: Option<u32>,
    /// URL.
    pub url: Option<String>,
    /// Number of times the URL was opened.
    pub click: Option<u64>,
    /// Number of users that opened the URL.
    pub unique_click: Option<u64>,
    /// Number of users who opened this URL through any link in the message.
    ///
    /// A user who opens two links to the same URL in the message is counted only once.
    pub unique_click_of_request: Option<u64>,
}

/// Statistics per aggregation unit.
/// <https://developers.line.biz/en/reference/messaging-api/#get-statistics-per-unit-response>
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct GetStatisticsPerUnitResponse {
    /// Statistics related to messages.
    pub overview: GetStatisticsPerUnitResponseOverview,
    /// Information about individual message bubbles.
    pub messages: Vec<GetStatisticsPerUnitResponseMessage>,
    /// Information about opened URLs in the message.
    pub clicks: Vec<GetStatisticsPerUnitResponseClick>,
}

/// Statistics related to messages.
/// <https://developers.line.biz/en/reference/messaging-api/#get-statistics-per-unit-response>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStatisticsPerUnitResponseOverview {
    /// Number of users who opened the message, meaning they displayed at least 1 bubble.
    pub unique_impression: Option<u64>,
    /// Number of users who opened any URL in the message.
    pub unique_click: Option<u64>,
    /// Number of users who started playing any video or audio in the message.
    pub unique_media_played: Option<u64>,
    /// Number of users who played the entirety of any video or audio in the message.
    pub unique_media_played100_percent: Option<u64>,
}

/// Statistics of a message bubble.
/// <https://developers.line.biz/en/reference/messaging-api/#get-statistics-per-unit-response>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStatisticsPerUnitResponseMessage {
    /// Bubble's serial number.
    pub seq: u32,
    /// Number of times the bubble was displayed.
    pub impression: Option<u64>,
    /// Number of times audio or video in the bubble started playing.
    pub media_played: Option<u64>,
    /// Number of times audio or video in the bubble started playing and was played 25% of the total time.
    pub media_played25_percent: Option<u64>,
    /// Number of times audio or video in the bubble started playing and was played 50% of the total time.
    pub media_played50_percent: Option<u64>,
    /// Number of times audio or video in the bubble started playing and was played 75% of the total time.
    pub media_played75_percent: Option<u64>,
    /// Number of times audio or video in the bubble started playing and was played 100% of the total time.
    pub media_played100_percent: Option<u64>,
    /// Number of users the bubble was displayed to.
    pub unique_impression: Option<u64>,
    /// Number of users that started playing audio or video in the bubble.
    pub unique_media_played: Option<u64>,
    /// Number of users that started playing audio or video in the bubble and played 25% of the total time.
    pub unique_media_played25_percent: Option<u64>,
    /// Number of users that started playing audio or video in the bubble and played 50% of the total time.
    pub unique_media_played50_percent: Option<u64>,
    /// Number of users that started playing audio or video in the bubble and played 75% of the total time.
    pub unique_media_played75_percent: Option<u64>,
    /// Number of users that started playing audio or video in the bubble and played 100% of the total time.
    pub unique_media_played100_percent: Option<u64>,
}

/// Statistics of a URL opened from the message.
/// <https://developers.line.biz/en/reference/messaging-api/#get-statistics-per-unit-response>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStatisticsPerUnitResponseClick {
    /// The URL's serial number.
    pub seq: u64,
    /// URL.
    pub url: String,
    /// Number of times the URL in the bubble was opened.
    pub click: Option<u64>,
    /// Number of users that opened the URL in the bubble.
    pub unique_click: Option<u64>,
    /// Number of users who opened this URL through any link in the message.
    ///
    /// A user who opens the same URL in another bubble too is counted only once.
    pub unique_click_of_request: Option<u64>,
}

/// Summary of impression and click statistics for a rich menu created via the Messaging API.
/// <https://developers.line.biz/en/reference/messaging-api/#get-rich-menu-insight-summary>
///
/// Only `rich_menu_id` is set when the number of unique clicks is below the privacy threshold.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRichMenuInsightSummaryResponse {
    /// Rich menu ID.
    pub rich_menu_id: String,
    /// Start date (JST, `yyyyMMdd`) of the period actually covered.
    pub metrics_from: Option<String>,
    /// End date (JST, `yyyyMMdd`) of the period actually covered.
    pub metrics_to: Option<String>,
    /// Impression metrics for the whole rich menu.
    pub impression: Option<GetRichMenuInsightSummaryResponseImpression>,
    /// Click metrics for each tappable area of the rich menu.
    pub clicks: Option<Vec<GetRichMenuInsightSummaryResponseClick>>,
}

/// Impression metrics for the whole rich menu.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct GetRichMenuInsightSummaryResponseImpression {
    /// Aggregated impressions over the whole period.
    pub metrics: GetRichMenuInsightSummaryResponseMetrics,
}

/// Click metrics for one tappable area.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct GetRichMenuInsightSummaryResponseClick {
    /// Target area of the metrics.
    pub bounds: RichMenuInsightBounds,
    /// Aggregated clicks on the area over the whole period.
    pub metrics: GetRichMenuInsightSummaryResponseMetrics,
}

/// Number of events and unique users over the whole period.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRichMenuInsightSummaryResponseMetrics {
    /// Number of impressions or clicks.
    pub count: u64,
    /// Approximate number of unique users who triggered an impression or click.
    pub unique_users: u64,
}

/// Target area of rich menu insight metrics, given by its top-left coordinate, width, and height
/// (`GetRichMenuInsightSummaryResponseBounds` / `GetRichMenuInsightDailyResponseBounds`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RichMenuInsightBounds {
    /// The x coordinate of the top-left corner of the target area.
    pub x: i32,
    /// The y coordinate of the top-left corner of the target area.
    pub y: i32,
    /// The width of the target area.
    pub width: i32,
    /// The height of the target area.
    pub height: i32,
}

/// Daily impression and click statistics for a rich menu created via the Messaging API.
/// <https://developers.line.biz/en/reference/messaging-api/#get-rich-menu-insight-daily>
///
/// Only `rich_menu_id` is set when the number of unique clicks is below the privacy threshold.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRichMenuInsightDailyResponse {
    /// Rich menu ID.
    pub rich_menu_id: String,
    /// Start date (JST, `yyyyMMdd`) of the period actually covered.
    pub metrics_from: Option<String>,
    /// End date (JST, `yyyyMMdd`) of the period actually covered.
    pub metrics_to: Option<String>,
    /// Daily impression metrics for the whole rich menu.
    pub impression: Option<GetRichMenuInsightDailyResponseImpression>,
    /// Daily click metrics for each tappable area of the rich menu.
    pub clicks: Option<Vec<GetRichMenuInsightDailyResponseClick>>,
}

/// Daily impression metrics for the whole rich menu.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct GetRichMenuInsightDailyResponseImpression {
    /// Per-day impression metrics.
    pub metrics: Vec<GetRichMenuInsightDailyResponseDailyMetrics>,
}

/// Daily click metrics for one tappable area.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct GetRichMenuInsightDailyResponseClick {
    /// Target area of the metrics.
    pub bounds: RichMenuInsightBounds,
    /// Per-day click metrics for the target area.
    pub metrics: Vec<GetRichMenuInsightDailyResponseDailyMetrics>,
}

/// Number of events and unique users on one day.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRichMenuInsightDailyResponseDailyMetrics {
    /// Date of these metrics (JST, `yyyyMMdd`).
    pub date: String,
    /// Number of impressions or clicks on this day.
    pub count: u64,
    /// Approximate number of unique users who triggered an impression or click on this day.
    pub unique_users: u64,
}
