// #[derive(Debug, thiserror::Error)]
// pub enum Error {
//     #[error("{0}")]
//     Request(#[from] reqwest::Error),

//     #[error("{0}")]
//     SerdeJson(#[from] serde_json::Error),
// }

// pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, uniffi::Error, thiserror::Error)]
pub enum Error {
    #[error("Request error: {0}")]
    Request(String),

    #[error("JSON error: {0}")]
    SerdeJson(String),

    #[error("extraction error: {0}")]
    Extraction(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Request(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeJson(err.to_string())
    }
}

impl From<ExtractionError> for Error {
    fn from(err: ExtractionError) -> Self {
        Error::Extraction(err.to_string())
    }
}

/// Error extracting content from YouTube
#[derive(thiserror::Error, Debug)]
pub enum ExtractionError {
    /// YouTube returned data that could not be deserialized or parsed
    #[error("invalid data from YT: {0}")]
    InvalidData(&'static str),

    /// Content cannot be extracted
    ///
    /// Reasons include:
    /// - Deletion/Censorship
    /// - Age restriction
    /// - Private video
    /// - DRM (Movies and TV shows)
    #[error("content unavailable ({reason})")]
    Unavailable {
        /// Reason why the video could not be extracted
        reason: String,
    },
}
