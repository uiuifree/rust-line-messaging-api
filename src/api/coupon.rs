use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::client::{Host, LineClient};
use crate::error::Result;

impl LineClient {
    /// Gets the list of coupons.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-coupons-list>
    pub async fn list_coupon(
        &self,
        params: &ListCouponParams,
    ) -> Result<MessagingApiPagerCouponListResponse> {
        let mut rb = self.request(Method::GET, Host::Api, &["v2", "bot", "coupon"]);
        for status in params.status.iter().flatten() {
            rb = rb.query(&[("status", status)]);
        }
        let rb = rb
            .query(&[("start", params.start.as_deref())])
            .query(&[("limit", params.limit)]);
        self.send_json(rb).await
    }

    /// Creates a coupon.
    /// <https://developers.line.biz/en/reference/messaging-api/#create-coupon>
    pub async fn create_coupon(
        &self,
        request: &CouponCreateRequest,
    ) -> Result<CouponCreateResponse> {
        let rb = self.request(Method::POST, Host::Api, &["v2", "bot", "coupon"]);
        self.send_json(rb.json(request)).await
    }

    /// Gets the details of a coupon.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-coupon>
    pub async fn get_coupon_detail(&self, coupon_id: &str) -> Result<CouponResponse> {
        let rb = self.request(Method::GET, Host::Api, &["v2", "bot", "coupon", coupon_id]);
        self.send_json(rb).await
    }

    /// Discontinues a coupon.
    /// <https://developers.line.biz/en/reference/messaging-api/#discontinue-coupon>
    pub async fn close_coupon(&self, coupon_id: &str) -> Result<()> {
        let rb = self.request(
            Method::PUT,
            Host::Api,
            &["v2", "bot", "coupon", coupon_id, "close"],
        );
        self.send_empty(rb).await
    }
}

/// Query parameters of [`LineClient::list_coupon`].
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ListCouponParams {
    /// Filters coupons by status.
    pub status: Option<Vec<CouponStatus>>,
    /// Pagination token (`next` of the previous page).
    pub start: Option<String>,
    /// Coupons per page (1-100, default 20).
    pub limit: Option<u32>,
}

impl ListCouponParams {
    /// Sets the coupon statuses to filter by.
    pub fn status(mut self, status: impl IntoIterator<Item = CouponStatus>) -> Self {
        self.status = Some(status.into_iter().collect());
        self
    }

    /// Sets the pagination token (`next` of the previous page).
    pub fn start(mut self, start: impl Into<String>) -> Self {
        self.start = Some(start.into());
        self
    }

    /// Sets the number of coupons per page (1-100, default 20).
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Paginated list of coupons.
/// <https://developers.line.biz/en/reference/messaging-api/#get-coupons-list>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MessagingApiPagerCouponListResponse {
    /// Coupon summaries.
    pub items: Vec<CouponListResponse>,
    /// Token for fetching the next page.
    pub next: Option<String>,
}

/// Summary of a coupon in a list.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CouponListResponse {
    /// Unique identifier of the coupon.
    pub coupon_id: String,
    /// Title of the coupon.
    pub title: String,
}

/// Request body to create a coupon.
/// <https://developers.line.biz/en/reference/messaging-api/#create-coupon>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CouponCreateRequest {
    /// How the coupon is distributed.
    pub acquisition_condition: AcquisitionConditionRequest,
    /// URL of the barcode image used for in-store redemption.
    pub barcode_image_url: Option<String>,
    /// Code presented by the user to redeem the coupon.
    pub coupon_code: Option<String>,
    /// Description displayed to users (max 1000 characters).
    pub description: Option<String>,
    /// Expiration time (epoch seconds).
    pub end_timestamp: i64,
    /// URL of the main image displayed in the coupon list.
    pub image_url: Option<String>,
    /// Maximum uses per coupon ticket; `-1` for no limit.
    pub max_use_count_per_ticket: i32,
    /// Start time (epoch seconds).
    pub start_timestamp: i64,
    /// Title displayed in the coupon list (1-60 characters).
    pub title: String,
    /// Conditions for using the coupon, shown to users (max 100 characters).
    pub usage_condition: Option<String>,
    /// Benefit provided by the coupon.
    pub reward: Option<CouponRewardRequest>,
    /// Who can see or acquire the coupon.
    pub visibility: CouponCreateVisibility,
    /// Timezone for interpreting `start_timestamp` and `end_timestamp`.
    pub timezone: CouponTimezone,
}

