mod common;

use common::{TOKEN, setup};
use line_bot_messaging_api::Error;
use line_bot_messaging_api::api::*;
use serde_json::json;
use wiremock::matchers::{bearer_token, method, path, query_param, query_param_is_missing};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn get_friends_demographics() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/insight/demographic"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "available": true,
            "genders": [
                {"gender": "unknown", "percentage": 37.6},
                {"gender": "male", "percentage": 31.8},
                {"gender": "female", "percentage": 30.6}
            ],
            "ages": [
                {"age": "unknown", "percentage": 37.6},
                {"age": "from50", "percentage": 17.3}
            ],
            "areas": [
                {"area": "unknown", "percentage": 42.9},
                {"area": "徳島", "percentage": 2.9}
            ],
            "appTypes": [
                {"appType": "ios", "percentage": 62.4},
                {"appType": "android", "percentage": 27.7},
                {"appType": "others", "percentage": 9.9}
            ],
            "subscriptionPeriods": [
                {"subscriptionPeriod": "over365days", "percentage": 96.4},
                {"subscriptionPeriod": "within365days", "percentage": 1.9},
                {"subscriptionPeriod": "within180days", "percentage": 1.2},
                {"subscriptionPeriod": "within90days", "percentage": 0.5},
                {"subscriptionPeriod": "within30days", "percentage": 0.1},
                {"subscriptionPeriod": "within7days", "percentage": 0}
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let res = client.get_friends_demographics().await.unwrap();
    assert_eq!(res.available, Some(true));
    let genders = res.genders.unwrap();
    assert_eq!(
        genders.iter().map(|t| t.gender).collect::<Vec<_>>(),
        [
            Some(Gender::Unknown),
            Some(Gender::Male),
            Some(Gender::Female)
        ]
    );
    assert_eq!(genders[1].percentage, Some(31.8));
    let ages = res.ages.unwrap();
    assert_eq!(ages[1].age, Some(AgeGroup::From50));
    let areas = res.areas.unwrap();
    assert_eq!(areas[1].area.as_deref(), Some("徳島"));
    let app_types = res.app_types.unwrap();
    assert_eq!(
        app_types.iter().map(|t| t.app_type).collect::<Vec<_>>(),
        [
            Some(AppType::Ios),
            Some(AppType::Android),
            Some(AppType::Others)
        ]
    );
    let periods = res.subscription_periods.unwrap();
    assert_eq!(
        periods
            .iter()
            .map(|t| t.subscription_period)
            .collect::<Vec<_>>(),
        [
            Some(SubscriptionPeriod::Over365Days),
            Some(SubscriptionPeriod::Within365Days),
            Some(SubscriptionPeriod::Within180Days),
            Some(SubscriptionPeriod::Within90Days),
            Some(SubscriptionPeriod::Within30Days),
            Some(SubscriptionPeriod::Within7Days),
        ]
    );
    assert_eq!(periods[5].percentage, Some(0.0));
}

#[test]
fn demographic_enums_cover_spec_and_unknown_values() {
    let ages: Vec<AgeGroup> = serde_json::from_value(json!([
        "from0to14",
        "from15to19",
        "from20to24",
        "from25to29",
        "from30to34",
        "from35to39",
        "from40to44",
        "from45to49",
        "from50",
        "from50to54",
        "from55to59",
        "from60to64",
        "from65to69",
        "from70",
        "unknown",
        "from80"
    ]))
    .unwrap();
    assert_eq!(
        ages,
        [
            AgeGroup::From0To14,
            AgeGroup::From15To19,
            AgeGroup::From20To24,
            AgeGroup::From25To29,
            AgeGroup::From30To34,
            AgeGroup::From35To39,
            AgeGroup::From40To44,
            AgeGroup::From45To49,
            AgeGroup::From50,
            AgeGroup::From50To54,
            AgeGroup::From55To59,
            AgeGroup::From60To64,
            AgeGroup::From65To69,
            AgeGroup::From70,
            AgeGroup::Unknown,
            AgeGroup::Unknown,
        ]
    );
    assert_eq!(
        serde_json::to_value(AgeGroup::From0To14).unwrap(),
        json!("from0to14")
    );
    assert_eq!(
        serde_json::to_value(AgeGroup::Unknown).unwrap(),
        json!("unknown")
    );

    let gender: Gender = serde_json::from_value(json!("other")).unwrap();
    assert_eq!(gender, Gender::Unknown);
    let app_type: AppType = serde_json::from_value(json!("windows")).unwrap();
    assert_eq!(app_type, AppType::Unknown);
    let periods: Vec<SubscriptionPeriod> =
        serde_json::from_value(json!(["unknown", "over730days"])).unwrap();
    assert_eq!(
        periods,
        [SubscriptionPeriod::Unknown, SubscriptionPeriod::Unknown]
    );
    let statuses: Vec<InsightStatus> =
        serde_json::from_value(json!(["ready", "unready", "out_of_service", "paused"])).unwrap();
    assert_eq!(
        statuses,
        [
            InsightStatus::Ready,
            InsightStatus::Unready,
            InsightStatus::OutOfService,
            InsightStatus::Unknown,
        ]
    );
}

