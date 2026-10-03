mod common;

use common::setup;
use line_bot_messaging_api::api::*;
use line_bot_messaging_api::message::TextMessage;
use line_bot_messaging_api::{Error, ErrorDetail};
use serde_json::json;
use wiremock::matchers::{body_json, header, method, path, query_param, query_param_is_missing};
use wiremock::{Mock, MockServer, ResponseTemplate};

const AUTH: &str = "Bearer test-token";
const RETRY_KEY: &str = "123e4567-e89b-12d3-a456-426614174000";

async fn retry_key_sent(server: &MockServer) -> Option<String> {
    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 1);
    requests[0]
        .headers
        .get("x-line-retry-key")
        .map(|v| v.to_str().unwrap().to_owned())
}

#[tokio::test]
async fn reply_message() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/message/reply"))
        .and(header("authorization", AUTH))
        .and(body_json(json!({
            "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
            "messages": [{"type": "text", "text": "Hello"}, {"type": "text", "text": "World"}],
            "notificationDisabled": true
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "sentMessages": [
                {"id": "461230966842064897", "quoteToken": "IStG5h1Tz7b..."},
                {"id": "461230966842064898"}
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let request = ReplyMessageRequest::new(
        "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
        [TextMessage::new("Hello"), TextMessage::new("World")],
    )
    .notification_disabled(true);
    let response = client.reply_message(&request).await.unwrap();
    assert_eq!(
        response,
        ReplyMessageResponse {
            sent_messages: vec![
                SentMessage {
                    id: "461230966842064897".into(),
                    quote_token: Some("IStG5h1Tz7b...".into()),
                },
                SentMessage {
                    id: "461230966842064898".into(),
                    quote_token: None,
                },
            ],
        }
    );
}

#[tokio::test]
async fn push_message_with_retry_key() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/message/push"))
        .and(header("authorization", AUTH))
        .and(header("x-line-retry-key", RETRY_KEY))
        .and(body_json(json!({
            "to": "U4af4980629",
            "messages": [{"type": "text", "text": "Hello"}],
            "notificationDisabled": false,
            "customAggregationUnits": ["promotion_a"]
        })))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("x-line-request-id", "req-push")
                .set_body_json(
                    json!({"sentMessages": [{"id": "461230966842064897", "quoteToken": "q"}]}),
                ),
        )
        .expect(1)
        .mount(&server)
        .await;

    let request = PushMessageRequest::new("U4af4980629", [TextMessage::new("Hello")])
        .notification_disabled(false)
        .custom_aggregation_units(["promotion_a"])
        .retry_key(RETRY_KEY);
    let response = client.push_message(&request).await.unwrap();
    assert_eq!(response.request_id.as_deref(), Some("req-push"));
    assert_eq!(
        response.sent_messages,
        vec![SentMessage {
            id: "461230966842064897".into(),
            quote_token: Some("q".into()),
        }]
    );
}

#[tokio::test]
async fn push_message_without_retry_key() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/message/push"))
        .and(body_json(json!({
            "to": "U4af4980629",
            "messages": [{"type": "text", "text": "Hello"}]
        })))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"sentMessages": [{"id": "1"}]})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let request = PushMessageRequest::new("U4af4980629", [TextMessage::new("Hello")]);
    let response = client.push_message(&request).await.unwrap();
    assert_eq!(response.request_id, None);
    assert_eq!(retry_key_sent(&server).await, None);
}

#[tokio::test]
async fn push_message_conflict_returns_accepted_request_id_and_sent_messages() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/message/push"))
        .respond_with(
            ResponseTemplate::new(409)
                .insert_header("x-line-request-id", "req-new")
                .insert_header("x-line-accepted-request-id", "req-original")
                .set_body_json(json!({
                    "message": "The retry key is already accepted",
                    "sentMessages": [{"id": "461230966842064897", "quoteToken": "IStG5h1Tz7b..."}]
                })),
        )
        .mount(&server)
        .await;

    let request =
        PushMessageRequest::new("U4af4980629", [TextMessage::new("Hello")]).retry_key(RETRY_KEY);
    let err = client.push_message(&request).await.unwrap_err();
    let api = err.api_error().unwrap();
    assert_eq!(api.status, 409);
    assert_eq!(api.request_id.as_deref(), Some("req-new"));
    assert_eq!(api.accepted_request_id.as_deref(), Some("req-original"));
    assert_eq!(api.body.message, "The retry key is already accepted");
    assert_eq!(
        api.body.sent_messages,
        Some(vec![SentMessage {
            id: "461230966842064897".into(),
            quote_token: Some("IStG5h1Tz7b...".into()),
        }])
    );
}

