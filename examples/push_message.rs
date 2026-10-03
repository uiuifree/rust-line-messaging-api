//! Sends a text message with quick reply buttons to one user.
//!
//! ```sh
//! LINE_CHANNEL_ACCESS_TOKEN=... cargo run --example push_message -- <USER_ID>
//! ```

use line_bot_messaging_api::LineClient;
use line_bot_messaging_api::api::PushMessageRequest;
use line_bot_messaging_api::message::{MessageAction, QuickReply, QuickReplyItem, TextMessage};

#[tokio::main]
async fn main() -> line_bot_messaging_api::Result<()> {
    let token =
        std::env::var("LINE_CHANNEL_ACCESS_TOKEN").expect("LINE_CHANNEL_ACCESS_TOKEN is not set");
    let user_id = std::env::args()
        .nth(1)
        .expect("usage: push_message <USER_ID>");

    let client = LineClient::new(token);
    let message = TextMessage::new("Hello from Rust!").quick_reply(QuickReply::new([
        QuickReplyItem::new(MessageAction::new("Yes").label("Yes")),
        QuickReplyItem::new(MessageAction::new("No").label("No")),
    ]));
    let response = client
        .push_message(&PushMessageRequest::new(user_id, [message]))
        .await?;
    println!("sent: {:?}", response.sent_messages);
    Ok(())
}