#[tokio::test]
async fn get_number_of_message_deliveries() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/insight/message/delivery"))
        .and(query_param("date", "20191231"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "ready",
            "broadcast": 5385,
            "targeting": 522,
            "autoResponse": 1,
            "welcomeResponse": 2,
            "chat": 3,
            "apiBroadcast": 4,
            "apiPush": 5,
            "apiMulticast": 6,
            "apiNarrowcast": 7,
            "apiReply": 8
        })))
        .expect(1)
        .mount(&server)
        .await;

    let res = client
        .get_number_of_message_deliveries("20191231")
        .await
        .unwrap();
    assert_eq!(
        res,
        GetNumberOfMessageDeliveriesResponse {
            status: Some(InsightStatus::Ready),
            broadcast: Some(5385),
            targeting: Some(522),
            auto_response: Some(1),
            welcome_response: Some(2),
            chat: Some(3),
            api_broadcast: Some(4),
            api_push: Some(5),
            api_multicast: Some(6),
            api_narrowcast: Some(7),
            api_reply: Some(8),
        }
    );
}

#[tokio::test]
async fn get_number_of_followers_with_date() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/insight/followers"))
        .and(query_param("date", "20191231"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "ready",
            "followers": 7620,
            "targetedReaches": 5848,
            "blocks": 237
        })))
        .expect(1)
        .mount(&server)
        .await;

    let res = client
        .get_number_of_followers(Some("20191231"))
        .await
        .unwrap();
    assert_eq!(
        res,
        GetNumberOfFollowersResponse {
            status: Some(InsightStatus::Ready),
            followers: Some(7620),
            targeted_reaches: Some(5848),
            blocks: Some(237),
        }
    );
}

#[tokio::test]
async fn get_number_of_followers_without_date() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/insight/followers"))
        .and(query_param_is_missing("date"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "unready"})))
        .expect(1)
        .mount(&server)
        .await;

    let res = client.get_number_of_followers(None).await.unwrap();
    assert_eq!(res.status, Some(InsightStatus::Unready));
    assert_eq!(res.followers, None);
}

