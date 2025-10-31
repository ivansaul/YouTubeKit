use crate::models::thumbnail::Thumbnail;
use serde::Serialize;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Default)]
pub struct VideoDetails {
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
