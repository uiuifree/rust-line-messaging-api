//! LINE echo bot: replies to every text message with the same text.
//!
//! ```sh
//! LINE_CHANNEL_SECRET=... LINE_CHANNEL_ACCESS_TOKEN=... cargo run --example echo_bot
//! ```
//!
//! Then set `https://<your-host>/callback` as the webhook URL in the LINE Developers Console
//! (for local testing, expose port 3000 with a tunnel such as ngrok).

use std::sync::Arc;

use axum::Router;
use axum::body::Bytes;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::routing::post;
use line_bot_messaging_api::LineClient;
use line_bot_messaging_api::api::ReplyMessageRequest;
use line_bot_messaging_api::message::TextMessage;
use line_bot_messaging_api::webhook::{self, Event, MessageContent};

struct AppState {
    client: LineClient,
    channel_secret: String,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        client: LineClient::new(env("LINE_CHANNEL_ACCESS_TOKEN")),
        channel_secret: env("LINE_CHANNEL_SECRET"),
    });
    let app = Router::new()
        .route("/callback", post(callback))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn callback(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    let Some(signature) = headers
        .get("x-line-signature")
        .and_then(|v| v.to_str().ok())
    else {
        return StatusCode::BAD_REQUEST;
    };
    // Verify the signature against the raw body, then parse the events.
    let Ok(request) = webhook::parse_request(&state.channel_secret, &body, signature) else {
        return StatusCode::BAD_REQUEST;
    };
    for event in request.events {
        let Event::Message(event) = event else {
            continue;
        };
        let (Some(reply_token), MessageContent::Text(text)) = (&event.reply_token, &event.message)
        else {
            continue;
        };
        let reply = ReplyMessageRequest::new(reply_token, [TextMessage::new(&text.text)]);
        if let Err(err) = state.client.reply_message(&reply).await {
            eprintln!("reply failed: {err}");
        }
    }
    StatusCode::OK
}

fn env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{name} is not set"))
}
