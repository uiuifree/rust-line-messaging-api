mod common;

use common::setup;
use line_bot_messaging_api::Error;
use line_bot_messaging_api::api::*;
use serde_json::json;
use wiremock::matchers::{body_string, header, method, path, query_param};
use wiremock::{Match, Mock, Request, ResponseTemplate};

const FORM: &str = "application/x-www-form-urlencoded";
const ENCODED_ASSERTION_TYPE: &str =
    "urn%3Aietf%3Aparams%3Aoauth%3Aclient-assertion-type%3Ajwt-bearer";

/// Matches requests without an `Authorization` header.
struct NoAuthorization;

impl Match for NoAuthorization {
    fn matches(&self, request: &Request) -> bool {
        !request.headers.contains_key("authorization")
    }
}

#[tokio::test]
async fn issue_stateless_channel_token_by_jwt_assertion() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/oauth2/v3/token"))
        .and(NoAuthorization)
        .and(header("content-type", FORM))
        .and(body_string(format!(
            "grant_type=client_credentials&client_assertion_type={ENCODED_ASSERTION_TYPE}&client_assertion=eyJ.jwt"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "token_type": "Bearer",
            "access_token": "eyJhbGciOiJIUz.....",
            "expires_in": 899
        })))
        .expect(1)
        .mount(&server)
        .await;

    let res = client
        .issue_stateless_channel_token(&IssueStatelessChannelTokenRequest::jwt_assertion("eyJ.jwt"))
        .await
        .unwrap();
    assert_eq!(
        res,
        IssueStatelessChannelAccessTokenResponse {
            access_token: "eyJhbGciOiJIUz.....".into(),
            expires_in: 899,
            token_type: "Bearer".into(),
        }
    );
}

#[tokio::test]
async fn issue_stateless_channel_token_by_client_secret() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/oauth2/v3/token"))
        .and(NoAuthorization)
        .and(header("content-type", FORM))
        .and(body_string(
            "grant_type=client_credentials&client_id=1234&client_secret=s%26cret",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "token_type": "Bearer",
            "access_token": "token",
            "expires_in": 900
        })))
        .expect(1)
        .mount(&server)
        .await;

    let res = client
        .issue_stateless_channel_token(&IssueStatelessChannelTokenRequest::client_secret(
            "1234", "s&cret",
        ))
        .await
        .unwrap();
    assert_eq!(res.access_token, "token");
}

