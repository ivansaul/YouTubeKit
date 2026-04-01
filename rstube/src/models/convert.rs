use crate::models::{VideoItem, YouTubeItem};

/// Trait for casting generic YouTube items to a specific kind.
///
/// Returns [`None`] if the item does not match.
pub trait FromYtItem: Sized {
    /// Casting from a generic YouTube item to a specific kind
    ///
    /// Returns [`None`] if the item does not match.
    fn from_yt_item(_item: YouTubeItem) -> Option<Self> {
        None
    }
}

impl FromYtItem for VideoItem {
    fn from_yt_item(item: YouTubeItem) -> Option<Self> {
        match item {
            YouTubeItem::Video(video) => Some(video),
        }
    }
}

impl FromYtItem for YouTubeItem {
    fn from_yt_item(item: YouTubeItem) -> Option<Self> {
        Some(item)
    }
}
