use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::api::SentMessage;

/// Result type used throughout this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors returned by this crate.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The LINE Platform returned a non-2xx response.
    #[error("LINE API returned {}: {}", .0.status, .0.body.summary())]
    Api(Box<ApiError>),
    /// The request could not be sent or the response could not be read.
    ///
    /// The request URL is removed from the error, because some endpoints carry
    /// credentials in the query string.
    #[error("HTTP request failed: {0}")]
    Http(#[source] reqwest::Error),
    /// The response body did not match the expected type.
    #[error("failed to decode response body: {source}")]
    Decode {
        /// The underlying JSON error.
        source: serde_json::Error,
        /// The body that failed to decode.
        body: String,
        /// Value of the `x-line-request-id` response header, when the body came from
        /// the LINE Platform. A request that reaches this error may still have been
        /// accepted, so keep this ID to investigate.
        request_id: Option<String>,
    },
    /// A base URL passed to [`crate::LineClientBuilder`] is invalid.
    #[error("invalid base URL: {0}")]
    InvalidBaseUrl(#[from] url::ParseError),
    /// The `x-line-signature` header of a webhook request did not match its body.
    #[error("webhook signature does not match the request body")]
    InvalidSignature,
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Http(error.without_url())
    }
}

impl Error {
    /// Returns the API error details when the LINE Platform rejected the request.
    pub fn api_error(&self) -> Option<&ApiError> {
        match self {
            Error::Api(e) => Some(e),
            _ => None,
        }
    }
}

/// A non-2xx response from the LINE Platform.
#[derive(Debug, Clone, PartialEq)]
pub struct ApiError {
    /// HTTP status code.
    pub status: u16,
    /// Value of the `x-line-request-id` response header.
    pub request_id: Option<String>,
    /// Value of the `x-line-accepted-request-id` response header.
    /// Set on `409 Conflict` when a request with the same retry key was already accepted.
    pub accepted_request_id: Option<String>,
    /// Parsed error body. When the body is not a LINE error object (or could not be
    /// read), `message` holds the raw body text (or the read error).
    pub body: ErrorResponse,
}

/// Error response body returned by the LINE Platform.
///
/// Messaging API endpoints fill `message` (and `details`); the channel access token
/// endpoints fill `error` and `error_description` instead.
///
/// <https://developers.line.biz/en/reference/messaging-api/#error-responses>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    /// Message containing information about the error. Empty for channel access
    /// token errors.
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub message: String,
    /// Details of the error. Empty when the response has no details.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    /// Messages that were already sent, returned on `409 Conflict` for retried requests.
    pub sent_messages: Option<Vec<SentMessage>>,
    /// Error summary of a channel access token endpoint (e.g. `invalid_request`).
    pub error: Option<String>,
    /// Details of a channel access token error. Not returned in certain situations.
    #[serde(rename = "error_description")]
    pub error_description: Option<String>,
}

impl ErrorResponse {
    /// Returns the most descriptive text available: `message`, then
    /// `error_description`, then `error`.
    pub fn summary(&self) -> &str {
        if !self.message.is_empty() {
            return &self.message;
        }
        self.error_description
            .as_deref()
            .or(self.error.as_deref())
            .unwrap_or_default()
    }

    /// Parses a LINE error body, or keeps the raw text in `message` when it is not one.
    pub(crate) fn parse(text: String) -> Self {
        match serde_json::from_str::<ErrorResponse>(&text) {
            Ok(body) if !body.message.is_empty() || body.error.is_some() => body,
            _ => ErrorResponse {
                message: text,
                ..ErrorResponse::default()
            },
        }
    }
}

/// Detail of an [`ErrorResponse`].
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDetail {
    /// Details of the error.
    pub message: Option<String>,
    /// Location of the error, e.g. the property name in the request.
    pub property: Option<String>,
}