#[tokio::test]
async fn push_message_bad_request_returns_details() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/message/push"))
        .respond_with(
            ResponseTemplate::new(400)
                .insert_header("x-line-request-id", "req-bad")
                .set_body_json(json!({
                    "message": "The request body has 2 error(s)",
                    "details": [
                        {"message": "May not be empty", "property": "messages[0].text"},
                        {"message": "Must be one of the following values: [text, image]", "property": "messages[1].type"}
                    ]
                })),
        )
        .mount(&server)
        .await;

    let request = PushMessageRequest::new("U4af4980629", [TextMessage::new("")]);
    let err = client.push_message(&request).await.unwrap_err();
    let Error::Api(api) = &err else {
        panic!("expected Error::Api, got {err:?}");
    };
    assert_eq!(api.status, 400);
    assert_eq!(api.request_id.as_deref(), Some("req-bad"));
    assert_eq!(api.accepted_request_id, None);
    assert_eq!(
        api.body.details,
        vec![
            ErrorDetail {
                message: Some("May not be empty".into()),
                property: Some("messages[0].text".into()),
            },
            ErrorDetail {
                message: Some("Must be one of the following values: [text, image]".into()),
                property: Some("messages[1].type".into()),
            },
        ]
    );
}

#[tokio::test]
async fn non_json_error_body_is_kept_as_message() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/message/quota"))
        .respond_with(ResponseTemplate::new(502).set_body_string("<html>Bad Gateway</html>"))
        .mount(&server)
        .await;

    let err = client.get_message_quota().await.unwrap_err();
    let api = err.api_error().unwrap();
    assert_eq!(api.status, 502);
    assert_eq!(api.body.message, "<html>Bad Gateway</html>");
    assert!(api.body.details.is_empty());
}

#[tokio::test]
async fn multicast_with_retry_key() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/message/multicast"))
        .and(header("authorization", AUTH))
        .and(header("x-line-retry-key", RETRY_KEY))
        .and(body_json(json!({
            "messages": [{"type": "text", "text": "Hello"}],
            "to": ["U4af4980629", "U0c229f96c4"],
            "notificationDisabled": true,
            "customAggregationUnits": ["promotion_a"]
        })))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("x-line-request-id", "req-multicast")
                .set_body_json(json!({})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let request =
        MulticastRequest::new(["U4af4980629", "U0c229f96c4"], [TextMessage::new("Hello")])
            .notification_disabled(true)
            .custom_aggregation_units(["promotion_a"])
            .retry_key(RETRY_KEY);
    let response = client.multicast(&request).await.unwrap();
    assert_eq!(response.request_id.as_deref(), Some("req-multicast"));
}

