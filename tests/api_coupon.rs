mod common;

use common::{TOKEN, setup};
use line_bot_messaging_api::Error;
use line_bot_messaging_api::api::*;
use serde_json::{Value, json};
use wiremock::matchers::{bearer_token, body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn single_query(server: &MockServer) -> Option<String> {
    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 1);
    requests[0].url.query().map(str::to_owned)
}

#[tokio::test]
async fn list_coupon_with_all_params() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/coupon"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [
                {"couponId": "coupon-1", "title": "10% off"},
                {"couponId": "coupon-2", "title": "Free drink"}
            ],
            "next": "token-2"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let params = ListCouponParams::default()
        .status([CouponStatus::Draft, CouponStatus::Running])
        .start("token-1")
        .limit(50);
    let res = client.list_coupon(&params).await.unwrap();
    assert_eq!(
        res,
        MessagingApiPagerCouponListResponse {
            items: vec![
                CouponListResponse {
                    coupon_id: "coupon-1".into(),
                    title: "10% off".into(),
                },
                CouponListResponse {
                    coupon_id: "coupon-2".into(),
                    title: "Free drink".into(),
                },
            ],
            next: Some("token-2".into()),
        }
    );
    assert_eq!(
        single_query(&server).await.as_deref(),
        Some("status=DRAFT&status=RUNNING&start=token-1&limit=50")
    );
}

#[tokio::test]
async fn list_coupon_without_params_sends_no_query() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/coupon"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"items": []})))
        .expect(1)
        .mount(&server)
        .await;

    let res = client
        .list_coupon(&ListCouponParams::default())
        .await
        .unwrap();
    assert!(res.items.is_empty());
    assert_eq!(res.next, None);
    assert_eq!(single_query(&server).await, None);
}

fn create_request() -> CouponCreateRequest {
    CouponCreateRequest::new(
        LotteryAcquisitionConditionRequest::new(50, -1),
        1700000000,
        1800000000,
        -1,
        "10% off",
        CouponCreateVisibility::Public,
        CouponTimezone::AsiaTokyo,
    )
    .barcode_image_url("https://example.com/barcode.png")
    .coupon_code("CODE-1")
    .description("Discount on all items")
    .image_url("https://example.com/coupon.png")
    .usage_condition("Once per visit")
    .reward(
        CouponDiscountRewardRequest::default()
            .price_info(DiscountPercentagePriceInfoRequest::default().percentage(10)),
    )
}

#[tokio::test]
async fn create_coupon() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/coupon"))
        .and(bearer_token(TOKEN))
        .and(body_json(json!({
            "acquisitionCondition": {"type": "lottery", "lotteryProbability": 50, "maxAcquireCount": -1},
            "barcodeImageUrl": "https://example.com/barcode.png",
            "couponCode": "CODE-1",
            "description": "Discount on all items",
            "endTimestamp": 1800000000,
            "imageUrl": "https://example.com/coupon.png",
            "maxUseCountPerTicket": -1,
            "startTimestamp": 1700000000,
            "title": "10% off",
            "usageCondition": "Once per visit",
            "reward": {"type": "discount", "priceInfo": {"type": "percentage", "percentage": 10}},
            "visibility": "PUBLIC",
            "timezone": "ASIA_TOKYO"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"couponId": "coupon-1"})))
        .expect(1)
        .mount(&server)
        .await;

    let res = client.create_coupon(&create_request()).await.unwrap();
    assert_eq!(
        res,
        CouponCreateResponse {
            coupon_id: "coupon-1".into()
        }
    );
}

#[test]
fn minimal_create_request_omits_optional_fields() {
    let request = CouponCreateRequest::new(
        AcquisitionConditionRequest::Normal,
        1,
        2,
        1,
        "t",
        CouponCreateVisibility::Unlisted,
        CouponTimezone::EtcGmtMinus12,
    );
    assert_eq!(
        serde_json::to_value(request).unwrap(),
        json!({
            "acquisitionCondition": {"type": "normal"},
            "endTimestamp": 2,
            "maxUseCountPerTicket": 1,
            "startTimestamp": 1,
            "title": "t",
            "visibility": "UNLISTED",
            "timezone": "ETC_GMT_MINUS_12"
        })
    );
}

