use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{MessageContent, Source, UserSource};

tagged_enum! {
    /// Webhook event object.
    ///
    /// <https://developers.line.biz/en/reference/messaging-api/#webhook-event-objects>
    pub enum Event {
        "message" => Message(MessageEvent),
        "unsend" => Unsend(UnsendEvent),
        "follow" => Follow(FollowEvent),
        "unfollow" => Unfollow(UnfollowEvent),
        "join" => Join(JoinEvent),
        "leave" => Leave(LeaveEvent),
        "memberJoined" => MemberJoined(MemberJoinedEvent),
        "memberLeft" => MemberLeft(MemberLeftEvent),
        "postback" => Postback(PostbackEvent),
        "videoPlayComplete" => VideoPlayComplete(VideoPlayCompleteEvent),
        "beacon" => Beacon(BeaconEvent),
        "accountLink" => AccountLink(AccountLinkEvent),
        "membership" => Membership(MembershipEvent),
        "module" => Module(ModuleEvent),
        "activated" => Activated(ActivatedEvent),
        "deactivated" => Deactivated(DeactivatedEvent),
        "botSuspended" => BotSuspended(BotSuspendedEvent),
        "botResumed" => BotResumed(BotResumedEvent),
        "delivery" => Delivery(PnpDeliveryCompletionEvent),
        "messageEdited" => MessageEdited(MessageEditedEvent),
    }
}

impl Event {
    /// Returns the properties shared by every event, or `None` for an unknown event type.
    pub fn common(&self) -> Option<&EventCommon> {
        match self {
            Event::Message(MessageEvent { common, .. })
            | Event::Unsend(UnsendEvent { common, .. })
            | Event::Follow(FollowEvent { common, .. })
            | Event::Unfollow(UnfollowEvent { common })
            | Event::Join(JoinEvent { common, .. })
            | Event::Leave(LeaveEvent { common })
            | Event::MemberJoined(MemberJoinedEvent { common, .. })
            | Event::MemberLeft(MemberLeftEvent { common, .. })
            | Event::Postback(PostbackEvent { common, .. })
            | Event::VideoPlayComplete(VideoPlayCompleteEvent { common, .. })
            | Event::Beacon(BeaconEvent { common, .. })
            | Event::AccountLink(AccountLinkEvent { common, .. })
            | Event::Membership(MembershipEvent { common, .. })
            | Event::Module(ModuleEvent { common, .. })
            | Event::Activated(ActivatedEvent { common, .. })
            | Event::Deactivated(DeactivatedEvent { common })
            | Event::BotSuspended(BotSuspendedEvent { common })
            | Event::BotResumed(BotResumedEvent { common })
            | Event::Delivery(PnpDeliveryCompletionEvent { common, .. })
            | Event::MessageEdited(MessageEditedEvent { common, .. }) => Some(common),
            Event::Unknown(_) => None,
        }
    }

    /// Returns the source of the event, when included.
    pub fn source(&self) -> Option<&Source> {
        self.common()?.source.as_ref()
    }

    /// Returns the reply token, when the event can be replied to.
    pub fn reply_token(&self) -> Option<&str> {
        match self {
            Event::Follow(FollowEvent { reply_token, .. })
            | Event::Join(JoinEvent { reply_token, .. })
            | Event::MemberJoined(MemberJoinedEvent { reply_token, .. })
            | Event::VideoPlayComplete(VideoPlayCompleteEvent { reply_token, .. })
            | Event::Beacon(BeaconEvent { reply_token, .. })
            | Event::Membership(MembershipEvent { reply_token, .. })
            | Event::Message(MessageEvent { reply_token, .. })
            | Event::Postback(PostbackEvent { reply_token, .. })
            | Event::AccountLink(AccountLinkEvent { reply_token, .. })
            | Event::MessageEdited(MessageEditedEvent { reply_token, .. }) => {
                reply_token.as_deref()
            }
            _ => None,
        }
    }
}

/// Properties shared by every webhook event.
///
/// <https://developers.line.biz/en/reference/messaging-api/#common-properties>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventCommon {
    /// Source of the event.
    pub source: Option<Source>,
    /// Time of the event in milliseconds.
    pub timestamp: i64,
    /// Channel state.
    pub mode: EventMode,
    /// ID that uniquely identifies the webhook event (ULID).
    pub webhook_event_id: String,
    /// Delivery context of the webhook event.
    pub delivery_context: DeliveryContext,
}

/// Channel state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EventMode {
    /// The channel is active; you can reply.
    Active,
    /// The channel is waiting for another module channel to release control.
    Standby,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}

