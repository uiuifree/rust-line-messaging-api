mod common;

use common::setup;
use line_bot_messaging_api::api::*;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn issue_link_token_escapes_user_id() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/user/U4af%2F4980629/linkToken"))
        .and(header("authorization", "Bearer test-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "linkToken": "NMZTNuVrPTqlr2IF8Bnymkb7rXfYv5EY"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let response = client.issue_link_token("U4af/4980629").await.unwrap();
    assert_eq!(
        response,
        IssueLinkTokenResponse {
            link_token: "NMZTNuVrPTqlr2IF8Bnymkb7rXfYv5EY".into(),
        }
    );
    let requests = server.received_requests().await.unwrap();
    assert!(requests[0].body.is_empty());
}

#[tokio::test]
async fn issue_link_token_bad_request() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/user/Uunknown/linkToken"))
        .respond_with(
            ResponseTemplate::new(400)
                .insert_header("x-line-request-id", "req-400")
                .set_body_json(json!({"message": "Failed to issue link token"})),
        )
        .mount(&server)
        .await;

    let err = client.issue_link_token("Uunknown").await.unwrap_err();
    let api = err.api_error().unwrap();
    assert_eq!(api.status, 400);
    assert_eq!(api.request_id.as_deref(), Some("req-400"));
    assert_eq!(api.body.message, "Failed to issue link token");
}