#[tokio::test]
async fn get_message_event() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/insight/message/event"))
        .and(query_param("requestId", "f70dd685-499a-4231-a441-f24b8d4fba21"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "overview": {
                "requestId": "f70dd685-499a-4231-a441-f24b8d4fba21",
                "timestamp": 1568214000,
                "delivered": 320,
                "uniqueImpression": 82,
                "uniqueClick": 51,
                "uniqueMediaPlayed": null,
                "uniqueMediaPlayed100Percent": null
            },
            "messages": [{
                "seq": 1,
                "impression": 136,
                "mediaPlayed": null,
                "mediaPlayed25Percent": null,
                "mediaPlayed50Percent": null,
                "mediaPlayed75Percent": null,
                "mediaPlayed100Percent": 4,
                "uniqueMediaPlayed": null,
                "uniqueMediaPlayed25Percent": null,
                "uniqueMediaPlayed50Percent": null,
                "uniqueMediaPlayed75Percent": null,
                "uniqueMediaPlayed100Percent": 3
            }],
            "clicks": [
                {"seq": 1, "url": "https://line.me/", "click": 41, "uniqueClick": 30, "uniqueClickOfRequest": 30},
                {"seq": 1, "url": "https://www.linebiz.com/", "click": 59, "uniqueClick": 38, "uniqueClickOfRequest": 38}
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let res = client
        .get_message_event("f70dd685-499a-4231-a441-f24b8d4fba21")
        .await
        .unwrap();
    assert_eq!(
        res.overview,
        Some(GetMessageEventResponseOverview {
            request_id: Some("f70dd685-499a-4231-a441-f24b8d4fba21".into()),
            timestamp: Some(1568214000),
            delivered: Some(320),
            unique_impression: Some(82),
            unique_click: Some(51),
            unique_media_played: None,
            unique_media_played100_percent: None,
        })
    );
    let messages = res.messages.unwrap();
    assert_eq!(
        messages[0],
        GetMessageEventResponseMessage {
            seq: Some(1),
            impression: Some(136),
            media_played100_percent: Some(4),
            unique_media_played100_percent: Some(3),
            ..Default::default()
        }
    );
    let clicks = res.clicks.unwrap();
    assert_eq!(
        clicks[1],
        GetMessageEventResponseClick {
            seq: Some(1),
            url: Some("https://www.linebiz.com/".into()),
            click: Some(59),
            unique_click: Some(38),
            unique_click_of_request: Some(38),
        }
    );
}

#[tokio::test]
async fn get_statistics_per_unit() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/insight/message/event/aggregation"))
        .and(query_param("customAggregationUnit", "promotion_a"))
        .and(query_param("from", "20210301"))
        .and(query_param("to", "20210331"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "overview": {
                "uniqueImpression": 40,
                "uniqueClick": 30,
                "uniqueMediaPlayed": 25,
                "uniqueMediaPlayed100Percent": null
            },
            "messages": [{
                "seq": 1,
                "impression": 42,
                "mediaPlayed": 30,
                "mediaPlayed25Percent": null,
                "mediaPlayed50Percent": null,
                "mediaPlayed75Percent": null,
                "mediaPlayed100Percent": null,
                "uniqueImpression": 41,
                "uniqueMediaPlayed": 25,
                "uniqueMediaPlayed25Percent": null,
                "uniqueMediaPlayed50Percent": null,
                "uniqueMediaPlayed75Percent": null,
                "uniqueMediaPlayed100Percent": null
            }],
            "clicks": [
                {"seq": 1, "url": "https://developers.line.biz/", "click": 35, "uniqueClick": 25, "uniqueClickOfRequest": null},
                {"seq": 1, "url": "https://developers.line.biz/", "click": 29, "uniqueClick": null, "uniqueClickOfRequest": null}
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let res = client
        .get_statistics_per_unit("promotion_a", "20210301", "20210331")
        .await
        .unwrap();
    assert_eq!(
        res,
        GetStatisticsPerUnitResponse {
            overview: GetStatisticsPerUnitResponseOverview {
                unique_impression: Some(40),
                unique_click: Some(30),
                unique_media_played: Some(25),
                unique_media_played100_percent: None,
            },
            messages: vec![GetStatisticsPerUnitResponseMessage {
                seq: 1,
                impression: Some(42),
                media_played: Some(30),
                unique_impression: Some(41),
                unique_media_played: Some(25),
                ..Default::default()
            }],
            clicks: vec![
                GetStatisticsPerUnitResponseClick {
                    seq: 1,
                    url: "https://developers.line.biz/".into(),
                    click: Some(35),
                    unique_click: Some(25),
                    unique_click_of_request: None,
                },
                GetStatisticsPerUnitResponseClick {
                    seq: 1,
                    url: "https://developers.line.biz/".into(),
                    click: Some(29),
                    unique_click: None,
                    unique_click_of_request: None,
                },
            ],
        }
    );
}

#[tokio::test]
async fn get_rich_menu_insight_summary() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path(
            "/v2/bot/insight/richmenu/richmenu-0123456789abcdef0123456789abcdef/summary",
        ))
        .and(query_param("from", "20260213"))
        .and(query_param("to", "20260215"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "richMenuId": "richmenu-0123456789abcdef0123456789abcdef",
            "metricsFrom": "20260213",
            "metricsTo": "20260215",
            "impression": {"metrics": {"count": 567, "uniqueUsers": 300}},
            "clicks": [
                {"bounds": {"x": 0, "y": 0, "width": 400, "height": 250}, "metrics": {"count": 123, "uniqueUsers": 45}},
                {"bounds": {"x": 400, "y": 0, "width": 400, "height": 250}, "metrics": {"count": 45, "uniqueUsers": 24}}
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let res = client
        .get_rich_menu_insight_summary(
            "richmenu-0123456789abcdef0123456789abcdef",
            "20260213",
            "20260215",
        )
        .await
        .unwrap();
    assert_eq!(
        res,
        GetRichMenuInsightSummaryResponse {
            rich_menu_id: "richmenu-0123456789abcdef0123456789abcdef".into(),
            metrics_from: Some("20260213".into()),
            metrics_to: Some("20260215".into()),
            impression: Some(GetRichMenuInsightSummaryResponseImpression {
                metrics: GetRichMenuInsightSummaryResponseMetrics {
                    count: 567,
                    unique_users: 300,
                },
            }),
            clicks: Some(vec![
                GetRichMenuInsightSummaryResponseClick {
                    bounds: RichMenuInsightBounds {
                        x: 0,
                        y: 0,
                        width: 400,
                        height: 250,
                    },
                    metrics: GetRichMenuInsightSummaryResponseMetrics {
                        count: 123,
                        unique_users: 45,
                    },
                },
                GetRichMenuInsightSummaryResponseClick {
                    bounds: RichMenuInsightBounds {
                        x: 400,
                        y: 0,
                        width: 400,
                        height: 250,
                    },
                    metrics: GetRichMenuInsightSummaryResponseMetrics {
                        count: 45,
                        unique_users: 24,
                    },
                },
            ]),
        }
    );
}

#[tokio::test]
async fn get_rich_menu_insight_summary_below_threshold() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/insight/richmenu/richmenu%2F1/summary"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"richMenuId": "richmenu/1"})))
        .expect(1)
        .mount(&server)
        .await;

    let res = client
        .get_rich_menu_insight_summary("richmenu/1", "20260213", "20260215")
        .await
        .unwrap();
    assert_eq!(
        res,
        GetRichMenuInsightSummaryResponse {
            rich_menu_id: "richmenu/1".into(),
            ..Default::default()
        }
    );
}

#[tokio::test]
async fn get_rich_menu_insight_daily() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path(
            "/v2/bot/insight/richmenu/richmenu-0123456789abcdef0123456789abcdef/daily",
        ))
        .and(query_param("from", "20260213"))
        .and(query_param("to", "20260214"))
        .and(bearer_token(TOKEN))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "richMenuId": "richmenu-0123456789abcdef0123456789abcdef",
            "metricsFrom": "20260213",
            "metricsTo": "20260214",
            "impression": {
                "metrics": [
                    {"date": "20260213", "count": 300, "uniqueUsers": 191},
                    {"date": "20260214", "count": 267, "uniqueUsers": 222}
                ]
            },
            "clicks": [{
                "bounds": {"x": 0, "y": 0, "width": 400, "height": 250},
                "metrics": [
                    {"date": "20260213", "count": 45, "uniqueUsers": 22},
                    {"date": "20260214", "count": 78, "uniqueUsers": 35}
                ]
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let res = client
        .get_rich_menu_insight_daily(
            "richmenu-0123456789abcdef0123456789abcdef",
            "20260213",
            "20260214",
        )
        .await
        .unwrap();
    let day = |date: &str, count, unique_users| GetRichMenuInsightDailyResponseDailyMetrics {
        date: date.into(),
        count,
        unique_users,
    };
    assert_eq!(
        res,
        GetRichMenuInsightDailyResponse {
            rich_menu_id: "richmenu-0123456789abcdef0123456789abcdef".into(),
            metrics_from: Some("20260213".into()),
            metrics_to: Some("20260214".into()),
            impression: Some(GetRichMenuInsightDailyResponseImpression {
                metrics: vec![day("20260213", 300, 191), day("20260214", 267, 222)],
            }),
            clicks: Some(vec![GetRichMenuInsightDailyResponseClick {
                bounds: RichMenuInsightBounds {
                    x: 0,
                    y: 0,
                    width: 400,
                    height: 250,
                },
                metrics: vec![day("20260213", 45, 22), day("20260214", 78, 35)],
            }]),
        }
    );
}

#[tokio::test]
async fn insight_error_response() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/insight/richmenu/richmenu-404/daily"))
        .respond_with(
            ResponseTemplate::new(400)
                .insert_header("x-line-request-id", "req-400")
                .set_body_json(json!({
                    "message": "The request body has 1 error(s)",
                    "details": [{"message": "invalid date", "property": "from"}]
                })),
        )
        .mount(&server)
        .await;

    let err = client
        .get_rich_menu_insight_daily("richmenu-404", "2026", "20260214")
        .await
        .unwrap_err();
    let Error::Api(api) = err else {
        panic!("expected Error::Api, got {err:?}");
    };
    assert_eq!(api.status, 400);
    assert_eq!(api.request_id.as_deref(), Some("req-400"));
    assert_eq!(api.body.message, "The request body has 1 error(s)");
    assert_eq!(api.body.details[0].property.as_deref(), Some("from"));
}
