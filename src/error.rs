// #[derive(Debug, uniffi::Error, thiserror::Error)]
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