#[test]
fn reward_requests_serialize() {
    let rewards: Vec<CouponRewardRequest> = vec![
        CouponCashBackRewardRequest::default()
            .price_info(CashBackFixedPriceInfoRequest::default().fixed_amount(100))
            .into(),
        CouponCashBackRewardRequest::default()
            .price_info(CashBackPercentagePriceInfoRequest::default().percentage(5))
            .into(),
        CouponCashBackRewardRequest::default().into(),
        CouponDiscountRewardRequest::default()
            .price_info(DiscountFixedPriceInfoRequest::default().fixed_amount(300))
            .into(),
        CouponDiscountRewardRequest::default()
            .price_info(
                DiscountExplicitPriceInfoRequest::default()
                    .price_after_discount(800)
                    .original_price(1000),
            )
            .into(),
        CouponRewardRequest::Free,
        CouponRewardRequest::Gift,
        CouponRewardRequest::Others,
    ];
    assert_eq!(
        serde_json::to_value(rewards).unwrap(),
        json!([
            {"type": "cashBack", "priceInfo": {"type": "fixed", "fixedAmount": 100}},
            {"type": "cashBack", "priceInfo": {"type": "percentage", "percentage": 5}},
            {"type": "cashBack"},
            {"type": "discount", "priceInfo": {"type": "fixed", "fixedAmount": 300}},
            {"type": "discount", "priceInfo": {"type": "explicit", "priceAfterDiscount": 800, "originalPrice": 1000}},
            {"type": "free"},
            {"type": "gift"},
            {"type": "others"}
        ])
    );
}

#[test]
fn timezones_serialize_with_spec_names() {
    let all = [
        (CouponTimezone::EtcGmtMinus12, "ETC_GMT_MINUS_12"),
        (CouponTimezone::EtcGmtMinus11, "ETC_GMT_MINUS_11"),
        (CouponTimezone::PacificHonolulu, "PACIFIC_HONOLULU"),
        (CouponTimezone::AmericaAnchorage, "AMERICA_ANCHORAGE"),
        (CouponTimezone::AmericaLosAngeles, "AMERICA_LOS_ANGELES"),
        (CouponTimezone::AmericaPhoenix, "AMERICA_PHOENIX"),
        (CouponTimezone::AmericaChicago, "AMERICA_CHICAGO"),
        (CouponTimezone::AmericaNewYork, "AMERICA_NEW_YORK"),
        (CouponTimezone::AmericaCaracas, "AMERICA_CARACAS"),
        (CouponTimezone::AmericaSantiago, "AMERICA_SANTIAGO"),
        (CouponTimezone::AmericaStJohns, "AMERICA_ST_JOHNS"),
        (CouponTimezone::AmericaSaoPaulo, "AMERICA_SAO_PAULO"),
        (CouponTimezone::EtcGmtMinus2, "ETC_GMT_MINUS_2"),
        (CouponTimezone::AtlanticCapeVerde, "ATLANTIC_CAPE_VERDE"),
        (CouponTimezone::EuropeLondon, "EUROPE_LONDON"),
        (CouponTimezone::EuropeParis, "EUROPE_PARIS"),
        (CouponTimezone::EuropeIstanbul, "EUROPE_ISTANBUL"),
        (CouponTimezone::EuropeMoscow, "EUROPE_MOSCOW"),
        (CouponTimezone::AsiaTehran, "ASIA_TEHRAN"),
        (CouponTimezone::AsiaTbilisi, "ASIA_TBILISI"),
        (CouponTimezone::AsiaKabul, "ASIA_KABUL"),
        (CouponTimezone::AsiaTashkent, "ASIA_TASHKENT"),
        (CouponTimezone::AsiaColombo, "ASIA_COLOMBO"),
        (CouponTimezone::AsiaKathmandu, "ASIA_KATHMANDU"),
        (CouponTimezone::AsiaAlmaty, "ASIA_ALMATY"),
        (CouponTimezone::AsiaRangoon, "ASIA_RANGOON"),
        (CouponTimezone::AsiaBangkok, "ASIA_BANGKOK"),
        (CouponTimezone::AsiaTaipei, "ASIA_TAIPEI"),
        (CouponTimezone::AsiaTokyo, "ASIA_TOKYO"),
        (CouponTimezone::AustraliaDarwin, "AUSTRALIA_DARWIN"),
        (CouponTimezone::AustraliaSydney, "AUSTRALIA_SYDNEY"),
        (CouponTimezone::AsiaVladivostok, "ASIA_VLADIVOSTOK"),
        (CouponTimezone::EtcGmtPlus12, "ETC_GMT_PLUS_12"),
        (CouponTimezone::PacificTongatapu, "PACIFIC_TONGATAPU"),
    ];
    for (timezone, name) in all {
        assert_eq!(serde_json::to_value(timezone).unwrap(), json!(name));
        assert_eq!(
            serde_json::from_value::<CouponTimezone>(json!(name)).unwrap(),
            timezone
        );
    }
    assert_eq!(
        serde_json::from_value::<CouponTimezone>(json!("MARS_OLYMPUS")).unwrap(),
        CouponTimezone::Unknown
    );
}

