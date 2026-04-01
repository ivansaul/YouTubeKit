use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use time::OffsetDateTime;

use crate::{
    error::ExtractionError,
    models::VideoDetails,
    response::Thumbnails,
    serializer::{MapRespCtx, MapResponse, MapResult},
};

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

    /// Video can't be played because of DRM / Geoblock
    #[serde(rename_all = "camelCase")]
    Unplayable {
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

impl MapResponse<VideoDetails> for PlayerResponse {
    fn map_response(
        self,
        ctx: &MapRespCtx<'_>,
    ) -> Result<MapResult<VideoDetails>, ExtractionError> {
        let mut warnings = Vec::new();

        match self.playability_status {
            PlayabilityStatus::Ok => {}
            PlayabilityStatus::LoginRequired { reason }
            | PlayabilityStatus::Unplayable { reason } => warnings.push(reason),
            PlayabilityStatus::Error { reason } => {
                return Err(ExtractionError::Unavailable { reason });
            }
        }

        let details = self
            .video_details
            .ok_or_else(|| ExtractionError::InvalidData("Video details not found".into()))?;

        let video_id = details.video_id;

        if ctx.id != video_id {
            return Err(ExtractionError::WrongResult(format!(
                "got wrong video id {}, expected {}",
                video_id, ctx.id
            )));
        }

        let mut video = VideoDetails {
            id: video_id,
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

        Ok(MapResult {
            content: video,
            warnings,
        })
    }
}
