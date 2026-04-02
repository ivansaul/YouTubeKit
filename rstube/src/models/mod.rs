pub mod channel;
pub mod convert;
pub mod paginator;
pub mod thumbnail;
pub mod video;

// Re-export top-level types for convenience
pub use thumbnail::Thumbnail;
pub use video::{VideoDetails, VideoItem, VideoPreview};

/// Discriminated union of all YouTube content types.
///
/// Used in mixed-content lists (search results, recommendations)
/// where the type of each item is determined at runtime.
#[derive(Debug)]
pub enum YouTubeItem {
    /// YouTube video item
    Video(VideoItem),
}