impl CouponCreateRequest {
    /// Creates a request with the required properties.
    pub fn new(
        acquisition_condition: impl Into<AcquisitionConditionRequest>,
        start_timestamp: i64,
        end_timestamp: i64,
        max_use_count_per_ticket: i32,
        title: impl Into<String>,
        visibility: CouponCreateVisibility,
        timezone: CouponTimezone,
    ) -> Self {
        Self {
            acquisition_condition: acquisition_condition.into(),
            barcode_image_url: None,
            coupon_code: None,
            description: None,
            end_timestamp,
            image_url: None,
            max_use_count_per_ticket,
            start_timestamp,
            title: title.into(),
            usage_condition: None,
            reward: None,
            visibility,
            timezone,
        }
    }

    /// Sets the URL of the barcode image used for in-store redemption.
    pub fn barcode_image_url(mut self, barcode_image_url: impl Into<String>) -> Self {
        self.barcode_image_url = Some(barcode_image_url.into());
        self
    }

    /// Sets the code presented by the user to redeem the coupon.
    pub fn coupon_code(mut self, coupon_code: impl Into<String>) -> Self {
        self.coupon_code = Some(coupon_code.into());
        self
    }

    /// Sets the description displayed to users (max 1000 characters).
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the URL of the main image displayed in the coupon list.
    pub fn image_url(mut self, image_url: impl Into<String>) -> Self {
        self.image_url = Some(image_url.into());
        self
    }

    /// Sets the conditions for using the coupon, shown to users (max 100 characters).
    pub fn usage_condition(mut self, usage_condition: impl Into<String>) -> Self {
        self.usage_condition = Some(usage_condition.into());
        self
    }

    /// Sets the benefit provided by the coupon.
    pub fn reward(mut self, reward: impl Into<CouponRewardRequest>) -> Self {
        self.reward = Some(reward.into());
        self
    }
}

/// Response of creating a coupon.
/// <https://developers.line.biz/en/reference/messaging-api/#create-coupon>
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CouponCreateResponse {
    /// Unique identifier of the coupon.
    pub coupon_id: String,
}

/// Details of a coupon, including its current status.
/// <https://developers.line.biz/en/reference/messaging-api/#get-coupon>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CouponResponse {
    /// How the coupon is distributed.
    pub acquisition_condition: Option<AcquisitionConditionResponse>,
    /// URL of the barcode image used for in-store redemption.
    pub barcode_image_url: Option<String>,
    /// Code presented by the user to redeem the coupon.
    pub coupon_code: Option<String>,
    /// Description displayed to users.
    pub description: Option<String>,
    /// Expiration time (epoch seconds).
    pub end_timestamp: Option<i64>,
    /// URL of the main image displayed in the coupon list.
    pub image_url: Option<String>,
    /// Maximum number of coupons that can be issued in total.
    pub max_acquire_count: Option<i64>,
    /// Maximum number of times a single coupon ticket can be used.
    pub max_use_count_per_ticket: Option<i32>,
    /// Maximum number of coupon tickets a single user can acquire.
    pub max_ticket_per_user: Option<i64>,
    /// Start time (epoch seconds).
    pub start_timestamp: Option<i64>,
    /// Title displayed in the coupon list.
    pub title: Option<String>,
    /// Conditions for using the coupon, shown to users.
    pub usage_condition: Option<String>,
    /// Benefit provided by the coupon.
    pub reward: Option<CouponRewardResponse>,
    /// Who can see or acquire the coupon.
    pub visibility: Option<CouponVisibility>,
    /// Timezone for interpreting `start_timestamp` and `end_timestamp`.
    pub timezone: Option<CouponTimezone>,
    /// Unique identifier of the coupon.
    pub coupon_id: Option<String>,
    /// Creation time (epoch seconds).
    pub created_timestamp: Option<i64>,
    /// Current status of the coupon.
    pub status: Option<CouponStatus>,
}

/// Status of a coupon.
///
/// `Unknown` is the fallback for values added later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CouponStatus {
    /// `DRAFT`.
    Draft,
    /// `RUNNING`.
    Running,
    /// `CLOSED`.
    Closed,
    /// A value not known to this crate.
    #[serde(other)]
    Unknown,
}

/// Visibility accepted by [`LineClient::create_coupon`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CouponCreateVisibility {
    /// `UNLISTED`.
    Unlisted,
    /// `PUBLIC`.
    Public,
}

