use std::fmt;

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::client::{Host, LineClient};
use crate::error::Result;

/// Printed by `Debug` in place of credentials and tokens.
const REDACTED: &str = "<redacted>";

const GRANT_TYPE: &str = "client_credentials";
const CLIENT_ASSERTION_TYPE: &str = "urn:ietf:params:oauth:client-assertion-type:jwt-bearer";

// These endpoints authenticate with the form body, so they never send the
// channel access token in the `Authorization` header.
impl LineClient {
    /// Issues a stateless channel access token (valid for 15 minutes).
    /// <https://developers.line.biz/en/reference/messaging-api/#issue-stateless-channel-access-token>
    pub async fn issue_stateless_channel_token(
        &self,
        request: &IssueStatelessChannelTokenRequest,
    ) -> Result<IssueStatelessChannelAccessTokenResponse> {
        let form = match request {
            IssueStatelessChannelTokenRequest::JwtAssertion { client_assertion } => vec![
                ("grant_type", GRANT_TYPE),
                ("client_assertion_type", CLIENT_ASSERTION_TYPE),
                ("client_assertion", client_assertion.as_str()),
            ],
            IssueStatelessChannelTokenRequest::ClientSecret {
                client_id,
                client_secret,
            } => vec![
                ("grant_type", GRANT_TYPE),
                ("client_id", client_id.as_str()),
                ("client_secret", client_secret.as_str()),
            ],
        };
        let rb = self.request_without_auth(Method::POST, Host::Api, &["oauth2", "v3", "token"]);
        self.send_json(rb.form(&form)).await
    }

    /// Gets all valid channel access token key IDs (v2.1).
    /// <https://developers.line.biz/en/reference/messaging-api/#get-all-valid-channel-access-token-key-ids-v2-1>
    pub async fn gets_all_valid_channel_access_token_key_ids(
        &self,
        client_assertion: &str,
    ) -> Result<ChannelAccessTokenKeyIdsResponse> {
        let rb = self
            .request_without_auth(Method::GET, Host::Api, &["oauth2", "v2.1", "tokens", "kid"])
            .query(&[
                ("client_assertion_type", CLIENT_ASSERTION_TYPE),
                ("client_assertion", client_assertion),
            ]);
        self.send_json(rb).await
    }

    /// Issues a channel access token v2.1 with a JWT signed by the assertion signing key.
    /// <https://developers.line.biz/en/reference/messaging-api/#issue-channel-access-token-v2-1>
    pub async fn issue_channel_token_by_jwt(
        &self,
        client_assertion: &str,
    ) -> Result<IssueChannelAccessTokenResponse> {
        let rb = self
            .request_without_auth(Method::POST, Host::Api, &["oauth2", "v2.1", "token"])
            .form(&[
                ("grant_type", GRANT_TYPE),
                ("client_assertion_type", CLIENT_ASSERTION_TYPE),
                ("client_assertion", client_assertion),
            ]);
        self.send_json(rb).await
    }

    /// Verifies a channel access token v2.1.
    /// <https://developers.line.biz/en/reference/messaging-api/#verify-channel-access-token-v2-1>
    pub async fn verify_channel_token_by_jwt(
        &self,
        access_token: &str,
    ) -> Result<VerifyChannelAccessTokenResponse> {
        let rb = self
            .request_without_auth(Method::GET, Host::Api, &["oauth2", "v2.1", "verify"])
            .query(&[("access_token", access_token)]);
        self.send_json(rb).await
    }

    /// Revokes a channel access token v2.1.
    /// <https://developers.line.biz/en/reference/messaging-api/#revoke-channel-access-token-v2-1>
    pub async fn revoke_channel_token_by_jwt(
        &self,
        client_id: &str,
        client_secret: &str,
        access_token: &str,
    ) -> Result<()> {
        let rb = self
            .request_without_auth(Method::POST, Host::Api, &["oauth2", "v2.1", "revoke"])
            .form(&[
                ("client_id", client_id),
                ("client_secret", client_secret),
                ("access_token", access_token),
            ]);
        self.send_empty(rb).await
    }

    /// Issues a short-lived channel access token (valid for 30 days).
    /// <https://developers.line.biz/en/reference/messaging-api/#issue-shortlived-channel-access-token>
    pub async fn issue_channel_token(
        &self,
        client_id: &str,
        client_secret: &str,
    ) -> Result<IssueShortLivedChannelAccessTokenResponse> {
        let rb = self
            .request_without_auth(Method::POST, Host::Api, &["v2", "oauth", "accessToken"])
            .form(&[
                ("grant_type", GRANT_TYPE),
                ("client_id", client_id),
                ("client_secret", client_secret),
            ]);
        self.send_json(rb).await
    }

    /// Verifies a short-lived or long-lived channel access token.
    /// <https://developers.line.biz/en/reference/messaging-api/#verify-channel-access-token>
    pub async fn verify_channel_token(
        &self,
        access_token: &str,
    ) -> Result<VerifyChannelAccessTokenResponse> {
        let rb = self
            .request_without_auth(Method::POST, Host::Api, &["v2", "oauth", "verify"])
            .form(&[("access_token", access_token)]);
        self.send_json(rb).await
    }