/// Delivery context of a webhook event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryContext {
    /// Whether the webhook event is a redelivered one.
    pub is_redelivery: bool,
}

/// Event for a message sent to the bot.
///
/// <https://developers.line.biz/en/reference/messaging-api/#message-event>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "message", rename_all = "camelCase")]
pub struct MessageEvent {
    /// Properties shared by every webhook event.
    #[serde(flatten)]
    pub common: EventCommon,
    /// Reply token used to reply to this event.
    pub reply_token: Option<String>,
    /// The sent message.
    pub message: MessageContent,
}

/// Event for when the user unsends a message.
///
/// <https://developers.line.biz/en/reference/messaging-api/#unsend-event>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "unsend")]
pub struct UnsendEvent {
    /// Properties shared by every webhook event.
    #[serde(flatten)]
    pub common: EventCommon,
    /// The unsent message.
    pub unsend: UnsendDetail,
}

/// <https://developers.line.biz/en/reference/messaging-api/#unsend-event>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnsendDetail {
    /// Message ID of the unsent message.
    pub message_id: String,
}

/// Event for when the LINE Official Account is added as a friend (or unblocked).
///
/// <https://developers.line.biz/en/reference/messaging-api/#follow-event>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "follow", rename_all = "camelCase")]
pub struct FollowEvent {
    /// Properties shared by every webhook event.
    #[serde(flatten)]
    pub common: EventCommon,
    /// Reply token used to reply to this event.
    ///
    /// The specification marks it as required; it is decoded as optional so that an
    /// event without it does not make the whole webhook request fail to parse.
    pub reply_token: Option<String>,
    /// Follow details.
    pub follow: FollowDetail,
}

/// <https://developers.line.biz/en/reference/messaging-api/#follow-event>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowDetail {
    /// `true` when the user unblocked the account, `false` when they added it as a friend.
    pub is_unblocked: bool,
}

/// Event for when the LINE Official Account is blocked.
///
/// <https://developers.line.biz/en/reference/messaging-api/#unfollow-event>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "unfollow")]
pub struct UnfollowEvent {
    /// Properties shared by every webhook event.
    #[serde(flatten)]
    pub common: EventCommon,
}

/// Event for when the LINE Official Account joins a group chat or multi-person chat.
///
/// <https://developers.line.biz/en/reference/messaging-api/#join-event>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "join", rename_all = "camelCase")]
pub struct JoinEvent {
    /// Properties shared by every webhook event.
    #[serde(flatten)]
    pub common: EventCommon,
    /// Reply token used to reply to this event.
    ///
    /// The specification marks it as required; it is decoded as optional so that an
    /// event without it does not make the whole webhook request fail to parse.
    pub reply_token: Option<String>,
}

/// Event for when the LINE Official Account leaves or is removed from a chat.
///
/// <https://developers.line.biz/en/reference/messaging-api/#leave-event>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "leave")]
pub struct LeaveEvent {
    /// Properties shared by every webhook event.
    #[serde(flatten)]
    pub common: EventCommon,
}

/// Event for when users join a chat the LINE Official Account is in.
///
/// <https://developers.line.biz/en/reference/messaging-api/#member-joined-event>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "memberJoined", rename_all = "camelCase")]
pub struct MemberJoinedEvent {
    /// Properties shared by every webhook event.
    #[serde(flatten)]
    pub common: EventCommon,
    /// Reply token used to reply to this event.
    ///
    /// The specification marks it as required; it is decoded as optional so that an
    /// event without it does not make the whole webhook request fail to parse.
    pub reply_token: Option<String>,
    /// Users who joined.
    pub joined: JoinedMembers,
}

/// <https://developers.line.biz/en/reference/messaging-api/#member-joined-event>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JoinedMembers {
    /// Users who joined.
    pub members: Vec<UserSource>,
}

/// Event for when users leave a chat the LINE Official Account is in.
///
/// <https://developers.line.biz/en/reference/messaging-api/#member-left-event>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "memberLeft")]
pub struct MemberLeftEvent {
    /// Properties shared by every webhook event.
    #[serde(flatten)]
    pub common: EventCommon,
    /// Users who left.
    pub left: LeftMembers,
}

/// <https://developers.line.biz/en/reference/messaging-api/#member-left-event>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeftMembers {
    /// Users who left.
    pub members: Vec<UserSource>,
}

/// Event for when a user performs a postback action.
///
/// <https://developers.line.biz/en/reference/messaging-api/#postback-event>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "postback", rename_all = "camelCase")]
pub struct PostbackEvent {
    /// Properties shared by every webhook event.
    #[serde(flatten)]
    pub common: EventCommon,
    /// Reply token used to reply to this event.
    pub reply_token: Option<String>,
    /// Postback content.
    pub postback: PostbackContent,
}

