use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize)]
pub struct ChannelPreview {
    /// Unique YouTube channel ID
    pub id: String,
    /// Channel name
    pub name: String,
    /// Channel label
    pub label: String,
    /// Channel avatar/profile picture
    pub avatar: Vec<Thumbnail>,
    /// Approximate number of subscribers
    ///
    /// [`None`] if hidden by the owner or not present.
    ///
    /// Info: This is only present in the `VideoDetails` response
    pub subscriber_count: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct VideoPreview {
    pub id: String,
    pub title: String,
    pub thumbnail: Vec<Thumbnail>,
    pub publish_date: Option<String>,
    pub length_text: Option<String>,
    pub view_count: Option<String>,
    pub channel: Option<ChannelPreview>,
    // pub is_short: bool,
    // pub is_live: bool,
    // pub description: Option<String>,
}

#[derive(Debug, Serialize, Default)]
pub struct VideoInfo {
    /// Unique YouTube video ID
    pub id: String,
    /// Video title
    pub title: String,
    /// Video duration in seconds.
    ///
    /// Is [`None`] for livestreams.
    pub duration: Option<u32>,
    /// List of words that describe the topic of the video
    pub keywords: Vec<String>,
    /// Video description
    pub description: Option<String>,
    /// Video thumbnails
    pub thumbnail: Vec<Thumbnail>,
    /// Number of views / current viewers in case of a livestream.
    pub view_count: Option<u64>,
    /// Is the video a livestream?
    pub is_live: bool,
    /// Number of likes
    ///
    /// [`None`] if the like count was hidden by the creator.
    pub like_count: Option<u32>,
    /// Category of the video.
    pub category: Option<String>,
    /// Is the video a YouTube Short video?
    pub is_short: bool,
    /// Video publishing date.
    ///
    /// [`None`] if the date could not be parsed.
    #[serde(with = "time::serde::rfc3339::option")]
    pub publish_date: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Thumbnail {
    pub url: String,
    pub width: i64,
    pub height: i64,
}
