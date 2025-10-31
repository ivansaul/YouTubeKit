use crate::models::thumbnail::Thumbnail;
use crate::models::video_details::VideoDetails;

use common::error::ExtractionError;
use serde::Deserialize;
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use time::OffsetDateTime;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse {
    pub playability_status: PlayabilityStatus,
    pub video_details: Option<VideoDetailsResponse>,
    pub microformat: Option<Microformat>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Microformat {
    pub player_microformat_renderer: PlayerMicroformatRenderer,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMicroformatRenderer {
    pub category: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub publish_date: Option<OffsetDateTime>,
    pub is_shorts_eligible: bool,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub like_count: Option<u32>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDetailsResponse {
    pub video_id: String,
    pub title: String,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub length_seconds: Option<u32>,
    #[serde(default)]
    pub keywords: Vec<String>,
    pub short_description: Option<String>,
    pub thumbnail: Thumbnails,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub view_count: Option<u64>,
    pub is_live_content: bool,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(tag = "status", rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum PlayabilityStatus {
    #[serde(rename_all = "camelCase")]
    Ok,

    /// Age limit / Private video
    #[serde(rename_all = "camelCase")]
    LoginRequired {
        #[serde(default)]
        reason: String,
    },

    /// Video was censored / deleted / unavailable
    #[serde(rename_all = "camelCase")]
    Error {
        #[serde(default)]
        reason: String,
    },
}

#[derive(Debug, Deserialize)]
pub struct Thumbnails {
    pub thumbnails: Vec<Thumbnail>,
}

impl PlayerResponse {
    pub fn map_video_details(self) -> Result<VideoDetails, ExtractionError> {
        if let PlayabilityStatus::Error { reason } = self.playability_status {
            return Err(ExtractionError::Unavailable { reason });
        }

        let details = self.video_details.ok_or(ExtractionError::InvalidData(
            "[PlayerResponse] Video details not found",
        ))?;

        let mut video = VideoDetails {
            id: details.video_id,
            title: details.title,
            duration: details.length_seconds,
            keywords: details.keywords,
            description: details.short_description,
            thumbnail: details.thumbnail.thumbnails,
            view_count: details.view_count,
            is_live: details.is_live_content,
            ..Default::default()
        };

        if let Some(mf) = self.microformat.map(|p| p.player_microformat_renderer) {
            video.like_count = mf.like_count;
            video.is_short = mf.is_shorts_eligible;
            video.category = mf.category;
            video.publish_date = mf.publish_date;
        }

        Ok(video)
    }
}