#[tokio::test]
async fn multicast_without_retry_key() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/message/multicast"))
        .and(body_json(json!({
            "messages": [{"type": "text", "text": "Hello"}],
            "to": ["U4af4980629"]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    let request = MulticastRequest::new(["U4af4980629"], [TextMessage::new("Hello")]);
    let response = client.multicast(&request).await.unwrap();
    assert_eq!(response, MulticastResponse::default());
    assert_eq!(retry_key_sent(&server).await, None);
}

#[tokio::test]
async fn narrowcast_with_recipient_filter_and_limit() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/message/narrowcast"))
        .and(header("authorization", AUTH))
        .and(header("x-line-retry-key", RETRY_KEY))
        .and(body_json(json!({
            "messages": [{"type": "text", "text": "Hello"}],
            "recipient": {
                "type": "operator",
                "and": [
                    {"type": "audience", "audienceGroupId": 5614991017776_i64},
                    {"type": "operator", "not": {"type": "audience", "audienceGroupId": 4389303728991_i64}}
                ]
            },
            "filter": {
                "demographic": {
                    "type": "operator",
                    "or": [
                        {"type": "operator", "and": [
                            {"type": "gender", "oneOf": ["male", "female"]},
                            {"type": "age", "gte": "age_20", "lt": "age_25"},
                            {"type": "appType", "oneOf": ["android", "ios"]},
                            {"type": "area", "oneOf": ["jp_23", "jp_05"]},
                            {"type": "subscriptionPeriod", "gte": "day_7", "lt": "day_30"}
                        ]},
                        {"type": "operator", "and": [
                            {"type": "age", "gte": "age_35", "lt": "age_40"},
                            {"type": "operator", "not": {"type": "gender", "oneOf": ["male"]}}
                        ]}
                    ]
                }
            },
            "limit": {"max": 100, "upToRemainingQuota": true, "forbidPartialDelivery": false},
            "notificationDisabled": true
        })))
        .respond_with(
            ResponseTemplate::new(202)
                .insert_header("x-line-request-id", "req-narrowcast")
                .set_body_json(json!({})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let recipient = OperatorRecipient::default().and([
        Recipient::from(AudienceRecipient::default().audience_group_id(5614991017776)),
        OperatorRecipient::default()
            .not(AudienceRecipient::default().audience_group_id(4389303728991))
            .into(),
    ]);
    let demographic = OperatorDemographicFilter::default().or([
        OperatorDemographicFilter::default().and([
            DemographicFilter::from(
                GenderDemographicFilter::default()
                    .one_of([GenderDemographic::Male, GenderDemographic::Female]),
            ),
            AgeDemographicFilter::default()
                .gte(AgeDemographic::Age20)
                .lt(AgeDemographic::Age25)
                .into(),
            AppTypeDemographicFilter::default()
                .one_of([AppTypeDemographic::Android, AppTypeDemographic::Ios])
                .into(),
            AreaDemographicFilter::default()
                .one_of([AreaDemographic::Aichi, AreaDemographic::Akita])
                .into(),
            SubscriptionPeriodDemographicFilter::default()
                .gte(SubscriptionPeriodDemographic::Day7)
                .lt(SubscriptionPeriodDemographic::Day30)
                .into(),
        ]),
        OperatorDemographicFilter::default().and([
            DemographicFilter::from(
                AgeDemographicFilter::default()
                    .gte(AgeDemographic::Age35)
                    .lt(AgeDemographic::Age40),
            ),
            OperatorDemographicFilter::default()
                .not(GenderDemographicFilter::default().one_of([GenderDemographic::Male]))
                .into(),
        ]),
    ]);
    let request = NarrowcastRequest::new([TextMessage::new("Hello")])
        .recipient(recipient)
        .filter(Filter::default().demographic(demographic))
        .limit(
            Limit::default()
                .max(100)
                .up_to_remaining_quota(true)
                .forbid_partial_delivery(false),
        )
        .notification_disabled(true)
        .retry_key(RETRY_KEY);
    let response = client.narrowcast(&request).await.unwrap();
    assert_eq!(response.request_id.as_deref(), Some("req-narrowcast"));
}

#[tokio::test]
async fn narrowcast_minimal_without_retry_key() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/message/narrowcast"))
        .and(body_json(json!({
            "messages": [{"type": "text", "text": "Hello"}],
            "recipient": {
                "type": "operator",
                "or": [{"type": "redelivery", "requestId": "5b59509c-c57b-11e9-aa8c-2a2ae2dbcce4"}]
            }
        })))
        .respond_with(ResponseTemplate::new(202))
        .expect(1)
        .mount(&server)
        .await;

    let request = NarrowcastRequest::new([TextMessage::new("Hello")]).recipient(
        OperatorRecipient::default().or([
            RedeliveryRecipient::default().request_id("5b59509c-c57b-11e9-aa8c-2a2ae2dbcce4")
        ]),
    );
    let response = client.narrowcast(&request).await.unwrap();
    assert_eq!(response, NarrowcastResponse::default());
    assert_eq!(retry_key_sent(&server).await, None);
}

#[test]
fn area_demographic_codes_follow_descriptions() {
    // x-enum-varnames is ordered differently from enum; the codes come from x-enum-descriptions.
    let cases = [
        (AreaDemographic::Hokkaido, "jp_01"),
        (AreaDemographic::Tokyo, "jp_13"),
        (AreaDemographic::Hyougo, "jp_28"),
        (AreaDemographic::Okinawa, "jp_47"),
        (AreaDemographic::TaipeiCity, "tw_01"),
        (AreaDemographic::LienchiangCounty, "tw_22"),
        (AreaDemographic::Bangkok, "th_01"),
        (AreaDemographic::Western, "th_08"),
        (AreaDemographic::Bali, "id_01"),
        (AreaDemographic::Jabodetabek, "id_04"),
        (AreaDemographic::Lainnya, "id_05"),
        (AreaDemographic::Makassar, "id_06"),
        (AreaDemographic::Yogyakarta, "id_12"),
    ];
    for (area, code) in cases {
        assert_eq!(serde_json::to_value(area).unwrap(), json!(code));
    }
}

#[test]
fn demographic_enums_serialize_every_value() {
    let ages = [
        AgeDemographic::Age15,
        AgeDemographic::Age20,
        AgeDemographic::Age25,
        AgeDemographic::Age30,
        AgeDemographic::Age35,
        AgeDemographic::Age40,
        AgeDemographic::Age45,
        AgeDemographic::Age50,
        AgeDemographic::Age55,
        AgeDemographic::Age60,
        AgeDemographic::Age65,
        AgeDemographic::Age70,
    ];
    assert_eq!(
        serde_json::to_value(ages).unwrap(),
        json!([
            "age_15", "age_20", "age_25", "age_30", "age_35", "age_40", "age_45", "age_50",
            "age_55", "age_60", "age_65", "age_70"
        ])
    );
    let periods = [
        SubscriptionPeriodDemographic::Day7,
        SubscriptionPeriodDemographic::Day30,
        SubscriptionPeriodDemographic::Day90,
        SubscriptionPeriodDemographic::Day180,
        SubscriptionPeriodDemographic::Day365,
    ];
    assert_eq!(
        serde_json::to_value(periods).unwrap(),
        json!(["day_7", "day_30", "day_90", "day_180", "day_365"])
    );
}

#[tokio::test]
async fn get_narrowcast_progress() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/message/progress/narrowcast"))
        .and(query_param(
            "requestId",
            "7d51557d-7c6c-4b5b-a1a8-d4e3c6e2c5bd",
        ))
        .and(header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "phase": "failed",
            "successCount": 1,
            "failureCount": 2,
            "targetCount": 3,
            "failedDescription": "unknown",
            "errorCode": 1,
            "acceptedTime": "2020-12-03T10:15:30.121Z",
            "completedTime": "2020-12-03T10:15:31.121Z"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let response = client
        .get_narrowcast_progress("7d51557d-7c6c-4b5b-a1a8-d4e3c6e2c5bd")
        .await
        .unwrap();
    assert_eq!(
        response,
        NarrowcastProgressResponse {
            phase: NarrowcastPhase::Failed,
            success_count: Some(1),
            failure_count: Some(2),
            target_count: Some(3),
            failed_description: Some("unknown".into()),
            error_code: Some(1),
            accepted_time: "2020-12-03T10:15:30.121Z".into(),
            completed_time: Some("2020-12-03T10:15:31.121Z".into()),
        }
    );
}

#[test]
fn narrowcast_phase_values() {
    let phases: Vec<NarrowcastPhase> = serde_json::from_value(json!([
        "waiting",
        "sending",
        "succeeded",
        "failed",
        "paused"
    ]))
    .unwrap();
    assert_eq!(
        phases,
        vec![
            NarrowcastPhase::Waiting,
            NarrowcastPhase::Sending,
            NarrowcastPhase::Succeeded,
            NarrowcastPhase::Failed,
            NarrowcastPhase::Unknown,
        ]
    );
}

#[tokio::test]
async fn broadcast_with_retry_key() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/message/broadcast"))
        .and(header("authorization", AUTH))
        .and(header("x-line-retry-key", RETRY_KEY))
        .and(body_json(json!({
            "messages": [{"type": "text", "text": "Hello"}],
            "notificationDisabled": true
        })))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("x-line-request-id", "req-broadcast")
                .set_body_json(json!({})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let request = BroadcastRequest::new([TextMessage::new("Hello")])
        .notification_disabled(true)
        .retry_key(RETRY_KEY);
    let response = client.broadcast(&request).await.unwrap();
    assert_eq!(response.request_id.as_deref(), Some("req-broadcast"));
}

#[tokio::test]
async fn broadcast_without_retry_key() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/message/broadcast"))
        .and(body_json(
            json!({"messages": [{"type": "text", "text": "Hello"}]}),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    let request = BroadcastRequest::new([TextMessage::new("Hello")]);
    let response = client.broadcast(&request).await.unwrap();
    assert_eq!(response, BroadcastResponse::default());
    assert_eq!(retry_key_sent(&server).await, None);
}

#[tokio::test]
async fn get_message_quota() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/message/quota"))
        .and(header("authorization", AUTH))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"type": "limited", "value": 1000})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let response = client.get_message_quota().await.unwrap();
    assert_eq!(
        response,
        MessageQuotaResponse {
            r#type: QuotaType::Limited,
            value: Some(1000),
        }
    );
}

#[test]
fn quota_type_values() {
    let types: Vec<QuotaType> =
        serde_json::from_value(json!(["none", "limited", "unlimited"])).unwrap();
    assert_eq!(
        types,
        vec![QuotaType::None, QuotaType::Limited, QuotaType::Unknown]
    );
}

#[tokio::test]
async fn get_message_quota_consumption() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/message/quota/consumption"))
        .and(header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"totalUsage": 500})))
        .expect(1)
        .mount(&server)
        .await;

    let response = client.get_message_quota_consumption().await.unwrap();
    assert_eq!(response, QuotaConsumptionResponse { total_usage: 500 });
}

