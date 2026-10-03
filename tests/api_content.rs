mod common;

use common::setup_with_data_host;
use line_bot_messaging_api::Content;
use line_bot_messaging_api::api::*;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, ResponseTemplate};

const AUTH: &str = "Bearer test-token";

#[tokio::test]
async fn get_message_content_uses_data_host() {
    let (api, data, client) = setup_with_data_host().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/message/325708%2F1%201/content"))
        .and(header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_raw(vec![0xff, 0xd8, 0xff], "image/jpeg"))
        .expect(1)
        .mount(&data)
        .await;

    let content = client.get_message_content("325708/1 1").await.unwrap();
    assert_eq!(
        content,
        Content {
            data: vec![0xff, 0xd8, 0xff],
            content_type: Some("image/jpeg".into()),
        }
    );
    assert!(api.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn get_message_content_preview_uses_data_host() {
    let (api, data, client) = setup_with_data_host().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/message/325708/content/preview"))
        .and(header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_raw(vec![0x89, 0x50], "image/png"))
        .expect(1)
        .mount(&data)
        .await;

    let content = client.get_message_content_preview("325708").await.unwrap();
    assert_eq!(content.data, vec![0x89, 0x50]);
    assert_eq!(content.content_type.as_deref(), Some("image/png"));
    assert!(api.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn get_message_content_transcoding_uses_data_host() {
    let (api, data, client) = setup_with_data_host().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/message/325708/content/transcoding"))
        .and(header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "processing"})))
        .expect(1)
        .mount(&data)
        .await;

    let response = client
        .get_message_content_transcoding_by_message_id("325708")
        .await
        .unwrap();
    assert_eq!(
        response,
        GetMessageContentTranscodingResponse {
            status: TranscodingStatus::Processing,
        }
    );
    assert!(api.received_requests().await.unwrap().is_empty());
}

#[test]
fn transcoding_status_values() {
    let statuses: Vec<TranscodingStatus> =
        serde_json::from_value(json!(["processing", "succeeded", "failed", "expired"])).unwrap();
    assert_eq!(
        statuses,
        vec![
            TranscodingStatus::Processing,
            TranscodingStatus::Succeeded,
            TranscodingStatus::Failed,
            TranscodingStatus::Unknown,
        ]
    );
}

#[tokio::test]
async fn get_message_content_not_found() {
    let (_api, data, client) = setup_with_data_host().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/message/325708/content"))
        .respond_with(
            ResponseTemplate::new(404)
                .insert_header("x-line-request-id", "req-404")
                .set_body_json(json!({"message": "Not found"})),
        )
        .mount(&data)
        .await;

    let err = client.get_message_content("325708").await.unwrap_err();
    let api = err.api_error().unwrap();
    assert_eq!(api.status, 404);
    assert_eq!(api.request_id.as_deref(), Some("req-404"));
    assert_eq!(api.body.message, "Not found");
}
