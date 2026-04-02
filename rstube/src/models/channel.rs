use crate::models::thumbnail::Thumbnail;
use serde::Deserialize;

/// Lightweight channel preview used in search results and video details
#[derive(Debug)]
pub struct ChannelPreview {
    /// Unique YouTube channel ID
    pub id: String,
    /// Channel name
    pub name: String,
    /// Channel label
    pub label: String,
    /// Channel avatar/profile picture
    pub avatar: Vec<Thumbnail>,
    /// Approximate number of subscribers.
    ///
    /// [`None`] if hidden by the owner or not present.
    pub subscriber_count: Option<u64>,
}

/// Channel information attached to a video or comment
#[derive(Debug, Default, Deserialize)]
pub struct ChannelTag {
    /// Unique YouTube channel ID
    pub id: String,
    /// Channel name
    pub name: String,
    /// YouTube channel handle (e.g. @YouTube)
    pub handle: Option<String>,
    /// Channel avatar/profile picture
    pub avatar: Vec<Thumbnail>,
    /// Channel verification mark
    pub verification: Verification,
    /// Approximate number of subscribers
    ///
    /// [`None`] if hidden by the owner or not present.
    ///
    /// Info: This is only present in the `VideoDetails` response
    pub subscriber_count: Option<String>,
}

/// Verification status of a channel
#[derive(Debug, Default, Deserialize, PartialEq)]
pub enum Verification {
    #[default]
    /// Unverified channel (default)
    None,
    /// Verified channel (✓ checkmark symbol)
    Verified,
    /// Verified music artist (♪ music note symbol)
    Artist,
}
