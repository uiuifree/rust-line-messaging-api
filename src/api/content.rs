use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::client::Host;
use crate::error::Result;
use crate::{Content, LineClient};

impl LineClient {
    /// Downloads the image, video, audio or file sent by a user.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-content>
    pub async fn get_message_content(&self, message_id: &str) -> Result<Content> {
        let rb = self.request(
            Method::GET,
            Host::Data,
            &["v2", "bot", "message", message_id, "content"],
        );
        self.send_bytes(rb).await
    }

    /// Downloads the preview image of an image or video sent by a user.
    /// <https://developers.line.biz/en/reference/messaging-api/#get-image-or-video-preview>
    pub async fn get_message_content_preview(&self, message_id: &str) -> Result<Content> {
        let rb = self.request(
            Method::GET,
            Host::Data,
            &["v2", "bot", "message", message_id, "content", "preview"],
        );
        self.send_bytes(rb).await
    }

    /// Checks whether a video or audio sent by a user is ready to download.
    /// <https://developers.line.biz/en/reference/messaging-api/#verify-video-or-audio-preparation-status>
    pub async fn get_message_content_transcoding_by_message_id(
        &self,
        message_id: &str,
    ) -> Result<GetMessageContentTranscodingResponse> {
        let rb = self.request(
            Method::GET,
            Host::Data,
            &["v2", "bot", "message", message_id, "content", "transcoding"],
        );
        self.send_json(rb).await
    }
}

/// Preparation status of a video or audio sent by a user.
///
/// <https://developers.line.biz/en/reference/messaging-api/#verify-video-or-audio-preparation-status>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMessageContentTranscodingResponse {
    /// The preparation status.
    pub status: TranscodingStatus,
}

/// Preparation status of content sent by a user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TranscodingStatus {
    /// Preparing to get content.
    Processing,
    /// Ready to get the content. You can get the content sent by users.
    Succeeded,
    /// Failed to prepare to get the content.
    Failed,
    /// A status this crate does not know about.
    #[serde(other)]
    Unknown,
}
