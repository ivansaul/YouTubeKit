use crate::models::thumbnail::Thumbnail;
use serde::Deserialize;
use serde_with::serde_as;
use serde_with::DisplayFromStr;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse {
    pub video_details: Option<VideoDetails>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDetails {
    pub video_id: String,
    pub title: String,
    pub length_seconds: String,
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

#[derive(Debug, Deserialize)]
pub struct Thumbnails {
    pub thumbnails: Vec<Thumbnail>,
}
