use serde::Deserialize;

/// Video or channel thumbnail image
#[derive(Debug, Clone, Deserialize)]
pub struct Thumbnail {
    pub url: String,
    pub width: i64,
    pub height: i64,
}