async fn mount_delivery(server: &MockServer, kind: &str, body: serde_json::Value) {
    Mock::given(method("GET"))
        .and(path(format!("/v2/bot/message/delivery/{kind}")))
        .and(query_param("date", "20191231"))
        .and(header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .expect(1)
        .mount(server)
        .await;
}

#[tokio::test]
async fn get_number_of_sent_messages() {
    let (server, client) = setup().await;
    mount_delivery(
        &server,
        "reply",
        json!({"status": "ready", "success": 10000}),
    )
    .await;
    mount_delivery(&server, "push", json!({"status": "unready"})).await;
    mount_delivery(
        &server,
        "multicast",
        json!({"status": "unavailable_for_privacy"}),
    )
    .await;
    mount_delivery(&server, "broadcast", json!({"status": "out_of_service"})).await;

    assert_eq!(
        client
            .get_number_of_sent_reply_messages("20191231")
            .await
            .unwrap(),
        NumberOfMessagesResponse {
            status: NumberOfMessagesStatus::Ready,
            success: Some(10000),
        }
    );
    assert_eq!(
        client
            .get_number_of_sent_push_messages("20191231")
            .await
            .unwrap(),
        NumberOfMessagesResponse {
            status: NumberOfMessagesStatus::Unready,
            success: None,
        }
    );
    assert_eq!(
        client
            .get_number_of_sent_multicast_messages("20191231")
            .await
            .unwrap()
            .status,
        NumberOfMessagesStatus::UnavailableForPrivacy
    );
    assert_eq!(
        client
            .get_number_of_sent_broadcast_messages("20191231")
            .await
            .unwrap()
            .status,
        NumberOfMessagesStatus::OutOfService
    );
}

#[test]
fn number_of_messages_status_unknown() {
    let status: NumberOfMessagesStatus = serde_json::from_value(json!("archived")).unwrap();
    assert_eq!(status, NumberOfMessagesStatus::Unknown);
}

#[tokio::test]
async fn validate_endpoints() {
    let (server, client) = setup().await;
    for kind in ["reply", "push", "multicast", "narrowcast", "broadcast"] {
        Mock::given(method("POST"))
            .and(path(format!("/v2/bot/message/validate/{kind}")))
            .and(header("authorization", AUTH))
            .and(body_json(
                json!({"messages": [{"type": "text", "text": "Hello"}]}),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .expect(1)
            .mount(&server)
            .await;
    }

    let request = ValidateMessageRequest::new([TextMessage::new("Hello")]);
    client.validate_reply(&request).await.unwrap();
    client.validate_push(&request).await.unwrap();
    client.validate_multicast(&request).await.unwrap();
    client.validate_narrowcast(&request).await.unwrap();
    client.validate_broadcast(&request).await.unwrap();
}

#[tokio::test]
async fn get_aggregation_unit_usage() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/message/aggregation/info"))
        .and(header("authorization", AUTH))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"numOfCustomAggregationUnits": 22})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let response = client.get_aggregation_unit_usage().await.unwrap();
    assert_eq!(
        response,
        GetAggregationUnitUsageResponse {
            num_of_custom_aggregation_units: 22,
        }
    );
}

