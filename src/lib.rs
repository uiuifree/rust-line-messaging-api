#![doc = include_str!("../README.en.md")]
#![warn(missing_docs)]

/// Compiles the examples in the Japanese README as doctests.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeJaDoctests;

#[macro_use]
mod macros;

pub mod api;
mod client;
mod error;
pub mod message;
pub mod webhook;

pub use client::{Content, LineClient, LineClientBuilder};
pub use error::{ApiError, Error, ErrorDetail, ErrorResponse, Result};
/// Re-exported so that [`LineClientBuilder::http_client`] can be configured without
/// depending on a matching `reqwest` version.
pub use reqwest;
/// Re-exported for `Unknown(serde_json::Value)` variants and Flex JSON.
pub use serde_json;