#[tokio::test]
async fn gets_all_valid_channel_access_token_key_ids() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/oauth2/v2.1/tokens/kid"))
        .and(NoAuthorization)
        .and(query_param(
            "client_assertion_type",
            "urn:ietf:params:oauth:client-assertion-type:jwt-bearer",
        ))
        .and(query_param("client_assertion", "eyJ.jwt"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "kids": [
                "U_gdnFYKTWRxxxxDVZexGg",
                "sDTOzw5wIfWxxxxzcmeQA",
                "73hDyp3PxGfxxxxD6U5qYA",
                "FHGanaP79smDxxxxyPrVw",
                "CguB-0kxxxxdSM3A5Q_UtQ",
                "G82YP96jhHwyKSxxxx7IFA"
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let res = client
        .gets_all_valid_channel_access_token_key_ids("eyJ.jwt")
        .await
        .unwrap();
    assert_eq!(res.kids.len(), 6);
    assert_eq!(res.kids[0], "U_gdnFYKTWRxxxxDVZexGg");
}

#[tokio::test]
async fn issue_channel_token_by_jwt() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/oauth2/v2.1/token"))
        .and(NoAuthorization)
        .and(header("content-type", FORM))
        .and(body_string(format!(
            "grant_type=client_credentials&client_assertion_type={ENCODED_ASSERTION_TYPE}&client_assertion=eyJ.jwt"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "access_token": "eyJhbGciOiJIUz.....",
            "token_type": "Bearer",
            "expires_in": 2592000,
            "key_id": "sDTOzw5wIfxxxxPEzcmeQA"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let res = client.issue_channel_token_by_jwt("eyJ.jwt").await.unwrap();
    assert_eq!(
        res,
        IssueChannelAccessTokenResponse {
            access_token: "eyJhbGciOiJIUz.....".into(),
            expires_in: 2592000,
            token_type: "Bearer".into(),
            key_id: "sDTOzw5wIfxxxxPEzcmeQA".into(),
        }
    );
}

#[tokio::test]
async fn verify_channel_token_by_jwt() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/oauth2/v2.1/verify"))
        .and(NoAuthorization)
        .and(query_param("access_token", "eyJ.token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "client_id": "1573163733",
            "expires_in": 2591659,
            "scope": "profile chat_message.write"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let res = client
        .verify_channel_token_by_jwt("eyJ.token")
        .await
        .unwrap();
    assert_eq!(
        res,
        VerifyChannelAccessTokenResponse {
            client_id: "1573163733".into(),
            expires_in: 2591659,
            scope: Some("profile chat_message.write".into()),
        }
    );
}

#[tokio::test]
async fn revoke_channel_token_by_jwt() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/oauth2/v2.1/revoke"))
        .and(NoAuthorization)
        .and(header("content-type", FORM))
        .and(body_string(
            "client_id=1234&client_secret=secret&access_token=eyJ.token",
        ))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    client
        .revoke_channel_token_by_jwt("1234", "secret", "eyJ.token")
        .await
        .unwrap();
}

#[tokio::test]
async fn issue_channel_token() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/oauth/accessToken"))
        .and(NoAuthorization)
        .and(header("content-type", FORM))
        .and(body_string(
            "grant_type=client_credentials&client_id=1234&client_secret=secret",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "access_token": "W1TeHCgfH2Liwa.....",
            "expires_in": 2592000,
            "token_type": "Bearer"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let res = client.issue_channel_token("1234", "secret").await.unwrap();
    assert_eq!(
        res,
        IssueShortLivedChannelAccessTokenResponse {
            access_token: "W1TeHCgfH2Liwa.....".into(),
            expires_in: 2592000,
            token_type: "Bearer".into(),
        }
    );
}

#[tokio::test]
async fn verify_channel_token() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/oauth/verify"))
        .and(NoAuthorization)
        .and(header("content-type", FORM))
        .and(body_string("access_token=W1TeHCgfH2Liwa"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "client_id": "1350031035",
            "expires_in": 3138007490u64,
            "scope": "P CM"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let res = client.verify_channel_token("W1TeHCgfH2Liwa").await.unwrap();
    assert_eq!(res.client_id, "1350031035");
    assert_eq!(res.expires_in, 3138007490);
    assert_eq!(res.scope.as_deref(), Some("P CM"));
}

#[test]
fn verify_response_without_scope_omits_it() {
    let res = VerifyChannelAccessTokenResponse {
        client_id: "1".into(),
        expires_in: 10,
        scope: None,
    };
    assert_eq!(
        serde_json::to_value(res).unwrap(),
        json!({"client_id": "1", "expires_in": 10})
    );
}

#[tokio::test]
async fn revoke_channel_token() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/oauth/revoke"))
        .and(NoAuthorization)
        .and(header("content-type", FORM))
        .and(body_string("access_token=W1TeHCgfH2Liwa"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    client.revoke_channel_token("W1TeHCgfH2Liwa").await.unwrap();
}

#[tokio::test]
async fn channel_access_token_error_parses_oauth_error_body() {
    let (server, client) = setup().await;
    let body =
        r#"{"error":"invalid_request","error_description":"some parameters missed or invalid"}"#;
    Mock::given(method("POST"))
        .and(path("/v2/oauth/accessToken"))
        .respond_with(
            ResponseTemplate::new(400)
                .insert_header("x-line-request-id", "req-400")
                .set_body_raw(body, "application/json"),
        )
        .mount(&server)
        .await;

    let err = client.issue_channel_token("1234", "bad").await.unwrap_err();
    let Error::Api(api) = err else {
        panic!("expected Error::Api, got {err:?}");
    };
    assert_eq!(api.status, 400);
    assert_eq!(api.request_id.as_deref(), Some("req-400"));
    assert!(api.body.message.is_empty());
    assert_eq!(api.body.error.as_deref(), Some("invalid_request"));
    assert_eq!(api.body.summary(), "some parameters missed or invalid");
    assert!(api.body.details.is_empty());
}

#[tokio::test]
async fn oauth_error_body_is_typed() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/oauth/accessToken"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "error": "invalid_client",
            "error_description": "invalid client_id"
        })))
        .mount(&server)
        .await;

    let err = client
        .issue_channel_token("1234567890", "secret")
        .await
        .unwrap_err();
    assert_eq!(err.to_string(), "LINE API returned 400: invalid client_id");
    let body = &err.api_error().unwrap().body;
    assert_eq!(body.error.as_deref(), Some("invalid_client"));
    assert_eq!(body.error_description.as_deref(), Some("invalid client_id"));
    assert!(body.message.is_empty());
}

#[tokio::test]
async fn oauth_error_without_description_shows_the_error_code() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/v2/oauth/accessToken"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({"error": "invalid_request"})))
        .mount(&server)
        .await;

    let err = client
        .issue_channel_token("1234567890", "secret")
        .await
        .unwrap_err();
    assert_eq!(err.to_string(), "LINE API returned 400: invalid_request");
}

#[test]
fn debug_output_redacts_credentials_and_tokens() {
    let request = IssueStatelessChannelTokenRequest::client_secret("1234567890", "channel-secret");
    let debug = format!("{request:?}");
    assert!(debug.contains("1234567890"));
    assert!(!debug.contains("channel-secret"));

    let request = IssueStatelessChannelTokenRequest::jwt_assertion("signed-jwt");
    assert!(!format!("{request:?}").contains("signed-jwt"));

    let stateless: IssueStatelessChannelAccessTokenResponse = serde_json::from_value(json!({
        "access_token": "token-a", "expires_in": 900, "token_type": "Bearer"
    }))
    .unwrap();
    let v21: IssueChannelAccessTokenResponse = serde_json::from_value(json!({
        "access_token": "token-b", "expires_in": 2592000, "token_type": "Bearer", "key_id": "kid-1"
    }))
    .unwrap();
    let short: IssueShortLivedChannelAccessTokenResponse = serde_json::from_value(json!({
        "access_token": "token-c", "expires_in": 2592000, "token_type": "Bearer"
    }))
    .unwrap();
    for (debug, token) in [
        (format!("{stateless:?}"), "token-a"),
        (format!("{v21:?}"), "token-b"),
        (format!("{short:?}"), "token-c"),
    ] {
        assert!(!debug.contains(token), "{debug}");
        assert!(debug.contains("<redacted>"), "{debug}");
        assert!(debug.contains("Bearer"), "{debug}");
    }
    assert!(format!("{v21:?}").contains("kid-1"));
}

#[tokio::test]
async fn http_error_does_not_leak_credentials_in_the_url() {
    // Bind and release a port so that the request fails to connect.
    let port = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let client = line_bot_messaging_api::LineClient::builder()
        .api_base_url(format!("http://127.0.0.1:{port}"))
        .build()
        .unwrap();

    let err = client
        .gets_all_valid_channel_access_token_key_ids("secret-jwt")
        .await
        .unwrap_err();
    assert!(matches!(err, Error::Http(_)));
    assert!(!err.to_string().contains("secret-jwt"), "{err}");
    assert!(!format!("{err:?}").contains("secret-jwt"), "{err:?}");
}
