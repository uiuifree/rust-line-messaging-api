use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineMessageActionPostback {
    #[serde(rename = "type")]
    message_type: String,
    label: String,
    data: String,
    #[serde(rename = "displayText")]
    display_text: String,
    #[serde(rename = "inputOption")]
    input_option: Option<LineMessageActionPostbackInputAction>,
    #[serde(rename = "fillInText")]
    fill_in_text: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum LineMessageActionPostbackInputAction {
    #[serde(rename = "closeRichMenu")]
    CloseRichMenu,
    #[serde(rename = "openRichMenu")]
    OpenRichMenu,
    #[serde(rename = "openKeyboard")]
    OpenKeyboard,
    #[serde(rename = "openVoice")]
    OpenVoice,
}
