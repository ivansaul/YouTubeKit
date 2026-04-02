use serde::Deserialize;
use time::OffsetDateTime;

use crate::models::{
    channel::{ChannelPreview, ChannelTag},
    thumbnail::Thumbnail,
};

/// YouTube video item from search results, recommendations or playlists
#[derive(Debug)]
#[non_exhaustive]
pub struct VideoItem {
    /// Unique YouTube video ID
    pub id: String,
    /// Video title
    pub name: String,
    /// Video thumbnails
    pub thumbnails: Vec<Thumbnail>,
}

/// Lightweight video preview (for listings)
#[derive(Debug)]
pub struct VideoPreview {
    pub id: String,
    pub title: String,
    pub thumbnail: Vec<Thumbnail>,
    pub publish_date: Option<String>,
    pub length_text: Option<String>,
    pub view_count: Option<String>,
    pub channel: Option<ChannelPreview>,
}

/// Full video details fetched from the player endpoint
#[derive(Debug, Default, Deserialize)]
pub struct VideoDetails {
    /// Unique YouTube video ID
    pub id: String,
    /// Video title
    pub title: String,
    /// Video duration in seconds. [`None`] for livestreams.
    ///
    /// Info: This is only present in the `VideoDetails` player response
    pub duration: Option<u32>,
    /// Channel of the video
    pub channel: ChannelTag,
    /// List of words that describe the topic of the video
    ///
    /// Info: This is only present in the `VideoDetails` player response
    pub keywords: Vec<String>,
    /// Video description
    pub description: Option<String>,
    /// Video thumbnails
    ///
    /// Info: This is only present in the `VideoDetails` player response
    pub thumbnail: Vec<Thumbnail>,
    /// Number of views / current viewers in case of a livestream
    pub view_count: Option<u64>,
    /// Is the video a livestream?
    pub is_live: bool,
    /// Number of likes. [`None`] if hidden by the creator.
    pub like_count: Option<u32>,
    /// Category of the video
    ///
    /// Info: This is only present in the `VideoDetails` player response
    pub category: Option<String>,
    /// Is the video a YouTube Short?
    pub is_short: bool,
    /// Video publishing date. [`None`] if parsing failed/// Video publishing date. Start date in case of a livestream.
    ///
    /// [`None`] if the date could not be parsed.
    #[serde(with = "time::serde::rfc3339::option")]
    pub publish_date: Option<OffsetDateTime>,
    /// Textual video publishing date (e.g. `Aug 2, 2013`, depends on language)
    pub publish_date_text: Option<String>,
}