#[test]
fn coupon_string_enums_tolerate_unknown_values() {
    let statuses: Vec<CouponStatus> =
        serde_json::from_value(json!(["DRAFT", "RUNNING", "CLOSED", "ARCHIVED"])).unwrap();
    assert_eq!(
        statuses,
        [
            CouponStatus::Draft,
            CouponStatus::Running,
            CouponStatus::Closed,
            CouponStatus::Unknown,
        ]
    );
    let visibilities: Vec<CouponVisibility> =
        serde_json::from_value(json!(["UNLISTED", "PUBLIC", "PRIVATE", "SECRET"])).unwrap();
    assert_eq!(
        visibilities,
        [
            CouponVisibility::Unlisted,
            CouponVisibility::Public,
            CouponVisibility::Private,
            CouponVisibility::Unknown,
        ]
    );
    let currencies: Vec<CouponCurrency> =
        serde_json::from_value(json!(["JPY", "THB", "TWD", "USD"])).unwrap();
    assert_eq!(
        currencies,
        [
            CouponCurrency::Jpy,
            CouponCurrency::Thb,
            CouponCurrency::Twd,
            CouponCurrency::Unknown,
        ]
    );
}

fn coupon_detail_json() -> Value {
    json!({
        "acquisitionCondition": {"type": "lottery", "lotteryProbability": 30, "maxAcquireCount": 100},
        "barcodeImageUrl": "https://example.com/barcode.png",
        "couponCode": "CODE-1",
        "description": "Discount on all items",
        "endTimestamp": 1800000000,
        "imageUrl": "https://example.com/coupon.png",
        "maxAcquireCount": 100,
        "maxUseCountPerTicket": 1,
        "maxTicketPerUser": 1,
        "startTimestamp": 1700000000,
        "title": "300 yen off",
        "usageCondition": "Once per visit",
        "reward": {"type": "discount", "priceInfo": {"type": "fixed", "currency": "JPY", "fixedAmount": 300}},
        "visibility": "PRIVATE",
        "timezone": "ASIA_TOKYO",
        "couponId": "coupon 1",
        "createdTimestamp": 1690000000,
        "status": "RUNNING"
    })
}

