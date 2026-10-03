use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::LineClient;
use crate::client::Host;
use crate::error::Result;

impl LineClient {
    /// Issues a link token used for account linking.
    /// <https://developers.line.biz/en/reference/messaging-api/#issue-link-token>
    pub async fn issue_link_token(&self, user_id: &str) -> Result<IssueLinkTokenResponse> {
        let rb = self.request(
            Method::POST,
            Host::Api,
            &["v2", "bot", "user", user_id, "linkToken"],
        );
        self.send_json(rb).await
    }
}

/// Response of [`LineClient::issue_link_token`](crate::LineClient::issue_link_token).
///
/// <https://developers.line.biz/en/reference/messaging-api/#issue-link-token>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueLinkTokenResponse {
    /// Link token. Valid for 10 minutes and can only be used once.
    pub link_token: String,
}
