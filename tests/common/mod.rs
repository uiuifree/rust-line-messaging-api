#![allow(dead_code)]

use line_bot_messaging_api::LineClient;
use wiremock::MockServer;

pub const TOKEN: &str = "test-token";

/// Starts a mock server and returns a client whose both base URLs point at it.
pub async fn setup() -> (MockServer, LineClient) {
    let server = MockServer::start().await;
    let client = LineClient::builder()
        .channel_access_token(TOKEN)
        .api_base_url(server.uri())
        .data_base_url(server.uri())
        .build()
        .unwrap();
    (server, client)
}

/// Like [`setup`], but the data host (`api-data.line.me`) is a separate server,
/// so tests can assert which host an endpoint uses.
pub async fn setup_with_data_host() -> (MockServer, MockServer, LineClient) {
    let api = MockServer::start().await;
    let data = MockServer::start().await;
    let client = LineClient::builder()
        .channel_access_token(TOKEN)
        .api_base_url(api.uri())
        .data_base_url(data.uri())
        .build()
        .unwrap();
    (api, data, client)
}
