mod common;

use common::setup;
use line_bot_messaging_api::api::*;
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param, query_param_is_missing};
use wiremock::{Mock, ResponseTemplate};

const AUTH: &str = "Bearer test-token";

#[tokio::test]
async fn get_profile_escapes_user_id() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/profile/U4af%2F49%2080629"))
        .and(header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "displayName": "LINE taro",
            "userId": "U4af4980629...",
            "language": "en",
            "pictureUrl": "https://obs.line-apps.com/...",
            "statusMessage": "Hello, LINE!"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let response = client.get_profile("U4af/49 80629").await.unwrap();
    assert_eq!(
        response,
        UserProfileResponse {
            display_name: "LINE taro".into(),
            user_id: "U4af4980629...".into(),
            picture_url: Some("https://obs.line-apps.com/...".into()),
            status_message: Some("Hello, LINE!".into()),
            language: Some("en".into()),
        }
    );
}

#[tokio::test]
async fn get_profile_not_found() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/profile/Uunknown"))
        .respond_with(
            ResponseTemplate::new(404)
                .insert_header("x-line-request-id", "req-404")
                .set_body_json(json!({"message": "Not found"})),
        )
        .mount(&server)
        .await;

    let err = client.get_profile("Uunknown").await.unwrap_err();
    let api = err.api_error().unwrap();
    assert_eq!(api.status, 404);
    assert_eq!(api.request_id.as_deref(), Some("req-404"));
    assert_eq!(api.body.message, "Not found");
}

#[tokio::test]
async fn get_followers_with_paging() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/followers/ids"))
        .and(query_param("start", "yANU9IA..."))
        .and(query_param("limit", "1000"))
        .and(header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "userIds": ["U4af4980629...", "U0c229f96c4...", "U95afb1d4df..."],
            "next": "yANU9IB..."
        })))
        .expect(1)
        .mount(&server)
        .await;

    let response = client
        .get_followers(Some("yANU9IA..."), Some(1000))
        .await
        .unwrap();
    assert_eq!(
        response,
        GetFollowersResponse {
            user_ids: vec![
                "U4af4980629...".into(),
                "U0c229f96c4...".into(),
                "U95afb1d4df...".into(),
            ],
            next: Some("yANU9IB...".into()),
        }
    );
}

#[tokio::test]
async fn get_followers_first_page() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/followers/ids"))
        .and(query_param_is_missing("start"))
        .and(query_param_is_missing("limit"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"userIds": []})))
        .expect(1)
        .mount(&server)
        .await;

    let response = client.get_followers(None, None).await.unwrap();
    assert!(response.user_ids.is_empty());
    assert_eq!(response.next, None);
    let requests = server.received_requests().await.unwrap();
    assert!(requests[0].body.is_empty());
}

#[tokio::test]
async fn get_bot_info() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/info"))
        .and(header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "userId": "Ub9952f8...",
            "basicId": "@216ru...",
            "premiumId": "@example",
            "displayName": "Example name",
            "pictureUrl": "https://obs.line-apps.com/...",
            "chatMode": "chat",
            "markAsReadMode": "manual"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let response = client.get_bot_info().await.unwrap();
    assert_eq!(
        response,
        BotInfoResponse {
            user_id: "Ub9952f8...".into(),
            basic_id: "@216ru...".into(),
            premium_id: Some("@example".into()),
            display_name: "Example name".into(),
            picture_url: Some("https://obs.line-apps.com/...".into()),
            chat_mode: ChatMode::Chat,
            mark_as_read_mode: MarkAsReadMode::Manual,
        }
    );
}

#[test]
fn bot_info_enum_values() {
    let modes: Vec<ChatMode> = serde_json::from_value(json!(["chat", "bot", "other"])).unwrap();
    assert_eq!(
        modes,
        vec![ChatMode::Chat, ChatMode::Bot, ChatMode::Unknown]
    );
    let modes: Vec<MarkAsReadMode> =
        serde_json::from_value(json!(["auto", "manual", "other"])).unwrap();
    assert_eq!(
        modes,
        vec![
            MarkAsReadMode::Auto,
            MarkAsReadMode::Manual,
            MarkAsReadMode::Unknown
        ]
    );
}