/// Visibility of an existing coupon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CouponVisibility {
    /// `UNLISTED`.
    Unlisted,
    /// `PUBLIC`.
    Public,
    /// `PRIVATE`.
    Private,
    /// A value not known to this crate.
    #[serde(other)]
    Unknown,
}

/// Timezone for interpreting coupon start and end timestamps.
///
/// `Unknown` is the fallback for values added later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CouponTimezone {
    /// `ETC_GMT_MINUS_12`.
    #[serde(rename = "ETC_GMT_MINUS_12")]
    EtcGmtMinus12,
    /// `ETC_GMT_MINUS_11`.
    #[serde(rename = "ETC_GMT_MINUS_11")]
    EtcGmtMinus11,
    /// `PACIFIC_HONOLULU`.
    PacificHonolulu,
    /// `AMERICA_ANCHORAGE`.
    AmericaAnchorage,
    /// `AMERICA_LOS_ANGELES`.
    AmericaLosAngeles,
    /// `AMERICA_PHOENIX`.
    AmericaPhoenix,
    /// `AMERICA_CHICAGO`.
    AmericaChicago,
    /// `AMERICA_NEW_YORK`.
    AmericaNewYork,
    /// `AMERICA_CARACAS`.
    AmericaCaracas,
    /// `AMERICA_SANTIAGO`.
    AmericaSantiago,
    /// `AMERICA_ST_JOHNS`.
    AmericaStJohns,
    /// `AMERICA_SAO_PAULO`.
    AmericaSaoPaulo,
    /// `ETC_GMT_MINUS_2`.
    #[serde(rename = "ETC_GMT_MINUS_2")]
    EtcGmtMinus2,
    /// `ATLANTIC_CAPE_VERDE`.
    AtlanticCapeVerde,
    /// `EUROPE_LONDON`.
    EuropeLondon,
    /// `EUROPE_PARIS`.
    EuropeParis,
    /// `EUROPE_ISTANBUL`.
    EuropeIstanbul,
    /// `EUROPE_MOSCOW`.
    EuropeMoscow,
    /// `ASIA_TEHRAN`.
    AsiaTehran,
    /// `ASIA_TBILISI`.
    AsiaTbilisi,
    /// `ASIA_KABUL`.
    AsiaKabul,
    /// `ASIA_TASHKENT`.
    AsiaTashkent,
    /// `ASIA_COLOMBO`.
    AsiaColombo,
    /// `ASIA_KATHMANDU`.
    AsiaKathmandu,
    /// `ASIA_ALMATY`.
    AsiaAlmaty,
    /// `ASIA_RANGOON`.
    AsiaRangoon,
    /// `ASIA_BANGKOK`.
    AsiaBangkok,
    /// `ASIA_TAIPEI`.
    AsiaTaipei,
    /// `ASIA_TOKYO`.
    AsiaTokyo,
    /// `AUSTRALIA_DARWIN`.
    AustraliaDarwin,
    /// `AUSTRALIA_SYDNEY`.
    AustraliaSydney,
    /// `ASIA_VLADIVOSTOK`.
    AsiaVladivostok,
    /// `ETC_GMT_PLUS_12`.
    #[serde(rename = "ETC_GMT_PLUS_12")]
    EtcGmtPlus12,
    /// `PACIFIC_TONGATAPU`.
    PacificTongatapu,
    /// A value not known to this crate.
    #[serde(other)]
    Unknown,
}

/// Currency code.
///
/// `Unknown` is the fallback for values added later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CouponCurrency {
    /// `JPY`.
    Jpy,
    /// `THB`.
    Thb,
    /// `TWD`.
    Twd,
    /// A value not known to this crate.
    #[serde(other)]
    Unknown,
}

/// How the coupon is distributed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum AcquisitionConditionRequest {
    /// `normal`.
    Normal,
    /// `lottery`.
    Lottery(LotteryAcquisitionConditionRequest),
}

/// Lottery acquisition condition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LotteryAcquisitionConditionRequest {
    /// Probability (1-99) of winning the coupon.
    pub lottery_probability: u32,
    /// Maximum number of coupons issued in total (max 999999); `-1` for no limit.
    pub max_acquire_count: i32,
}

impl LotteryAcquisitionConditionRequest {
    /// Creates a lottery condition with the winning probability (1-99) and the total issue limit (`-1` for no limit).
    pub fn new(lottery_probability: u32, max_acquire_count: i32) -> Self {
        Self {
            lottery_probability,
            max_acquire_count,
        }
    }
}