#[tokio::test]
async fn get_coupon_detail() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/coupon/coupon%201"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(coupon_detail_json()))
        .expect(1)
        .mount(&server)
        .await;

    let res = client.get_coupon_detail("coupon 1").await.unwrap();
    assert_eq!(
        res,
        CouponResponse {
            acquisition_condition: Some(AcquisitionConditionResponse::Lottery(
                LotteryAcquisitionConditionResponse {
                    lottery_probability: Some(30),
                    max_acquire_count: Some(100),
                }
            )),
            barcode_image_url: Some("https://example.com/barcode.png".into()),
            coupon_code: Some("CODE-1".into()),
            description: Some("Discount on all items".into()),
            end_timestamp: Some(1800000000),
            image_url: Some("https://example.com/coupon.png".into()),
            max_acquire_count: Some(100),
            max_use_count_per_ticket: Some(1),
            max_ticket_per_user: Some(1),
            start_timestamp: Some(1700000000),
            title: Some("300 yen off".into()),
            usage_condition: Some("Once per visit".into()),
            reward: Some(CouponRewardResponse::Discount(
                CouponDiscountRewardResponse {
                    price_info: Some(DiscountPriceInfoResponse::Fixed(
                        DiscountFixedPriceInfoResponse {
                            currency: Some(CouponCurrency::Jpy),
                            fixed_amount: Some(300),
                        }
                    )),
                }
            )),
            visibility: Some(CouponVisibility::Private),
            timezone: Some(CouponTimezone::AsiaTokyo),
            coupon_id: Some("coupon 1".into()),
            created_timestamp: Some(1690000000),
            status: Some(CouponStatus::Running),
        }
    );
    assert_eq!(serde_json::to_value(&res).unwrap(), coupon_detail_json());
}

#[test]
fn polymorphic_responses_decode_and_round_trip() {
    let conditions = json!([
        {"type": "normal"},
        {"type": "lottery", "lotteryProbability": 10, "maxAcquireCount": -1},
        {"type": "referral"},
        {"type": "invitation", "code": "x"}
    ]);
    let decoded: Vec<AcquisitionConditionResponse> =
        serde_json::from_value(conditions.clone()).unwrap();
    assert_eq!(
        decoded,
        [
            AcquisitionConditionResponse::Normal,
            AcquisitionConditionResponse::Lottery(LotteryAcquisitionConditionResponse {
                lottery_probability: Some(10),
                max_acquire_count: Some(-1),
            }),
            AcquisitionConditionResponse::Referral,
            AcquisitionConditionResponse::Unknown(json!({"type": "invitation", "code": "x"})),
        ]
    );
    assert_eq!(serde_json::to_value(&decoded).unwrap(), conditions);

    let rewards = json!([
        {"type": "cashBack", "priceInfo": {"type": "fixed", "currency": "THB", "fixedAmount": 50}},
        {"type": "cashBack", "priceInfo": {"type": "percentage", "percentage": 5}},
        {"type": "cashBack", "priceInfo": {"type": "points", "points": 5}},
        {"type": "discount", "priceInfo": {"type": "percentage", "percentage": 20}},
        {"type": "discount", "priceInfo": {"type": "explicit", "currency": "TWD", "priceAfterDiscount": 80, "originalPrice": 100}},
        {"type": "discount", "priceInfo": {"type": "bogo"}},
        {"type": "free"},
        {"type": "gift"},
        {"type": "others"},
        {"type": "stamp"}
    ]);
    let decoded: Vec<CouponRewardResponse> = serde_json::from_value(rewards.clone()).unwrap();
    assert_eq!(
        decoded,
        [
            CouponRewardResponse::CashBack(CouponCashBackRewardResponse {
                price_info: Some(CashBackPriceInfoResponse::Fixed(
                    CashBackFixedPriceInfoResponse {
                        currency: Some(CouponCurrency::Thb),
                        fixed_amount: Some(50),
                    }
                )),
            }),
            CouponRewardResponse::CashBack(CouponCashBackRewardResponse {
                price_info: Some(CashBackPriceInfoResponse::Percentage(
                    CashBackPercentagePriceInfoResponse {
                        percentage: Some(5)
                    }
                )),
            }),
            CouponRewardResponse::CashBack(CouponCashBackRewardResponse {
                price_info: Some(CashBackPriceInfoResponse::Unknown(
                    json!({"type": "points", "points": 5})
                )),
            }),
            CouponRewardResponse::Discount(CouponDiscountRewardResponse {
                price_info: Some(DiscountPriceInfoResponse::Percentage(
                    DiscountPercentagePriceInfoResponse {
                        percentage: Some(20)
                    }
                )),
            }),
            CouponRewardResponse::Discount(CouponDiscountRewardResponse {
                price_info: Some(DiscountPriceInfoResponse::Explicit(
                    DiscountExplicitPriceInfoResponse {
                        currency: Some(CouponCurrency::Twd),
                        price_after_discount: Some(80),
                        original_price: Some(100),
                    }
                )),
            }),
            CouponRewardResponse::Discount(CouponDiscountRewardResponse {
                price_info: Some(DiscountPriceInfoResponse::Unknown(json!({"type": "bogo"}))),
            }),
            CouponRewardResponse::Free,
            CouponRewardResponse::Gift,
            CouponRewardResponse::Others,
            CouponRewardResponse::Unknown(json!({"type": "stamp"})),
        ]
    );
    assert_eq!(serde_json::to_value(&decoded).unwrap(), rewards);
}

