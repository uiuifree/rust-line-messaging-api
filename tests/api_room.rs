mod common;

use common::setup;
use line_bot_messaging_api::api::*;
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param, query_param_is_missing};
use wiremock::{Mock, ResponseTemplate};

const AUTH: &str = "Bearer test-token";

#[tokio::test]
async fn get_room_member_profile_escapes_ids() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/room/Ra8%2Fdbf4673c/member/U4af%204980629"))
        .and(header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "displayName": "LINE taro",
            "userId": "U4af4980629...",
            "pictureUrl": "https://obs.line-apps.com/..."
        })))
        .expect(1)
        .mount(&server)
        .await;

    let response = client
        .get_room_member_profile("Ra8/dbf4673c", "U4af 4980629")
        .await
        .unwrap();
    assert_eq!(
        response,
        RoomUserProfileResponse {
            display_name: "LINE taro".into(),
            user_id: "U4af4980629...".into(),
            picture_url: Some("https://obs.line-apps.com/...".into()),
        }
    );
}

#[tokio::test]
async fn get_room_members_ids_with_start() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/room/Ra8dbf4673c/members/ids"))
        .and(query_param("start", "jxEWCEEP..."))
        .and(header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "memberIds": ["U4af4980629...", "U0c229f96c4..."],
            "next": "jxEWCEEQ..."
        })))
        .expect(1)
        .mount(&server)
        .await;

    let response = client
        .get_room_members_ids("Ra8dbf4673c", Some("jxEWCEEP..."))
        .await
        .unwrap();
    assert_eq!(
        response,
        MembersIdsResponse {
            member_ids: vec!["U4af4980629...".into(), "U0c229f96c4...".into()],
            next: Some("jxEWCEEQ...".into()),
        }
    );
}

#[tokio::test]
async fn get_room_members_ids_first_page() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/room/Ra8dbf4673c/members/ids"))
        .and(query_param_is_missing("start"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"memberIds": []})))
        .expect(1)
        .mount(&server)
        .await;

    let response = client
        .get_room_members_ids("Ra8dbf4673c", None)
        .await
        .unwrap();
    assert!(response.member_ids.is_empty());
}

#[tokio::test]
async fn leave_room() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/room/Ra8dbf4673c/leave"))
        .and(header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    client.leave_room("Ra8dbf4673c").await.unwrap();
}

#[tokio::test]
async fn get_room_member_count() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/room/Ra8dbf4673c/members/count"))
        .and(header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"count": 3})))
        .expect(1)
        .mount(&server)
        .await;

    let response = client.get_room_member_count("Ra8dbf4673c").await.unwrap();
    assert_eq!(response, RoomMemberCountResponse { count: 3 });
}

#[tokio::test]
async fn get_room_member_count_forbidden() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/room/Ra8dbf4673c/members/count"))
        .respond_with(
            ResponseTemplate::new(403)
                .insert_header("x-line-request-id", "req-403")
                .set_body_json(
                    json!({"message": "Access to this API is not available for your account"}),
                ),
        )
        .mount(&server)
        .await;

    let err = client
        .get_room_member_count("Ra8dbf4673c")
        .await
        .unwrap_err();
    let api = err.api_error().unwrap();
    assert_eq!(api.status, 403);
    assert_eq!(api.request_id.as_deref(), Some("req-403"));
    assert_eq!(
        api.body.message,
        "Access to this API is not available for your account"
    );
}