impl From<LotteryAcquisitionConditionRequest> for AcquisitionConditionRequest {
    fn from(value: LotteryAcquisitionConditionRequest) -> Self {
        Self::Lottery(value)
    }
}

/// Benefit provided by the coupon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum CouponRewardRequest {
    /// `cashBack`.
    CashBack(CouponCashBackRewardRequest),
    /// `discount`.
    Discount(CouponDiscountRewardRequest),
    /// `free`.
    Free,
    /// `gift`.
    Gift,
    /// `others`.
    Others,
}

/// Cashback reward.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CouponCashBackRewardRequest {
    /// Cashback amount or rate.
    pub price_info: Option<CashBackPriceInfoRequest>,
}

impl CouponCashBackRewardRequest {
    /// Sets the cashback amount or rate.
    pub fn price_info(mut self, price_info: impl Into<CashBackPriceInfoRequest>) -> Self {
        self.price_info = Some(price_info.into());
        self
    }
}

impl From<CouponCashBackRewardRequest> for CouponRewardRequest {
    fn from(value: CouponCashBackRewardRequest) -> Self {
        Self::CashBack(value)
    }
}

/// Discount reward.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CouponDiscountRewardRequest {
    /// Discount amount, rate, or explicit prices.
    pub price_info: Option<DiscountPriceInfoRequest>,
}

impl CouponDiscountRewardRequest {
    /// Sets the discount amount, rate, or explicit prices.
    pub fn price_info(mut self, price_info: impl Into<DiscountPriceInfoRequest>) -> Self {
        self.price_info = Some(price_info.into());
        self
    }
}

impl From<CouponDiscountRewardRequest> for CouponRewardRequest {
    fn from(value: CouponDiscountRewardRequest) -> Self {
        Self::Discount(value)
    }
}

/// Cashback amount or rate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum CashBackPriceInfoRequest {
    /// `fixed`.
    Fixed(CashBackFixedPriceInfoRequest),
    /// `percentage`.
    Percentage(CashBackPercentagePriceInfoRequest),
}

/// Fixed cashback amount.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashBackFixedPriceInfoRequest {
    /// Fixed cashback amount.
    pub fixed_amount: Option<i64>,
}

impl CashBackFixedPriceInfoRequest {
    /// Sets the fixed cashback amount.
    pub fn fixed_amount(mut self, fixed_amount: i64) -> Self {
        self.fixed_amount = Some(fixed_amount);
        self
    }
}

impl From<CashBackFixedPriceInfoRequest> for CashBackPriceInfoRequest {
    fn from(value: CashBackFixedPriceInfoRequest) -> Self {
        Self::Fixed(value)
    }
}

/// Cashback rate as a percentage.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CashBackPercentagePriceInfoRequest {
    /// Cashback rate (1-99).
    pub percentage: Option<u32>,
}

impl CashBackPercentagePriceInfoRequest {
    /// Sets the cashback rate (1-99).
    pub fn percentage(mut self, percentage: u32) -> Self {
        self.percentage = Some(percentage);
        self
    }
}

impl From<CashBackPercentagePriceInfoRequest> for CashBackPriceInfoRequest {
    fn from(value: CashBackPercentagePriceInfoRequest) -> Self {
        Self::Percentage(value)
    }
}

/// Discount amount, rate, or explicit prices.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DiscountPriceInfoRequest {
    /// `fixed`.
    Fixed(DiscountFixedPriceInfoRequest),
    /// `percentage`.
    Percentage(DiscountPercentagePriceInfoRequest),
    /// `explicit`.
    Explicit(DiscountExplicitPriceInfoRequest),
}

/// Fixed discount amount.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscountFixedPriceInfoRequest {
    /// Fixed discount amount.
    pub fixed_amount: Option<i64>,
}

impl DiscountFixedPriceInfoRequest {
    /// Sets the fixed discount amount.
    pub fn fixed_amount(mut self, fixed_amount: i64) -> Self {
        self.fixed_amount = Some(fixed_amount);
        self
    }
}

impl From<DiscountFixedPriceInfoRequest> for DiscountPriceInfoRequest {
    fn from(value: DiscountFixedPriceInfoRequest) -> Self {
        Self::Fixed(value)
    }
}

/// Discount rate as a percentage.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DiscountPercentagePriceInfoRequest {
    /// Discount rate (1-99).
    pub percentage: Option<u32>,
}

