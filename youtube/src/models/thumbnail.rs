use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Thumbnail {
    pub url: String,
    pub width: i64,
    pub height: i64,
}