    /// Revokes a short-lived or long-lived channel access token.
    /// <https://developers.line.biz/en/reference/messaging-api/#revoke-longlived-or-shortlived-channel-access-token>
    pub async fn revoke_channel_token(&self, access_token: &str) -> Result<()> {
        let rb = self
            .request_without_auth(Method::POST, Host::Api, &["v2", "oauth", "revoke"])
            .form(&[("access_token", access_token)]);
        self.send_empty(rb).await
    }
}

/// Credentials for [`LineClient::issue_stateless_channel_token`].
/// <https://developers.line.biz/en/reference/messaging-api/#issue-stateless-channel-access-token>
#[derive(Clone, PartialEq, Eq)]
pub enum IssueStatelessChannelTokenRequest {
    /// `IssueStatelessChannelTokenByJWTAssertionRequest`.
    JwtAssertion {
        /// JWT signed with the private key of the assertion signing key.
        client_assertion: String,
    },
    /// `IssueStatelessChannelTokenByClientSecretRequest`.
    ClientSecret {
        /// Channel ID.
        client_id: String,
        /// Channel secret.
        client_secret: String,
    },
}

impl IssueStatelessChannelTokenRequest {
    /// Authenticates with a JWT signed with the private key of the assertion signing key.
    pub fn jwt_assertion(client_assertion: impl Into<String>) -> Self {
        Self::JwtAssertion {
            client_assertion: client_assertion.into(),
        }
    }

    /// Authenticates with the channel ID and channel secret.
    pub fn client_secret(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self::ClientSecret {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        }
    }
}

/// Issued stateless channel access token.
/// <https://developers.line.biz/en/reference/messaging-api/#issue-stateless-channel-access-token>
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IssueStatelessChannelAccessTokenResponse {
    /// Opaque stateless channel access token.
    pub access_token: String,
    /// Seconds until the token expires.
    pub expires_in: u32,
    /// Always `Bearer`.
    pub token_type: String,
}

/// Channel access token key IDs.
/// <https://developers.line.biz/en/reference/messaging-api/#get-all-valid-channel-access-token-key-ids-v2-1>
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ChannelAccessTokenKeyIdsResponse {
    /// Channel access token key IDs.
    pub kids: Vec<String>,
}

/// Issued channel access token v2.1.
/// <https://developers.line.biz/en/reference/messaging-api/#issue-channel-access-token-v2-1>
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IssueChannelAccessTokenResponse {
    /// Channel access token.
    pub access_token: String,
    /// Seconds from issue to expiration.
    pub expires_in: u32,
    /// Token type (`Bearer` by default).
    pub token_type: String,
    /// Key ID identifying the channel access token.
    pub key_id: String,
}

/// Issued short-lived channel access token.
/// <https://developers.line.biz/en/reference/messaging-api/#issue-shortlived-channel-access-token>
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IssueShortLivedChannelAccessTokenResponse {
    /// Short-lived channel access token, valid for 30 days.
    pub access_token: String,
    /// Seconds from issue to expiration.
    pub expires_in: u32,
    /// Always `Bearer`.
    pub token_type: String,
}

/// Verification result of a channel access token.
/// <https://developers.line.biz/en/reference/messaging-api/#verify-channel-access-token-v2-1>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifyChannelAccessTokenResponse {
    /// Channel ID the token was issued for.
    pub client_id: String,
    /// Seconds until the token expires.
    pub expires_in: u64,
    /// Permissions granted to the token.
    pub scope: Option<String>,
}

impl fmt::Debug for IssueStatelessChannelTokenRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::JwtAssertion { .. } => f
                .debug_struct("JwtAssertion")
                .field("client_assertion", &REDACTED)
                .finish(),
            Self::ClientSecret { client_id, .. } => f
                .debug_struct("ClientSecret")
                .field("client_id", client_id)
                .field("client_secret", &REDACTED)
                .finish(),
        }
    }
}

impl fmt::Debug for IssueStatelessChannelAccessTokenResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IssueStatelessChannelAccessTokenResponse")
            .field("access_token", &REDACTED)
            .field("expires_in", &self.expires_in)
            .field("token_type", &self.token_type)
            .finish()
    }
}

impl fmt::Debug for IssueChannelAccessTokenResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IssueChannelAccessTokenResponse")
            .field("access_token", &REDACTED)
            .field("expires_in", &self.expires_in)
            .field("token_type", &self.token_type)
            .field("key_id", &self.key_id)
            .finish()
    }
}

impl fmt::Debug for IssueShortLivedChannelAccessTokenResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IssueShortLivedChannelAccessTokenResponse")
            .field("access_token", &REDACTED)
            .field("expires_in", &self.expires_in)
            .field("token_type", &self.token_type)
            .finish()
    }
}