/// <https://developers.line.biz/en/reference/messaging-api/#postback-event>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostbackContent {
    /// Postback data.
    pub data: String,
    /// Values selected by the user, e.g. `date`, `time`, `datetime` (datetime picker)
    /// or `newRichMenuAliasId` and `status` (rich menu switch action).
    pub params: Option<BTreeMap<String, String>>,
}

/// Event for when a user finishes viewing a video with a `trackingId`.
///
/// <https://developers.line.biz/en/reference/messaging-api/#video-viewing-complete>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "videoPlayComplete", rename_all = "camelCase")]
pub struct VideoPlayCompleteEvent {
    /// Properties shared by every webhook event.
    #[serde(flatten)]
    pub common: EventCommon,
    /// Reply token used to reply to this event.
    ///
    /// The specification marks it as required; it is decoded as optional so that an
    /// event without it does not make the whole webhook request fail to parse.
    pub reply_token: Option<String>,
    /// The viewed video.
    pub video_play_complete: VideoPlayComplete,
}

/// <https://developers.line.biz/en/reference/messaging-api/#video-viewing-complete>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoPlayComplete {
    /// `trackingId` assigned to the video message.
    pub tracking_id: String,
}

/// Event for when a user enters the range of a LINE Beacon.
///
/// <https://developers.line.biz/en/reference/messaging-api/#beacon-event>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "beacon", rename_all = "camelCase")]
pub struct BeaconEvent {
    /// Properties shared by every webhook event.
    #[serde(flatten)]
    pub common: EventCommon,
    /// Reply token used to reply to this event.
    ///
    /// The specification marks it as required; it is decoded as optional so that an
    /// event without it does not make the whole webhook request fail to parse.
    pub reply_token: Option<String>,
    /// Detected beacon.
    pub beacon: BeaconContent,
}

/// <https://developers.line.biz/en/reference/messaging-api/#beacon-event>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeaconContent {
    /// Hardware ID of the detected beacon.
    pub hwid: String,
    /// Type of beacon event.
    pub r#type: BeaconEventType,
    /// Device message of the detected beacon.
    pub dm: Option<String>,
}

/// Type of beacon event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BeaconEventType {
    /// `enter`
    Enter,
    /// `banner`
    Banner,
    /// `stay`
    Stay,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}

/// Event for when a user links their LINE account with a provider's service account.
///
/// <https://developers.line.biz/en/reference/messaging-api/#account-link-event>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "accountLink", rename_all = "camelCase")]
pub struct AccountLinkEvent {
    /// Properties shared by every webhook event.
    #[serde(flatten)]
    pub common: EventCommon,
    /// Reply token used to reply to this event. Not included when linking failed.
    pub reply_token: Option<String>,
    /// Result of the account link.
    pub link: LinkContent,
}

/// <https://developers.line.biz/en/reference/messaging-api/#account-link-event>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkContent {
    /// Whether linking the account succeeded.
    pub result: LinkResult,
    /// Nonce specified when verifying the user ID.
    pub nonce: String,
}

/// Result of an account link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LinkResult {
    /// Linking the account succeeded.
    Ok,
    /// Linking the account failed.
    Failed,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}

/// Event for when a user joins, leaves or renews a membership.
///
/// <https://developers.line.biz/en/reference/messaging-api/#membership-event>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "membership", rename_all = "camelCase")]
pub struct MembershipEvent {
    /// Properties shared by every webhook event.
    #[serde(flatten)]
    pub common: EventCommon,
    /// Reply token used to reply to this event.
    ///
    /// The specification marks it as required; it is decoded as optional so that an
    /// event without it does not make the whole webhook request fail to parse.
    pub reply_token: Option<String>,
    /// Membership change.
    pub membership: MembershipContent,
}

tagged_enum! {
    /// Content of a membership event.
    ///
    /// <https://developers.line.biz/en/reference/messaging-api/#membership-event>
    pub enum MembershipContent {
        /// The user joined a membership.
        "joined" => Joined(JoinedMembershipContent),
        /// The user left a membership.
        "left" => Left(LeftMembershipContent),
        /// The user renewed a membership.
        "renewed" => Renewed(RenewedMembershipContent),
    }
}

/// The user joined a membership.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "joined", rename_all = "camelCase")]
pub struct JoinedMembershipContent {
    /// ID of the membership the user joined.
    pub membership_id: i64,
}

