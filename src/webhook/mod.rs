//! Webhook signature verification and event types.
//!
//! ```no_run
//! use line_bot_messaging_api::webhook::{self, Event};
//!
//! fn handle(channel_secret: &str, body: &[u8], signature: &str) -> line_bot_messaging_api::Result<()> {
//!     let request = webhook::parse_request(channel_secret, body, signature)?;
//!     for event in &request.events {
//!         if let Event::Message(message) = event {
//!             let _ = (&message.message, event.reply_token());
//!         }
//!     }
//!     Ok(())
//! }
//! ```
//!
//! <https://developers.line.biz/en/reference/messaging-api/#webhooks>

mod event;
mod message;
mod source;

pub use event::*;
pub use message::*;
pub use source::*;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use hmac::{Hmac, KeyInit, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::{Error, Result};

/// Request body of a webhook request.
///
/// <https://developers.line.biz/en/reference/messaging-api/#request-body>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallbackRequest {
    /// User ID of the bot that should receive the webhook events (`U[0-9a-f]{32}`).
    pub destination: String,
    /// Webhook events. May be empty when the LINE Platform confirms communication.
    pub events: Vec<Event>,
}

/// Verifies the `x-line-signature` header of a webhook request.
///
/// `body` must be the raw request body exactly as received.
///
/// # Errors
///
/// Returns [`Error::InvalidSignature`] when the signature is not valid base64 or does not match.
///
/// <https://developers.line.biz/en/reference/messaging-api/#signature-validation>
pub fn verify_signature(channel_secret: &str, body: &[u8], signature: &str) -> Result<()> {
    let expected = STANDARD
        .decode(signature)
        .map_err(|_| Error::InvalidSignature)?;
    let mut mac = Hmac::<Sha256>::new_from_slice(channel_secret.as_bytes())
        .expect("HMAC accepts keys of any length");
    mac.update(body);
    mac.verify_slice(&expected)
        .map_err(|_| Error::InvalidSignature)
}

/// Verifies the signature of a webhook request and decodes its body.
///
/// # Errors
///
/// Returns [`Error::InvalidSignature`] when the signature does not match and
/// [`Error::Decode`] when the body is not a valid [`CallbackRequest`].
///
/// <https://developers.line.biz/en/reference/messaging-api/#request-body>
pub fn parse_request(
    channel_secret: &str,
    body: &[u8],
    signature: &str,
) -> Result<CallbackRequest> {
    verify_signature(channel_secret, body, signature)?;
    serde_json::from_slice(body).map_err(|source| Error::Decode {
        source,
        body: String::from_utf8_lossy(body).into_owned(),
        request_id: None,
    })
}
