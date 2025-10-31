use crate::models::thumbnail::Thumbnail;
use crate::models::video_details::VideoDetails;
use serde::Deserialize;
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use time::OffsetDateTime;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse {
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

#[derive(Debug, Deserialize)]
pub struct Thumbnails {
    pub thumbnails: Vec<Thumbnail>,
}

impl PlayerResponse {
    pub fn map_video_details(self) -> Option<VideoDetails> {
        let mut video = VideoDetails {
            ..Default::default()
        };

        if let Some(vd) = self.video_details {
            video.id = vd.video_id;
            video.name = vd.title;
            video.duration = vd.length_seconds;
            video.keywords = vd.keywords;
            video.description = vd.short_description;
            video.thumbnail = vd.thumbnail.thumbnails;
            video.view_count = vd.view_count;
            video.is_live = vd.is_live_content;
        } else {
            return None;
        }

        if let Some(mf) = self.microformat {
            video.like_count = mf.player_microformat_renderer.like_count;
            video.is_short = mf.player_microformat_renderer.is_shorts_eligible;
            video.category = mf.player_microformat_renderer.category;
            video.publish_date = mf.player_microformat_renderer.publish_date;
        }

        Some(video)
    }
}
