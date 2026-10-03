use std::fmt;
use std::time::Duration;

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap};
use reqwest::{Method, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use url::Url;

use crate::error::{ApiError, Error, ErrorResponse, Result};

const DEFAULT_API_BASE_URL: &str = "https://api.line.me";
const DEFAULT_DATA_BASE_URL: &str = "https://api-data.line.me";

/// Default time allowed to establish a connection.
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
/// Default time allowed between two reads of the response. There is no limit on the
/// total time, so that large content downloads are not cut off.
const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(10);

const REQUEST_ID_HEADER: &str = "x-line-request-id";
const ACCEPTED_REQUEST_ID_HEADER: &str = "x-line-accepted-request-id";
pub(crate) const RETRY_KEY_HEADER: &str = "X-Line-Retry-Key";

/// Async client for the LINE Messaging API.
///
/// Cloning is cheap: clones share the underlying connection pool.
///
/// # Errors
///
/// Every method that calls an endpoint returns:
///
/// - [`Error::Api`] when the LINE Platform answers with a non-2xx status. The
///   [`ApiError`] holds the status code, the `x-line-request-id` header and the
///   LINE error body.
/// - [`Error::Http`] when the request cannot be sent (connection or transport
///   failure) or the response body cannot be read.
/// - [`Error::Decode`] when a 2xx response body does not match the expected type.
///
/// ```no_run
/// use line_bot_messaging_api::LineClient;
/// use line_bot_messaging_api::api::PushMessageRequest;
/// use line_bot_messaging_api::message::TextMessage;
///
/// # async fn run() -> line_bot_messaging_api::Result<()> {
/// let client = LineClient::new("CHANNEL_ACCESS_TOKEN");
/// let request = PushMessageRequest::new("U1234...", [TextMessage::new("Hello")]);
/// client.push_message(&request).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct LineClient {
    http: reqwest::Client,
    channel_access_token: Option<String>,
    api_base_url: Url,
    data_base_url: Url,
}

impl LineClient {
    /// Creates a client that authenticates with the given channel access token.
    ///
    /// The client gives up when a connection is not established within 5 seconds or
    /// the server sends nothing for 10 seconds. Use [`LineClientBuilder::http_client`]
    /// to change this (e.g. for [`LineClient::test_webhook_endpoint`] against a slow
    /// webhook server).
    ///
    /// # Panics
    ///
    /// Panics if the TLS backend cannot be initialized, like `reqwest::Client::new`.
    pub fn new(channel_access_token: impl Into<String>) -> Self {
        Self {
            http: default_http_client().expect("failed to initialize the HTTP client"),
            channel_access_token: Some(channel_access_token.into()),
            api_base_url: Url::parse(DEFAULT_API_BASE_URL).expect("valid default URL"),
            data_base_url: Url::parse(DEFAULT_DATA_BASE_URL).expect("valid default URL"),
        }
    }

    /// Returns a builder for a client with custom settings.
    pub fn builder() -> LineClientBuilder {
        LineClientBuilder::default()
    }

    /// Builds a request authenticated with the channel access token.
    pub(crate) fn request(&self, method: Method, host: Host, path: &[&str]) -> RequestBuilder {
        let builder = self.request_without_auth(method, host, path);
        match &self.channel_access_token {
            Some(token) => builder.header(AUTHORIZATION, format!("Bearer {token}")),
            None => builder,
        }
    }

    /// Builds a request without the `Authorization` header (channel access token endpoints).
    pub(crate) fn request_without_auth(
        &self,
        method: Method,
        host: Host,
        path: &[&str],
    ) -> RequestBuilder {
        let mut url = match host {
            Host::Api => self.api_base_url.clone(),
            Host::Data => self.data_base_url.clone(),
        };
        url.path_segments_mut()
            .expect("base URL can be a base")
            .pop_if_empty()
            .extend(path);
        self.http.request(method, url)
    }

    /// Sends the request and decodes the JSON response body.
    pub(crate) async fn send_json<R: DeserializeOwned>(
        &self,
        request: RequestBuilder,
    ) -> Result<R> {
        let (body, _) = self.send_json_with_request_id(request).await?;
        Ok(body)
    }

    /// Sends the request and decodes the JSON response body, also returning
    /// the `x-line-request-id` response header.
    pub(crate) async fn send_json_with_request_id<R: DeserializeOwned>(
        &self,
        request: RequestBuilder,
    ) -> Result<(R, Option<String>)> {
        let response = checked(request.send().await?).await?;
        let request_id = header(response.headers(), REQUEST_ID_HEADER);
        let text = response.text().await?;
        // Some endpoints answer with an empty body; decode it as an empty object.
        let source = if text.trim().is_empty() { "{}" } else { &text };
        let body = serde_json::from_str(source).map_err(|source| Error::Decode {
            source,
            body: text.clone(),
            request_id: request_id.clone(),
        })?;
        Ok((body, request_id))
    }