#[test]
fn polymorphic_responses_reject_malformed_known_variants() {
    assert!(
        serde_json::from_value::<AcquisitionConditionResponse>(
            json!({"type": "lottery", "lotteryProbability": "high"})
        )
        .is_err()
    );
    assert!(
        serde_json::from_value::<CouponRewardResponse>(
            json!({"type": "cashBack", "priceInfo": {"type": "fixed", "fixedAmount": "1"}})
        )
        .is_err()
    );
    assert!(
        serde_json::from_value::<CouponRewardResponse>(
            json!({"type": "discount", "priceInfo": {"type": "fixed", "fixedAmount": "1"}})
        )
        .is_err()
    );
    for value in [
        json!({"type": "fixed", "fixedAmount": "1"}),
        json!({"type": "percentage", "percentage": "1"}),
    ] {
        assert!(serde_json::from_value::<CashBackPriceInfoResponse>(value.clone()).is_err());
        assert!(serde_json::from_value::<DiscountPriceInfoResponse>(value).is_err());
    }
    assert!(
        serde_json::from_value::<DiscountPriceInfoResponse>(
            json!({"type": "explicit", "originalPrice": "1"})
        )
        .is_err()
    );
}

#[tokio::test]
async fn close_coupon() {
    let (server, client) = setup().await;
    Mock::given(method("PUT"))
        .and(path("/v2/bot/coupon/coupon-1/close"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    client.close_coupon("coupon-1").await.unwrap();
}

#[tokio::test]
async fn coupon_error_response() {
    let (server, client) = setup().await;
    Mock::given(method("PUT"))
        .and(path("/v2/bot/coupon/coupon-1/close"))
        .respond_with(
            ResponseTemplate::new(410)
                .insert_header("x-line-request-id", "req-410")
                .set_body_json(json!({
                    "message": "The coupon has already been closed",
                    "details": [{"message": "closed", "property": "couponId"}]
                })),
        )
        .mount(&server)
        .await;

    let err = client.close_coupon("coupon-1").await.unwrap_err();
    let Error::Api(api) = err else {
        panic!("expected Error::Api, got {err:?}");
    };
    assert_eq!(api.status, 410);
    assert_eq!(api.request_id.as_deref(), Some("req-410"));
    assert_eq!(api.body.message, "The coupon has already been closed");
    assert_eq!(api.body.details[0].property.as_deref(), Some("couponId"));
}
