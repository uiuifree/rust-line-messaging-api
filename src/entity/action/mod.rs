mod camera;
mod camera_roll;
mod date;
mod location;
mod message;
mod post_back;
mod rich_menu_switch;
mod uri;

use serde_json::Value;

/// https://developers.line.biz/ja/reference/messaging-api/#action-objects

pub trait LineActionObject {
    fn build(&self) -> Value;
}

pub use camera::*;
pub use camera_roll::*;
pub use date::*;
pub use location::*;
pub use message::*;
pub use post_back::*;
pub use rich_menu_switch::*;
pub use uri::*;