    /// Sends the request and ignores the response body.
    pub(crate) async fn send_empty(&self, request: RequestBuilder) -> Result<()> {
        checked(request.send().await?).await?;
        Ok(())
    }

    /// Sends the request and returns the raw response body.
    pub(crate) async fn send_bytes(&self, request: RequestBuilder) -> Result<Content> {
        let response = checked(request.send().await?).await?;
        let content_type = header(response.headers(), CONTENT_TYPE.as_str());
        let data = response.bytes().await?.into();
        Ok(Content { data, content_type })
    }
}

impl fmt::Debug for LineClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LineClient")
            .field(
                "channel_access_token",
                &redacted(&self.channel_access_token),
            )
            .field("api_base_url", &self.api_base_url.as_str())
            .field("data_base_url", &self.data_base_url.as_str())
            .finish_non_exhaustive()
    }
}

fn redacted(token: &Option<String>) -> Option<&'static str> {
    token.as_ref().map(|_| "<redacted>")
}

/// Binary content downloaded from the LINE Platform.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Content {
    /// Response body.
    pub data: Vec<u8>,
    /// Value of the `Content-Type` response header.
    pub content_type: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Host {
    /// `https://api.line.me`
    Api,
    /// `https://api-data.line.me`
    Data,
}

async fn checked(response: Response) -> Result<Response> {
    let status = response.status();
    if status.is_success() {
        return Ok(response);
    }
    let headers = response.headers().clone();
    // Keep the status and request IDs even when the body cannot be read.
    let body = match response.text().await {
        Ok(text) => ErrorResponse::parse(text),
        Err(error) => ErrorResponse {
            message: format!(
                "failed to read the error response body: {}",
                error.without_url()
            ),
            ..ErrorResponse::default()
        },
    };
    Err(Error::Api(Box::new(ApiError {
        status: status.as_u16(),
        request_id: header(&headers, REQUEST_ID_HEADER),
        accepted_request_id: header(&headers, ACCEPTED_REQUEST_ID_HEADER),
        body,
    })))
}

fn header(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned)
}

/// Builder for [`LineClient`].
#[derive(Default)]
pub struct LineClientBuilder {
    http: Option<reqwest::Client>,
    channel_access_token: Option<String>,
    api_base_url: Option<String>,
    data_base_url: Option<String>,
}

impl LineClientBuilder {
    /// Channel access token sent as `Authorization: Bearer ...`.
    /// Not needed for the channel access token issuing endpoints.
    pub fn channel_access_token(mut self, token: impl Into<String>) -> Self {
        self.channel_access_token = Some(token.into());
        self
    }

    /// Uses the given `reqwest::Client`, e.g. to configure timeouts or proxies.
    /// It replaces the default client, including its timeouts.
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http = Some(client);
        self
    }

    /// Overrides `https://api.line.me`.
    pub fn api_base_url(mut self, url: impl Into<String>) -> Self {
        self.api_base_url = Some(url.into());
        self
    }

    /// Overrides `https://api-data.line.me`.
    pub fn data_base_url(mut self, url: impl Into<String>) -> Self {
        self.data_base_url = Some(url.into());
        self
    }

    /// Builds the client.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidBaseUrl`] when a base URL cannot be parsed or cannot be
    /// used as a base (e.g. `mailto:` URLs), and [`Error::Http`] when the default HTTP
    /// client cannot be initialized.
    pub fn build(self) -> Result<LineClient> {
        let api_base_url = self.api_base_url.as_deref().unwrap_or(DEFAULT_API_BASE_URL);
        let data_base_url = self
            .data_base_url
            .as_deref()
            .unwrap_or(DEFAULT_DATA_BASE_URL);
        Ok(LineClient {
            http: match self.http {
                Some(http) => http,
                None => default_http_client()?,
            },
            channel_access_token: self.channel_access_token,
            api_base_url: base_url(api_base_url)?,
            data_base_url: base_url(data_base_url)?,
        })
    }
}

impl fmt::Debug for LineClientBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LineClientBuilder")
            .field(
                "channel_access_token",
                &redacted(&self.channel_access_token),
            )
            .field("api_base_url", &self.api_base_url)
            .field("data_base_url", &self.data_base_url)
            .finish_non_exhaustive()
    }
}

/// HTTP client with the default timeouts described on [`LineClient::new`].
fn default_http_client() -> std::result::Result<reqwest::Client, reqwest::Error> {
    reqwest::Client::builder()
        .connect_timeout(DEFAULT_CONNECT_TIMEOUT)
        .read_timeout(DEFAULT_READ_TIMEOUT)
        .build()
}

fn base_url(url: &str) -> Result<Url> {
    let parsed = Url::parse(url)?;
    if parsed.cannot_be_a_base() {
        return Err(url::ParseError::RelativeUrlWithCannotBeABaseBase.into());
    }
    Ok(parsed)
}
