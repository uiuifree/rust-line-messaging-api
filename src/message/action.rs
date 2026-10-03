use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tagged_enum! {
    /// An action performed when a user taps a button, an image, a rich menu area, etc.
    ///
    /// <https://developers.line.biz/en/reference/messaging-api/#action-objects>
    pub enum Action {
        "camera" => Camera(CameraAction),
        "cameraRoll" => CameraRoll(CameraRollAction),
        "clipboard" => Clipboard(ClipboardAction),
        "datetimepicker" => DatetimePicker(DatetimePickerAction),
        "location" => Location(LocationAction),
        "message" => Message(MessageAction),
        "postback" => Postback(PostbackAction),
        "richmenuswitch" => RichMenuSwitch(RichMenuSwitchAction),
        "uri" => Uri(UriAction),
    }
}

/// Opens the camera screen. Usable only in quick reply buttons.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "camera", rename_all = "camelCase")]
pub struct CameraAction {
    /// Label for the action.
    pub label: Option<String>,
}

setters!(CameraAction { label: String });

/// Opens the camera roll screen. Usable only in quick reply buttons.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "cameraRoll", rename_all = "camelCase")]
pub struct CameraRollAction {
    /// Label for the action.
    pub label: Option<String>,
}

setters!(CameraRollAction { label: String });

/// Copies text to the clipboard.
///
/// <https://developers.line.biz/en/reference/messaging-api/#clipboard-action>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "clipboard", rename_all = "camelCase")]
pub struct ClipboardAction {
    /// Label for the action.
    pub label: Option<String>,
    /// Text that is copied to the clipboard (max 1000 characters).
    pub clipboard_text: String,
}

impl ClipboardAction {
    /// Creates a clipboard action with the required `clipboard_text` (1 to 1000 characters).
    pub fn new(clipboard_text: impl Into<String>) -> Self {
        Self {
            label: None,
            clipboard_text: clipboard_text.into(),
        }
    }
}

setters!(ClipboardAction { label: String });

/// Lets the user select a date and/or time; the selection is returned in a postback event.
///
/// <https://developers.line.biz/en/reference/messaging-api/#datetime-picker-action>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "datetimepicker", rename_all = "camelCase")]
pub struct DatetimePickerAction {
    /// Label for the action.
    pub label: Option<String>,
    /// String returned in the postback event (max 300 characters).
    pub data: Option<String>,
    /// Action mode: date, time, or date and time.
    pub mode: Option<DatetimePickerMode>,
    /// Initial value of the date or time.
    pub initial: Option<String>,
    /// Largest date or time that can be selected.
    pub max: Option<String>,
    /// Smallest date or time that can be selected.
    pub min: Option<String>,
}

impl DatetimePickerAction {
    /// Creates a datetime picker action from the required `data` and `mode`.
    pub fn new(data: impl Into<String>, mode: DatetimePickerMode) -> Self {
        Self {
            data: Some(data.into()),
            mode: Some(mode),
            ..Self::default()
        }
    }
}

setters!(DatetimePickerAction {
    label: String,
    data: String,
    mode: DatetimePickerMode,
    initial: String,
    max: String,
    min: String,
});

/// `mode` of [`DatetimePickerAction`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DatetimePickerMode {
    /// `date`: pick a date.
    Date,
    /// `time`: pick a time.
    Time,
    /// `datetime`: pick a date and time.
    Datetime,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}

/// Opens the location screen. Usable only in quick reply buttons.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "location", rename_all = "camelCase")]
pub struct LocationAction {
    /// Label for the action.
    pub label: Option<String>,
}

setters!(LocationAction { label: String });

/// Sends a text message from the user.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "message", rename_all = "camelCase")]
pub struct MessageAction {
    /// Label for the action.
    pub label: Option<String>,
    /// Text sent when the action is performed.
    pub text: Option<String>,
}

impl MessageAction {
    /// Creates a message action that sends `text`.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            label: None,
            text: Some(text.into()),
        }
    }
}

setters!(MessageAction {
    label: String,
    text: String,
});

/// Returns a postback event containing `data` to the webhook.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "postback", rename_all = "camelCase")]
pub struct PostbackAction {
    /// Label for the action.
    pub label: Option<String>,
    /// String returned in the postback event (max 300 characters).
    pub data: Option<String>,
    /// Text displayed in the chat as a message sent by the user.
    pub display_text: Option<String>,
    /// Text sent as a message from the user.
    pub text: Option<String>,
    /// How the rich menu or keyboard is shown after the action is performed.
    pub input_option: Option<PostbackInputOption>,
    /// Text pre-filled in the input field when the keyboard opens.
    pub fill_in_text: Option<String>,
}

impl PostbackAction {
    /// Creates a postback action with the required `data` (max 300 characters).
    pub fn new(data: impl Into<String>) -> Self {
        Self {
            data: Some(data.into()),
            ..Self::default()
        }
    }
}

setters!(PostbackAction {
    label: String,
    data: String,
    display_text: String,
    text: String,
    input_option: PostbackInputOption,
    fill_in_text: String,
});

/// `inputOption` of [`PostbackAction`]: how the rich menu or keyboard is shown after the tap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PostbackInputOption {
    /// `closeRichMenu`: closes the rich menu.
    CloseRichMenu,
    /// `openRichMenu`: opens the rich menu.
    OpenRichMenu,
    /// `openKeyboard`: opens the keyboard.
    OpenKeyboard,
    /// `openVoice`: opens the voice message input.
    OpenVoice,
    /// A value not known to this version of the crate.
    #[serde(other)]
    Unknown,
}

/// Switches the rich menu shown to the user.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "richmenuswitch", rename_all = "camelCase")]
pub struct RichMenuSwitchAction {
    /// Label for the action.
    pub label: Option<String>,
    /// String returned in the postback event (max 300 characters).
    pub data: Option<String>,
    /// Alias ID of the rich menu to switch to (max 32 characters).
    pub rich_menu_alias_id: Option<String>,
}

impl RichMenuSwitchAction {
    /// Creates a rich menu switch action from the required `rich_menu_alias_id` and `data`.
    pub fn new(rich_menu_alias_id: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            label: None,
            data: Some(data.into()),
            rich_menu_alias_id: Some(rich_menu_alias_id.into()),
        }
    }
}

setters!(RichMenuSwitchAction {
    label: String,
    data: String,
    rich_menu_alias_id: String,
});

/// Opens a URI.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename = "uri", rename_all = "camelCase")]
pub struct UriAction {
    /// Label for the action.
    pub label: Option<String>,
    /// URI opened when the action is performed.
    pub uri: Option<String>,
    /// URI opened on LINE for macOS and Windows instead of `uri`.
    pub alt_uri: Option<AltUri>,
}

impl UriAction {
    /// Creates a URI action that opens `uri`.
    pub fn new(uri: impl Into<String>) -> Self {
        Self {
            uri: Some(uri.into()),
            ..Self::default()
        }
    }
}

setters!(UriAction {
    label: String,
    uri: String,
    alt_uri: AltUri,
});

/// URI opened on LINE for macOS and Windows instead of [`UriAction::uri`].
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AltUri {
    /// URI opened on the desktop version of LINE (max 1000 characters).
    pub desktop: Option<String>,
}

impl AltUri {
    /// Creates an alternative URI with the `desktop` URI (max 1000 characters).
    pub fn new(desktop: impl Into<String>) -> Self {
        Self {
            desktop: Some(desktop.into()),
        }
    }
}
