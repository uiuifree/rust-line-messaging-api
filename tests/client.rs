mod common;

use line_bot_messaging_api::{Error, LineClient};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, ResponseTemplate};

#[test]
fn debug_output_redacts_the_channel_access_token() {
    let client = LineClient::new("secret-token");
    let debug = format!("{client:?}");
    assert!(!debug.contains("secret-token"));
    assert!(debug.contains("<redacted>"));
    assert!(debug.contains("https://api.line.me/"));

    let builder = LineClient::builder().channel_access_token("secret-token");
    let debug = format!("{builder:?}");
    assert!(!debug.contains("secret-token"));
    assert!(debug.contains("<redacted>"));

    let debug = format!("{:?}", LineClient::builder().build().unwrap());
    assert!(debug.contains("channel_access_token: None"));
}

#[test]
fn builder_rejects_invalid_base_urls() {
    let err = LineClient::builder()
        .api_base_url("not a url")
        .build()
        .unwrap_err();
    assert!(matches!(err, Error::InvalidBaseUrl(_)));
    assert!(err.to_string().starts_with("invalid base URL"));

    let err = LineClient::builder()
        .data_base_url("mailto:someone@example.com")
        .build()
        .unwrap_err();
    assert!(matches!(err, Error::InvalidBaseUrl(_)));
}

#[tokio::test]
async fn builder_accepts_a_custom_http_client() {
    let server = wiremock::MockServer::start().await;
    let client = LineClient::builder()
        .http_client(reqwest::Client::new())
        .channel_access_token(common::TOKEN)
        .api_base_url(format!("{}/", server.uri()))
        .build()
        .unwrap();
    Mock::given(method("GET"))
        .and(path("/v2/bot/info"))
        .and(header("authorization", "Bearer test-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "userId": "U1", "basicId": "@b", "displayName": "bot",
            "chatMode": "bot", "markAsReadMode": "auto"
        })))
        .expect(1)
        .mount(&server)
        .await;
    client.get_bot_info().await.unwrap();
}

#[tokio::test]
async fn api_error_keeps_status_request_ids_and_details() {
    let (server, client) = common::setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/info"))
        .respond_with(
            ResponseTemplate::new(409)
                .insert_header("x-line-request-id", "req-1")
                .insert_header("x-line-accepted-request-id", "req-0")
                .set_body_json(serde_json::json!({
                    "message": "The request body has 1 error(s)",
                    "details": [{"message": "must be specified", "property": "to"}]
                })),
        )
        .mount(&server)
        .await;

    let err = client.get_bot_info().await.unwrap_err();
    assert_eq!(
        err.to_string(),
        "LINE API returned 409: The request body has 1 error(s)"
    );
    let api = err.api_error().unwrap();
    assert_eq!(api.status, 409);
    assert_eq!(api.request_id.as_deref(), Some("req-1"));
    assert_eq!(api.accepted_request_id.as_deref(), Some("req-0"));
    assert_eq!(api.body.details[0].property.as_deref(), Some("to"));
}

#[tokio::test]
async fn non_json_error_body_is_kept_as_the_message() {
    let (server, client) = common::setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/info"))
        .respond_with(ResponseTemplate::new(502).set_body_string("<html>Bad Gateway</html>"))
        .mount(&server)
        .await;

    let err = client.get_bot_info().await.unwrap_err();
    let api = err.api_error().unwrap();
    assert_eq!(api.status, 502);
    assert_eq!(api.body.message, "<html>Bad Gateway</html>");
    assert!(api.request_id.is_none());
}

#[tokio::test]
async fn undecodable_success_body_is_a_decode_error() {
    let (server, client) = common::setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/info"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("x-line-request-id", "req-decode")
                .set_body_string("{\"unexpected\":true}"),
        )
        .mount(&server)
        .await;

    let err = client.get_bot_info().await.unwrap_err();
    assert!(err.api_error().is_none());
    match err {
        Error::Decode {
            body, request_id, ..
        } => {
            assert_eq!(body, "{\"unexpected\":true}");
            assert_eq!(request_id.as_deref(), Some("req-decode"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

/// Returns a base URL on which nothing is listening.
fn closed_port_url() -> String {
    // Bind and immediately release a port so nothing is listening on it.
    let port = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    format!("http://127.0.0.1:{port}")
}

#[tokio::test]
async fn connection_failure_is_an_http_error() {
    let url = closed_port_url();
    let client = LineClient::builder()
        .api_base_url(&url)
        .data_base_url(&url)
        .build()
        .unwrap();

    let err = client.get_bot_info().await.unwrap_err();
    assert!(matches!(err, Error::Http(_)));
    assert!(err.to_string().starts_with("HTTP request failed"));

    // The paths that skip the body and that return binary content fail the same way.
    let err = client.leave_group("C1").await.unwrap_err();
    assert!(matches!(err, Error::Http(_)));
    let err = client.get_message_content("1").await.unwrap_err();
    assert!(matches!(err, Error::Http(_)));
}

/// Serves one connection that announces a longer body than it sends, then closes,
/// so reading the body fails after the status line was received.
async fn truncated_body_server(status: &str) -> String {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let response = format!(
        "HTTP/1.1 {status}\r\nx-line-request-id: req-truncated\r\ncontent-length: 100\r\n\r\n{{\"a\""
    );
    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buf = [0u8; 4096];
        let _ = socket.read(&mut buf).await.unwrap();
        socket.write_all(response.as_bytes()).await.unwrap();
    });
    url
}

#[tokio::test]
async fn truncated_success_body_is_an_http_error() {
    let url = truncated_body_server("200 OK").await;
    let client = LineClient::builder().api_base_url(url).build().unwrap();
    let err = client.get_bot_info().await.unwrap_err();
    assert!(matches!(err, Error::Http(_)));
}

#[tokio::test]
async fn truncated_binary_body_is_an_http_error() {
    let url = truncated_body_server("200 OK").await;
    let client = LineClient::builder().data_base_url(url).build().unwrap();
    let err = client.get_message_content("1").await.unwrap_err();
    assert!(matches!(err, Error::Http(_)));
}

#[tokio::test]
async fn truncated_error_body_keeps_status_and_request_id() {
    let url = truncated_body_server("500 Internal Server Error").await;
    let client = LineClient::builder().api_base_url(url).build().unwrap();
    let err = client.get_bot_info().await.unwrap_err();
    let api = err.api_error().expect("API error");
    assert_eq!(api.status, 500);
    assert_eq!(api.request_id.as_deref(), Some("req-truncated"));
    assert!(
        api.body
            .message
            .starts_with("failed to read the error response body"),
        "{}",
        api.body.message
    );
}

#[tokio::test]
async fn json_error_body_that_is_not_a_line_error_is_kept_raw() {
    let (server, client) = common::setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/info"))
        .respond_with(ResponseTemplate::new(503).set_body_string("{\"status\":\"down\"}"))
        .mount(&server)
        .await;

    let err = client.get_bot_info().await.unwrap_err();
    assert_eq!(
        err.api_error().unwrap().body.message,
        "{\"status\":\"down\"}"
    );
}

#[tokio::test]
async fn empty_error_body_has_an_empty_summary() {
    let (server, client) = common::setup().await;
    Mock::given(method("GET"))
        .and(path("/v2/bot/info"))
        .respond_with(ResponseTemplate::new(502))
        .mount(&server)
        .await;

    let err = client.get_bot_info().await.unwrap_err();
    assert_eq!(err.api_error().unwrap().body.summary(), "");
    assert_eq!(err.to_string(), "LINE API returned 502: ");
}
