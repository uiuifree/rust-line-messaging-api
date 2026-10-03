mod common;

use common::setup;
use line_bot_messaging_api::ErrorDetail;
use line_bot_messaging_api::api::*;
use serde_json::json;
use wiremock::matchers::{body_json, header, method, path};
use wiremock::{Mock, ResponseTemplate};

const AUTH: &str = "Bearer test-token";

#[tokio::test]
async fn get_webhook_endpoint() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/channel/webhook/endpoint"))
        .and(header("authorization", AUTH))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "endpoint": "https://example.com/test",
            "active": true
        })))
        .expect(1)
        .mount(&server)
        .await;

    let response = client.get_webhook_endpoint().await.unwrap();
    assert_eq!(
        response,
        GetWebhookEndpointResponse {
            endpoint: "https://example.com/test".into(),
            active: true,
        }
    );
}

#[tokio::test]
async fn set_webhook_endpoint() {
    let (server, client) = setup().await;
    Mock::given(method("PUT"))
        .and(path("/v2/bot/channel/webhook/endpoint"))
        .and(header("authorization", AUTH))
        .and(body_json(json!({"endpoint": "https://example.com/hoge"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    let request = SetWebhookEndpointRequest::new("https://example.com/hoge");
    client.set_webhook_endpoint(&request).await.unwrap();
}

#[tokio::test]
async fn set_webhook_endpoint_rejects_invalid_url() {
    let (server, client) = setup().await;
    Mock::given(method("PUT"))
        .and(path("/v2/bot/channel/webhook/endpoint"))
        .respond_with(
            ResponseTemplate::new(400)
                .insert_header("x-line-request-id", "req-invalid")
                .set_body_json(json!({
                    "message": "Invalid webhook endpoint URL",
                    "details": [{"message": "must be https", "property": "endpoint"}]
                })),
        )
        .mount(&server)
        .await;

    let request = SetWebhookEndpointRequest::new("http://example.com");
    let err = client.set_webhook_endpoint(&request).await.unwrap_err();
    let api = err.api_error().unwrap();
    assert_eq!(api.status, 400);
    assert_eq!(api.request_id.as_deref(), Some("req-invalid"));
    assert_eq!(api.body.message, "Invalid webhook endpoint URL");
    assert_eq!(
        api.body.details,
        vec![ErrorDetail {
            message: Some("must be https".into()),
            property: Some("endpoint".into()),
        }]
    );
}

#[tokio::test]
async fn test_webhook_endpoint_with_url() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/channel/webhook/test"))
        .and(header("authorization", AUTH))
        .and(body_json(
            json!({"endpoint": "https://example.com/webhook"}),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "success": true,
            "timestamp": "2020-09-30T05:38:20.031Z",
            "statusCode": 200,
            "reason": "OK",
            "detail": "200"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let request = TestWebhookEndpointRequest::default().endpoint("https://example.com/webhook");
    let response = client.test_webhook_endpoint(&request).await.unwrap();
    assert_eq!(
        response,
        TestWebhookEndpointResponse {
            success: Some(true),
            timestamp: "2020-09-30T05:38:20.031Z".into(),
            status_code: 200,
            reason: "OK".into(),
            detail: "200".into(),
        }
    );
}

#[tokio::test]
async fn test_webhook_endpoint_without_url_sends_empty_object() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/channel/webhook/test"))
        .and(body_json(json!({})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "success": false,
            "timestamp": "2020-09-30T05:38:20.031Z",
            "statusCode": 404,
            "reason": "ERROR_STATUS_CODE",
            "detail": "Not Found"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let response = client
        .test_webhook_endpoint(&TestWebhookEndpointRequest::default())
        .await
        .unwrap();
    assert_eq!(response.success, Some(false));
    assert_eq!(response.status_code, 404);
}

#[tokio::test]
async fn test_webhook_endpoint_decodes_negative_status_code() {
    // The spec sets statusCode to zero or a negative number when no response was received.
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/bot/channel/webhook/test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "success": false,
            "timestamp": "2020-09-30T05:38:20.031Z",
            "statusCode": -1,
            "reason": "COULD_NOT_CONNECT",
            "detail": "Connection refused"
        })))
        .mount(&server)
        .await;

    let response = client
        .test_webhook_endpoint(&TestWebhookEndpointRequest::default())
        .await
        .unwrap();
    assert_eq!(response.status_code, -1);
    assert_eq!(response.reason, "COULD_NOT_CONNECT");
}
