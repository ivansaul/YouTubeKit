use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, uniffi::Record)]
pub struct Thumbnail {
    pub url: String,
    pub width: i64,
    pub height: i64,
}
