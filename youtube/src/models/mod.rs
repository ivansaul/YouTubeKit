pub mod thumbnail;
pub mod video_details;
use crate::models::thumbnail::Thumbnail;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ChannelTag {
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
pub struct VideoTag {
    pub id: String,
    pub name: String,
    pub thumbnail: Vec<Thumbnail>,
    pub publish_date: Option<String>,
    pub length_text: Option<String>,
    pub view_count: Option<String>,
    pub channel: Option<ChannelTag>,
    // pub is_short: bool,
    // pub is_live: bool,
    // pub description: Option<String>,
}
