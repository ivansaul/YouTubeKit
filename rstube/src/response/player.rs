use serde::Deserialize;
use serde_with::{serde_as, DefaultOnError, DisplayFromStr};
use time::OffsetDateTime;

use crate::{
    error::{ExtractionError, UnavailabilityReason},
    models::{channel::ChannelTag, VideoDetails},
    response::{Empty, Thumbnails},
    serializer::{text::Text, MapRespCtx, MapResponse, MapResult},
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
    pub channel_id: String,
    pub short_description: Option<String>,
    pub thumbnail: Thumbnails,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub view_count: Option<u64>,
    pub author: Option<String>,
    pub is_live_content: bool,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(tag = "status", rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum PlayabilityStatus {
    #[serde(rename_all = "camelCase")]
    Ok { live_streamability: Option<Empty> },

    /// Age limit / Private video
    #[serde(rename_all = "camelCase")]
    LoginRequired {
        #[serde(default)]
        reason: String,
        #[serde(default)]
        messages: Vec<String>,
    },

    /// Video can't be played because of DRM / Geoblock
    #[serde(rename_all = "camelCase")]
    Unplayable {
        #[serde(default)]
        reason: String,
        #[serde(default)]
        error_screen: ErrorScreen,
    },

    #[serde(rename_all = "camelCase")]
    LiveStreamOffline {
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

#[serde_as]
#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ErrorScreen {
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub player_error_message_renderer: Option<ErrorMessage>,
    pub player_captcha_view_model: Option<Empty>,
}

#[serde_as]
#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ErrorMessage {
    #[serde_as(as = "Text")]
    pub subreason: String,
}

impl MapResponse<VideoDetails> for PlayerResponse {
    fn map_response(
        self,
        ctx: &MapRespCtx<'_>,
    ) -> Result<MapResult<VideoDetails>, ExtractionError> {
        let mut warnings = Vec::new();

        match self.playability_status {
            PlayabilityStatus::Ok { .. } => {}
            PlayabilityStatus::LoginRequired { reason, messages } => {
                let mut msg = reason;
                for m in &messages {
                    if !msg.is_empty() {
                        msg.push(' ');
                    }
                    msg.push_str(m);
                }

                // reason (age restriction): "Sign in to confirm your age"
                // or: "This video may be inappropriate for some users."
                // reason (private): "This video is private"
                let reason = msg
                    .split_whitespace()
                    .find_map(|word| match word {
                        "age" | "inappropriate" => Some(UnavailabilityReason::AgeRestricted),
                        "private" => Some(UnavailabilityReason::Private),
                        "bot" => Some(UnavailabilityReason::IpBan),
                        _ => None,
                    })
                    .unwrap_or_default();

                match reason {
                    UnavailabilityReason::AgeRestricted => {}
                    _ => return Err(ExtractionError::Unavailable { reason, msg }),
                };
            }
            PlayabilityStatus::Unplayable {
                reason,
                error_screen,
            } => {
                let mut msg = reason;
                if let Some(error_screen) = error_screen.player_error_message_renderer {
                    msg.push_str(" - ");
                    msg.push_str(&error_screen.subreason);
                }

                warnings.push(msg);

                if error_screen.player_captcha_view_model.is_some() {
                    // UnavailabilityReason::Captcha
                    warnings.push("Captcha required".to_string());
                }
            }
            PlayabilityStatus::LiveStreamOffline { reason } => {
                return Err(ExtractionError::Unavailable {
                    reason: UnavailabilityReason::OfflineLivestream,
                    msg: reason,
                });
            }
            PlayabilityStatus::Error { reason } => {
                // reason (censored): "This video has been removed for violating YouTube's policy on hate speech. Learn more about combating hate speech in your country."
                // reason: "This video is unavailable"
                return Err(ExtractionError::Unavailable {
                    reason: UnavailabilityReason::Deleted,
                    msg: reason,
                });
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

        let title = details.title;
        let duration = details.length_seconds;
        let keywords = details.keywords;
        let description = details.short_description;
        let thumbnail = details.thumbnail.thumbnails;
        let view_count = details.view_count;
        let is_live = details.is_live_content;

        let channel_id = details.channel_id;
        let channel_name = details.author;

        let (like_count, is_short, category, publish_date) =
            match self.microformat.map(|p| p.player_microformat_renderer) {
                Some(mf) => (
                    mf.like_count,
                    mf.is_shorts_eligible,
                    mf.category,
                    mf.publish_date,
                ),
                None => (None, false, None, None),
            };

        let video = VideoDetails {
            id: video_id,
            title,
            duration,
            keywords,
            description,
            thumbnail,
            view_count,
            is_live,
            like_count,
            is_short,
            category,
            publish_date,
            publish_date_text: None,
            channel: ChannelTag {
                id: channel_id,
                name: channel_name.unwrap_or_default(),
                ..Default::default()
            },
        };

        Ok(MapResult {
            content: video,
            warnings,
        })
    }
}
