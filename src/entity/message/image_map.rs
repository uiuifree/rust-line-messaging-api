use crate::message::LineMessageObject;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize, Serialize, Clone)]
pub struct LineMessageImageMap {
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "baseUrl")]
    base_url: String,
    #[serde(rename = "altText")]
    alt_text: String,
    #[serde(rename = "baseSize")]
    base_size: LineMessageImageMapBaseSize,

    #[serde(rename = "actions")]
    actions: Vec<Value>,
}

impl LineMessageImageMap {
    pub fn append_action(&mut self, action: impl LineMessageImageMapActionObject) {
        self.actions.push(action.build());
    }

    pub fn new<T: ToString>(
        base_url: T,
        alt_text: T,
        base_size: LineMessageImageMapBaseSize,
    ) -> Self {
        Self {
            message_type: "imagemap".to_string(),
            base_url: base_url.to_string(),
            alt_text: alt_text.to_string(),
            base_size,
            actions: vec![],
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LineMessageImageMapBaseSize {
    #[serde(rename = "height")]
    height: u32,
    #[serde(rename = "width")]
    width: u32,
}

impl LineMessageImageMapBaseSize {
    pub fn new(height: u32, width: u32) -> Self {
        Self { height, width }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LineMessageImageMapVideo {
    #[serde(rename = "originalContentUrl")]
    original_content_url: String,
    #[serde(rename = "previewImageUrl")]
    preview_image_url: String,
    area: LineMessageImageMapArea,
    #[serde(rename = "externalLink")]
    external_link: Option<LineMessageImageMapVideoExternalLink>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LineMessageImageMapVideoExternalLink {
    #[serde(rename = "linkUri")]
    link_uri: String,
    #[serde(rename = "label")]
    label: String,
}

/// https://developers.line.biz/ja/reference/messaging-api/#imagemap-action-objects

pub trait LineMessageImageMapActionObject {
    fn build(&self) -> Value;
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LineMessageImageMapActionUri {
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "label")]
    label: Option<String>,
    #[serde(rename = "linkUri")]
    link_uri: String,
    area: LineMessageImageMapArea,
}

impl LineMessageImageMapActionUri {
    pub fn new<T: ToString>(link_uri: T, area: LineMessageImageMapArea) -> Self {
        Self {
            message_type: "uri".to_string(),
            label: None,
            link_uri: link_uri.to_string(),
            area,
        }
    }
    pub fn set_label(&mut self, label: String) {
        self.label = Some(label);
    }
}

impl LineMessageImageMapActionObject for LineMessageImageMapActionUri {
    fn build(&self) -> Value {
        json!(self)
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LineMessageImageMapActionMessage {
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "label")]
    label: Option<String>,
    #[serde(rename = "linkUri")]
    text: String,
    area: LineMessageImageMapArea,
}

impl LineMessageImageMapActionObject for LineMessageImageMapActionMessage {
    fn build(&self) -> Value {
        json!(self)
    }
}
impl LineMessageImageMapActionMessage {
    pub fn new(text: String, area: LineMessageImageMapArea) -> Self {
        Self {
            message_type: "message".to_string(),
            label: None,
            text,
            area,
        }
    }
    pub fn set_label(&mut self, label: String) {
        self.label = Some(label);
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LineMessageImageMapArea {
    #[serde(rename = "x")]
    x: u32,
    #[serde(rename = "y")]
    y: u32,
    #[serde(rename = "width")]
    width: u32,
    #[serde(rename = "height")]
    height: u32,
}

impl LineMessageImageMapArea {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}
//

impl LineMessageObject for LineMessageImageMap {
    fn build(&self) -> Value {
        json!(self)
    }
}
