mod common;

use common::setup;
use line_bot_messaging_api::api::*;
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param, query_param_is_missing};
use wiremock::{Mock, ResponseTemplate};

const AUTH: &str = "Bearer test-token";

#[tokio::test]
async fn get_group_member_profile_escapes_ids() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/group/Ca56%2Ff94637c/member/U4af%204980629"))
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
        .get_group_member_profile("Ca56/f94637c", "U4af 4980629")
        .await
        .unwrap();
    assert_eq!(
        response,
        GroupUserProfileResponse {
            display_name: "LINE taro".into(),
            user_id: "U4af4980629...".into(),
            picture_url: Some("https://obs.line-apps.com/...".into()),
        }
    );
}

#[tokio::test]
async fn get_group_members_ids_with_start() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/group/Ca56f94637c/members/ids"))
        .and(query_param("start", "jxEWCEEP..."))
        .and(header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "memberIds": ["U4af4980629...", "U0c229f96c4...", "U95afb1d4df..."],
            "next": "jxEWCEEQ..."
        })))
        .expect(1)
        .mount(&server)
        .await;

    let response = client
        .get_group_members_ids("Ca56f94637c", Some("jxEWCEEP..."))
        .await
        .unwrap();
    assert_eq!(
        response,
        MembersIdsResponse {
            member_ids: vec![
                "U4af4980629...".into(),
                "U0c229f96c4...".into(),
                "U95afb1d4df...".into(),
            ],
            next: Some("jxEWCEEQ...".into()),
        }
    );
}

#[tokio::test]
async fn get_group_members_ids_first_page() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/group/Ca56f94637c/members/ids"))
        .and(query_param_is_missing("start"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"memberIds": ["U1"]})))
        .expect(1)
        .mount(&server)
        .await;

    let response = client
        .get_group_members_ids("Ca56f94637c", None)
        .await
        .unwrap();
    assert_eq!(response.member_ids, vec!["U1".to_owned()]);
    assert_eq!(response.next, None);
}

#[tokio::test]
async fn leave_group() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/group/Ca56f94637c/leave"))
        .and(header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    client.leave_group("Ca56f94637c").await.unwrap();
}

#[tokio::test]
async fn leave_group_not_found() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/group/Cunknown/leave"))
        .respond_with(
            ResponseTemplate::new(404)
                .insert_header("x-line-request-id", "req-404")
                .set_body_json(json!({"message": "Not found"})),
        )
        .mount(&server)
        .await;

    let err = client.leave_group("Cunknown").await.unwrap_err();
    let api = err.api_error().unwrap();
    assert_eq!(api.status, 404);
    assert_eq!(api.request_id.as_deref(), Some("req-404"));
    assert_eq!(api.body.message, "Not found");
}

#[tokio::test]
async fn get_group_summary() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/group/Ca56f94637c/summary"))
        .and(header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "groupId": "Ca56f94637c...",
            "groupName": "Group name",
            "pictureUrl": "https://profile.line-scdn.net/abcdefghijklmn"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let response = client.get_group_summary("Ca56f94637c").await.unwrap();
    assert_eq!(
        response,
        GroupSummaryResponse {
            group_id: "Ca56f94637c...".into(),
            group_name: "Group name".into(),
            picture_url: Some("https://profile.line-scdn.net/abcdefghijklmn".into()),
        }
    );
}

#[tokio::test]
async fn get_group_member_count() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/group/Ca56f94637c/members/count"))
        .and(header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"count": 3})))
        .expect(1)
        .mount(&server)
        .await;

    let response = client.get_group_member_count("Ca56f94637c").await.unwrap();
    assert_eq!(response, GroupMemberCountResponse { count: 3 });
}
