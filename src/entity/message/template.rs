use crate::action::LineActionObject;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::message::LineMessageObject;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LineMessageTemplate {
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "altText")]
    alt_text: String,
    template: Value,
}

impl LineMessageTemplate {
    pub fn new<T: ToString>(alt_text: T, template: impl LineMessageTemplateObject) -> Self {
        Self {
            message_type: "template".to_string(),
            alt_text: alt_text.to_string(),
            template: template.build(),
        }
    }
}
impl LineMessageObject for LineMessageTemplate{
    fn build(&self) -> Value {
        json!(self)
    }
}

pub trait LineMessageTemplateObject {
    fn build(&self) -> Value;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LineMessageTemplateButton {
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "thumbnailImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_image_url: Option<String>,
    #[serde(rename = "imageAspectRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    image_aspect_ratio: Option<String>,
    #[serde(rename = "imageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    image_size: Option<String>,
    #[serde(rename = "imageBackgroundColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    image_background_color: Option<String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(rename = "text")]
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "defaultAction")]
    default_action: Option<Value>,
    #[serde(rename = "actions")]
    actions: Vec<Value>,
}

impl LineMessageTemplateButton {
    pub fn new<T: ToString>(text: T) -> Self {
        Self {
            message_type: "buttons".to_string(),
            thumbnail_image_url: None,
            image_aspect_ratio: None,
            image_size: None,
            image_background_color: None,
            title: None,
            text: text.to_string(),
            default_action: None,
            actions: vec![],
        }
    }
    pub fn set_default_action(&mut self,action: impl LineActionObject){
        self.default_action = Some(action.build());
    }
    pub fn append_action(&mut self,action: impl LineActionObject){
        self.actions.push(action.build());
    }
    pub fn set_thumbnail_image_url<T: ToString>(&mut self, thumbnail_image_url: T) {
        self.thumbnail_image_url = Some(thumbnail_image_url.to_string());
    }
    pub fn set_image_aspect_ratio<T: ToString>(&mut self, image_aspect_ratio: T) {
        self.image_aspect_ratio = Some(image_aspect_ratio.to_string());
    }
    pub fn set_image_size<T: ToString>(&mut self, image_size: T) {
        self.image_size = Some(image_size.to_string());
    }
    pub fn set_image_background_color<T: ToString>(&mut self, image_background_color: T) {
        self.image_background_color = Some(image_background_color.to_string());
    }
    pub fn set_title<T: ToString>(&mut self, title: T) {
        self.title = Some(title.to_string());
    }
}

impl LineMessageTemplateObject for LineMessageTemplateButton {
    fn build(&self) -> Value {
        json!(self)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LineMessageTemplateConfirm {
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "text")]
    text: String,
    #[serde(rename = "actions")]
    actions: Vec<Value>,
}

impl LineMessageTemplateConfirm {
    pub fn new<T: ToString>(text: T) -> Self {
        Self {
            message_type: "confirm".to_string(),
            text: text.to_string(),
            actions: vec![],
        }
    }
    pub fn append_action(&mut self,action: impl LineActionObject){
        self.actions.push(action.build());
    }

}
impl LineMessageTemplateObject for LineMessageTemplateConfirm {
    fn build(&self) -> Value {
        json!(self)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LineMessageTemplateCarousel {
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "columns")]
    columns: Vec<LineMessageTemplateCarouselColumn>,
    #[serde(rename = "imageAspectRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    image_aspect_ratio: Option<String>,
    #[serde(rename = "imageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    image_size: Option<String>,
}
impl LineMessageTemplateCarousel {
    pub fn new(columns: Vec<LineMessageTemplateCarouselColumn>) -> Self {
        Self {
            message_type: "carousel".to_string(),
            columns,
            image_aspect_ratio: None,
            image_size: None,
        }
    }
    pub fn append_column(&mut self,column: LineMessageTemplateCarouselColumn){
        self.columns.push(column);
    }
    pub fn set_image_aspect_ratio<T: ToString>(&mut self, image_aspect_ratio: T) {
        self.image_aspect_ratio = Some(image_aspect_ratio.to_string());
    }
    pub fn set_image_size<T: ToString>(&mut self, image_size: T) {
        self.image_size = Some(image_size.to_string());
    }
}
impl LineMessageTemplateObject for LineMessageTemplateCarousel {
    fn build(&self) -> Value {
        json!(self)
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LineMessageTemplateCarouselColumn {
    #[serde(rename = "thumbnailImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_image_url: Option<String>,
    #[serde(rename = "imageBackgroundColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    image_background_color: Option<String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(rename = "text")]
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "defaultAction")]
    default_action: Option<Value>,
    #[serde(rename = "actions")]
    actions: Vec<Value>,
}

impl LineMessageTemplateCarouselColumn{

    pub fn new<T: ToString>(text: T) -> Self {
        Self {
            thumbnail_image_url: None,
            image_background_color: None,
            title: None,
            text: text.to_string(),
            default_action: None,
            actions: vec![],
        }
    }
    pub fn set_default_action(&mut self,action: impl LineActionObject){
        self.default_action = Some(action.build());
    }
    pub fn append_action(&mut self,action: impl LineActionObject){
        self.actions.push(action.build());
    }
    pub fn set_thumbnail_image_url<T: ToString>(&mut self, thumbnail_image_url: T) {
        self.thumbnail_image_url = Some(thumbnail_image_url.to_string());
    }
    pub fn set_image_background_color<T: ToString>(&mut self, image_background_color: T) {
        self.image_background_color = Some(image_background_color.to_string());
    }
    pub fn set_title<T: ToString>(&mut self, title: T) {
        self.title = Some(title.to_string());
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LineMessageTemplateCarouselImage {
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "columns")]
    columns: Vec<LineMessageTemplateCarouselImageColumn>,
}

impl LineMessageTemplateCarouselImage{
    pub fn new(columns: Vec<LineMessageTemplateCarouselImageColumn>) -> Self {
        Self {
            message_type: "image_carousel".to_string(),
            columns,
        }
    }
    pub fn append_column(&mut self,column: LineMessageTemplateCarouselImageColumn){
        self.columns.push(column);
    }
}
impl LineMessageTemplateObject for LineMessageTemplateCarouselImage {
    fn build(&self) -> Value {
        json!(self)
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LineMessageTemplateCarouselImageColumn {
    #[serde(rename = "imageUrl")]
    image_url: String,
    #[serde(rename = "action")]
    action: Value,
}
impl LineMessageTemplateCarouselImageColumn{
    pub fn new<T: ToString>(image_url: T,action: impl LineActionObject) -> Self {
        Self {
            image_url: image_url.to_string(),
            action: action.build(),
        }
    }
}