#[tokio::test]
async fn get_aggregation_unit_name_list_with_paging() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/message/aggregation/list"))
        .and(query_param("limit", "2"))
        .and(query_param("start", "jxEWCEEP..."))
        .and(header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "customAggregationUnits": ["promotion_a", "promotion_b"],
            "next": "jxEWCEEP2..."
        })))
        .expect(1)
        .mount(&server)
        .await;

    let response = client
        .get_aggregation_unit_name_list(Some("2"), Some("jxEWCEEP..."))
        .await
        .unwrap();
    assert_eq!(
        response,
        GetAggregationUnitNameListResponse {
            custom_aggregation_units: vec!["promotion_a".into(), "promotion_b".into()],
            next: Some("jxEWCEEP2...".into()),
        }
    );
}

#[tokio::test]
async fn get_aggregation_unit_name_list_without_paging() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/message/aggregation/list"))
        .and(query_param_is_missing("limit"))
        .and(query_param_is_missing("start"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"customAggregationUnits": []})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let response = client
        .get_aggregation_unit_name_list(None, None)
        .await
        .unwrap();
    assert!(response.custom_aggregation_units.is_empty());
    assert_eq!(response.next, None);
}

#[tokio::test]
async fn mark_messages_as_read() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/message/markAsRead"))
        .and(header("authorization", AUTH))
        .and(body_json(
            json!({"chat": {"userId": "Uxxxxxxxxxxxxxxxxxx"}}),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    let request = MarkMessagesAsReadRequest::new(ChatReference::new("Uxxxxxxxxxxxxxxxxxx"));
    client.mark_messages_as_read(&request).await.unwrap();
}

#[tokio::test]
async fn show_loading_animation() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/chat/loading/start"))
        .and(header("authorization", AUTH))
        .and(body_json(
            json!({"chatId": "U4af4980629", "loadingSeconds": 5}),
        ))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    let request = ShowLoadingAnimationRequest::new("U4af4980629").loading_seconds(5);
    client.show_loading_animation(&request).await.unwrap();
}

