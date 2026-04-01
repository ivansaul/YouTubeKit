pub mod continuation;
pub mod mapper;
pub mod player;
pub mod search;

use crate::models::Thumbnail;
use serde::Deserialize;

/// Wrapper for the YouTube API's thumbnail list format.
#[derive(Debug, Deserialize)]
pub struct Thumbnails {
    pub thumbnails: Vec<Thumbnail>,
}