/// The user left a membership.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "left", rename_all = "camelCase")]
pub struct LeftMembershipContent {
    /// ID of the membership the user left.
    pub membership_id: i64,
}

/// The user renewed a membership.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "renewed", rename_all = "camelCase")]
pub struct RenewedMembershipContent {
    /// ID of the membership the user renewed.
    pub membership_id: i64,
}

/// Event for when a module channel is attached to or detached from a LINE Official Account.
///
/// <https://developers.line.biz/en/reference/partner-docs/#module-channel-specific-webhook-events>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "module")]
pub struct ModuleEvent {
    /// Properties shared by every webhook event.
    #[serde(flatten)]
    pub common: EventCommon,
    /// Attach or detach details.
    pub module: ModuleContent,
}

tagged_enum! {
    /// Content of a module event.
    ///
    /// <https://developers.line.biz/en/reference/partner-docs/#module-channel-specific-webhook-events>
    pub enum ModuleContent {
        /// The module channel was attached.
        "attached" => Attached(AttachedModuleContent),
        /// The module channel was detached.
        "detached" => Detached(DetachedModuleContent),
    }
}

/// <https://developers.line.biz/en/reference/partner-docs/#attached-event>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "attached", rename_all = "camelCase")]
pub struct AttachedModuleContent {
    /// User ID of the bot on the attached LINE Official Account.
    pub bot_id: String,
    /// Scopes permitted by the admin of the LINE Official Account.
    pub scopes: Vec<String>,
}

/// <https://developers.line.biz/en/reference/partner-docs/#detached-event>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "detached", rename_all = "camelCase")]
pub struct DetachedModuleContent {
    /// User ID of the bot on the detached LINE Official Account.
    pub bot_id: String,
    /// Reason for detaching.
    pub reason: DetachedReason,
}

/// Reason a module channel was detached.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetachedReason {
    /// `bot_deleted`: the LINE Official Account was deleted.
    BotDeleted,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}

/// Event for when the module channel becomes the Active Channel.
///
/// <https://developers.line.biz/en/reference/partner-docs/#activated-event>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "activated", rename_all = "camelCase")]
pub struct ActivatedEvent {
    /// Properties shared by every webhook event.
    #[serde(flatten)]
    pub common: EventCommon,
    /// Chat control details.
    pub chat_control: ChatControl,
}

/// <https://developers.line.biz/en/reference/partner-docs/#activated-event>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatControl {
    /// When the chat control expires, in milliseconds since the epoch.
    pub expire_at: i64,
}

/// Event for when the module channel becomes the Standby Channel.
///
/// <https://developers.line.biz/en/reference/partner-docs/#deactivated-event>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "deactivated")]
pub struct DeactivatedEvent {
    /// Properties shared by every webhook event.
    #[serde(flatten)]
    pub common: EventCommon,
}

/// Event for when the LINE Official Account is suspended.
///
/// <https://developers.line.biz/en/reference/partner-docs/#botsuspend-event>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "botSuspended")]
pub struct BotSuspendedEvent {
    /// Properties shared by every webhook event.
    #[serde(flatten)]
    pub common: EventCommon,
}

/// Event for when the LINE Official Account returns from the suspended state.
///
/// <https://developers.line.biz/en/reference/partner-docs/#botresumed-event>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "botResumed")]
pub struct BotResumedEvent {
    /// Properties shared by every webhook event.
    #[serde(flatten)]
    pub common: EventCommon,
}

/// Event for when a LINE notification message has been delivered.
///
/// <https://developers.line.biz/en/docs/partner-docs/line-notification-messages/message-sending-complete-webhook-event/#overview-delivery-webhook-event>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "delivery")]
pub struct PnpDeliveryCompletionEvent {
    /// Properties shared by every webhook event.
    #[serde(flatten)]
    pub common: EventCommon,
    /// Delivery details.
    pub delivery: PnpDelivery,
}

/// <https://developers.line.biz/en/docs/partner-docs/line-notification-messages/message-sending-complete-webhook-event/#overview-delivery-webhook-event>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PnpDelivery {
    /// Hashed phone number or the value of the `X-Line-Delivery-Tag` header.
    pub data: String,
}

/// Event for when a user edits a text message (group chats only).
///
/// <https://developers.line.biz/en/reference/messaging-api/#edit-event>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "messageEdited", rename_all = "camelCase")]
pub struct MessageEditedEvent {
    /// Properties shared by every webhook event.
    #[serde(flatten)]
    pub common: EventCommon,
    /// Reply token used to reply to this event.
    pub reply_token: Option<String>,
    /// The edited message.
    pub message: MessageContent,
}
