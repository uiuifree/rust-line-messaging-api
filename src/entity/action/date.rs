use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LineMessageActionDatetimePicker {
    #[serde(rename = "type")]
    message_type: String,
    label: String,
    data: String,
    mode: LineMessageActionDatetimePickerMode,
    initial: String,
    max: String,
    min: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub enum LineMessageActionDatetimePickerMode {
    #[default]
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "time")]
    Time,
    #[serde(rename = "datetime")]
    Datetime,
}