impl DiscountPercentagePriceInfoRequest {
    /// Sets the discount rate (1-99).
    pub fn percentage(mut self, percentage: u32) -> Self {
        self.percentage = Some(percentage);
        self
    }
}

impl From<DiscountPercentagePriceInfoRequest> for DiscountPriceInfoRequest {
    fn from(value: DiscountPercentagePriceInfoRequest) -> Self {
        Self::Percentage(value)
    }
}

/// Explicit original and discounted prices.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscountExplicitPriceInfoRequest {
    /// Price after the discount.
    pub price_after_discount: Option<i64>,
    /// Original price.
    pub original_price: Option<i64>,
}

impl DiscountExplicitPriceInfoRequest {
    /// Sets the price after the discount.
    pub fn price_after_discount(mut self, price_after_discount: i64) -> Self {
        self.price_after_discount = Some(price_after_discount);
        self
    }

    /// Sets the original price.
    pub fn original_price(mut self, original_price: i64) -> Self {
        self.original_price = Some(original_price);
        self
    }
}

impl From<DiscountExplicitPriceInfoRequest> for DiscountPriceInfoRequest {
    fn from(value: DiscountExplicitPriceInfoRequest) -> Self {
        Self::Explicit(value)
    }
}

tagged_enum! {
    /// How the coupon is distributed.
    pub enum AcquisitionConditionResponse {
        "normal" => Normal,
        "lottery" => Lottery(LotteryAcquisitionConditionResponse),
        "referral" => Referral,
    }
}

/// Lottery acquisition condition.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "lottery", rename_all = "camelCase")]
pub struct LotteryAcquisitionConditionResponse {
    /// Probability of winning the coupon.
    pub lottery_probability: Option<i32>,
    /// Maximum number of coupons that can be issued in total.
    pub max_acquire_count: Option<i32>,
}

tagged_enum! {
    /// Benefit provided by the coupon.
    pub enum CouponRewardResponse {
        "cashBack" => CashBack(CouponCashBackRewardResponse),
        "discount" => Discount(CouponDiscountRewardResponse),
        "free" => Free,
        "gift" => Gift,
        "others" => Others,
    }
}

/// Cashback reward.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "cashBack", rename_all = "camelCase")]
pub struct CouponCashBackRewardResponse {
    /// Amount or rate of the reward.
    pub price_info: Option<CashBackPriceInfoResponse>,
}

/// Discount reward.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "discount", rename_all = "camelCase")]
pub struct CouponDiscountRewardResponse {
    /// Amount or rate of the reward.
    pub price_info: Option<DiscountPriceInfoResponse>,
}

tagged_enum! {
    /// Cashback amount or rate.
    pub enum CashBackPriceInfoResponse {
        "fixed" => Fixed(CashBackFixedPriceInfoResponse),
        "percentage" => Percentage(CashBackPercentagePriceInfoResponse),
    }
}

/// Fixed cashback amount.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "fixed", rename_all = "camelCase")]
pub struct CashBackFixedPriceInfoResponse {
    /// Currency code.
    pub currency: Option<CouponCurrency>,
    /// Fixed amount.
    pub fixed_amount: Option<i64>,
}

/// Cashback rate as a percentage.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "percentage")]
pub struct CashBackPercentagePriceInfoResponse {
    /// Cashback rate as a percentage.
    pub percentage: Option<i32>,
}

tagged_enum! {
    /// Discount amount, rate, or explicit prices.
    pub enum DiscountPriceInfoResponse {
        "fixed" => Fixed(DiscountFixedPriceInfoResponse),
        "percentage" => Percentage(DiscountPercentagePriceInfoResponse),
        "explicit" => Explicit(DiscountExplicitPriceInfoResponse),
    }
}

/// Fixed discount amount.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "fixed", rename_all = "camelCase")]
pub struct DiscountFixedPriceInfoResponse {
    /// Currency code.
    pub currency: Option<CouponCurrency>,
    /// Fixed amount.
    pub fixed_amount: Option<i64>,
}

/// Discount rate as a percentage.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "percentage")]
pub struct DiscountPercentagePriceInfoResponse {
    /// Discount rate as a percentage.
    pub percentage: Option<i32>,
}

/// Explicit original and discounted prices.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "explicit", rename_all = "camelCase")]
pub struct DiscountExplicitPriceInfoResponse {
    /// Currency code.
    pub currency: Option<CouponCurrency>,
    /// Price after the discount.
    pub price_after_discount: Option<i64>,
    /// Original price.
    pub original_price: Option<i64>,
}
