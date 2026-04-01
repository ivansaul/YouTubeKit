use crate::models::thumbnail::Thumbnail;

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
