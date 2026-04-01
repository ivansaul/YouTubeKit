use std::borrow::Cow;

use reqwest::header::InvalidHeaderValue;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error extracting content from YouTube
    #[error("extraction error: {0}")]
    Extraction(#[from] ExtractionError),

    /// Error from the HTTP client
    #[error(transparent)]
    Http(#[from] reqwest::Error),

    /// YouTube returned data that could not be deserialized or parsed
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    /// Bad request (Error 400 from YouTube), probably invalid input parameters
    #[error("bad request ({0})")]
    BadRequest(Cow<'static, str>),
}

impl From<InvalidHeaderValue> for Error {
    fn from(value: InvalidHeaderValue) -> Self {
        Error::BadRequest(value.to_string().into())
    }
}

/// Error extracting content from YouTube
#[derive(thiserror::Error, Debug)]
pub enum ExtractionError {
    /// YouTube returned data that could not be deserialized or parsed
    #[error("invalid data from YT: {0}")]
    InvalidData(Cow<'static, str>),

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
    /// YouTube returned data that does not match the queried ID
    ///
    /// Specifically YouTube may return this video <https://www.youtube.com/watch?v=aQvGIIdgFDM>,
    /// which is a 5 minute error message, instead of the requested video when using an outdated
    /// Android client.
    #[error("wrong result from YT: {0}")]
    WrongResult(String),
}
