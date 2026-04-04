use serde::Deserialize;
use time::OffsetDateTime;

use crate::models::{channel::ChannelTag, thumbnail::Thumbnail};

/// YouTube video item from search results, recommendations or playlists
#[derive(Debug, Deserialize)]
pub struct VideoItem {
    /// Unique YouTube video ID
    pub id: String,
    /// Video title
    pub name: String,
    /// Video thumbnails
    pub thumbnails: Vec<Thumbnail>,
    /// Channel of the video
    pub channel: Option<ChannelTag>,
    /// Video duration in seconds.
    ///
    /// Is [`None`] for livestreams.
    pub duration: Option<u32>,
    /// View count
    ///
    /// [`None`] if it could not be extracted.
    pub view_count: Option<u64>,
    /// Is the video an active livestream?
    pub is_live: bool,
    /// Is the video a YouTube Short video (vertical and <60s)?
    pub is_short: bool,
    /// Is the video announced, but not released yet (YouTube Premiere)?
    pub is_upcoming: bool,
    /// Video publishing date.
    ///
    /// [`None`] if the date could not be parsed.
    #[serde(with = "time::serde::rfc3339::option")]
    pub publish_date: Option<OffsetDateTime>,
    /// Textual video publish date (e.g. `11 months ago`, depends on language)
    ///
    /// Is [`None`] for livestreams and upcoming videos.
    pub publish_date_txt: Option<String>,
    /// Abbreviated video description
    pub short_description: Option<String>,
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