#[test]
fn show_loading_animation_request_omits_unset_seconds() {
    let request = ShowLoadingAnimationRequest::new("U4af4980629");
    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({"chatId": "U4af4980629"})
    );
}

#[tokio::test]
async fn mark_messages_as_read_by_token() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/chat/markAsRead"))
        .and(header("authorization", AUTH))
        .and(body_json(json!({"markAsReadToken": "nz0x74mZT2mRn..."})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    let request = MarkMessagesAsReadByTokenRequest::new("nz0x74mZT2mRn...");
    client
        .mark_messages_as_read_by_token(&request)
        .await
        .unwrap();
}

#[tokio::test]
async fn bulk_sends_return_rate_limit_errors() {
    let (server, client) = setup().await;
    for kind in ["multicast", "narrowcast", "broadcast"] {
        Mock::given(method("POST"))
            .and(path(format!("/v2/bot/message/{kind}")))
            .respond_with(
                ResponseTemplate::new(429)
                    .set_body_json(json!({"message": "You have reached your monthly limit."})),
            )
            .expect(1)
            .mount(&server)
            .await;
    }

    let messages = [TextMessage::new("Hello")];
    let errors = [
        client
            .multicast(&MulticastRequest::new(["U4af4980629"], messages.clone()))
            .await
            .unwrap_err(),
        client
            .narrowcast(&NarrowcastRequest::new(messages.clone()))
            .await
            .unwrap_err(),
        client
            .broadcast(&BroadcastRequest::new(messages))
            .await
            .unwrap_err(),
    ];
    for err in errors {
        let api = err.api_error().unwrap();
        assert_eq!(api.status, 429);
        assert_eq!(api.body.message, "You have reached your monthly limit.");
    }
}
