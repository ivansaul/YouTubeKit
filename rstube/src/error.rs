use std::{borrow::Cow, fmt::Display};

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
    #[error("content unavailable ({reason}). Reason (from YT): {msg}")]
    Unavailable {
        /// Reason why the video could not be extracted
        reason: UnavailabilityReason,
        /// The error message as returned from YouTube
        msg: String,
    },
    /// Content with the given ID does not exist
    #[error("content `{id}` was not found ({msg})")]
    NotFound {
        /// ID of the requested content
        id: String,
        /// Error message
        msg: Cow<'static, str>,
    },
    /// YouTube returned data that does not match the queried ID
    ///
    /// Specifically YouTube may return this video <https://www.youtube.com/watch?v=aQvGIIdgFDM>,
    /// which is a 5 minute error message, instead of the requested video when using an outdated
    /// Android client.
    #[error("wrong result from YT: {0}")]
    WrongResult(String),
}

/// Reason why a video cannot be extracted
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnavailabilityReason {
    /// Video/Channel is age restricted.
    AgeRestricted,
    /// Video was deleted or censored
    Deleted,
    /// Video is not available in your country
    Geoblocked,
    /// Video cannot be extracted with the specified client
    UnsupportedClient,
    /// Video is private
    Private,
    /// Video needs to be purchased and is protected by digital restrictions management
    /// (e.g. movies and TV shows)
    Paid,
    /// Video is only available to YouTube Premium users
    Premium,
    /// Video is only available to channel members
    MembersOnly,
    /// Livestream has gone offline
    OfflineLivestream,
    /// YouTube banned your IP address from accessing the platform without an account
    IpBan,
    /// YouTube bans IP addresses from certain VPN providers from accessing certain geo-restricted
    /// videos.
    ///
    /// If this happens to you, you can try another server / VPN provider or disable your VPN.
    VpnBan,
    /// YouTube requires the user to solve a ReCaptcha
    Captcha,
    /// Video temporarily unavailable (rate limit)
    TryAgain,
    /// Video cant be played for other reasons
    #[default]
    Unplayable,
}

impl Display for UnavailabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnavailabilityReason::AgeRestricted => f.write_str("age-restricted"),
            UnavailabilityReason::Deleted => f.write_str("deleted"),
            UnavailabilityReason::Geoblocked => f.write_str("geoblocked"),
            UnavailabilityReason::UnsupportedClient => f.write_str("unsupported by client"),
            UnavailabilityReason::Private => f.write_str("private"),
            UnavailabilityReason::Paid => f.write_str("paid"),
            UnavailabilityReason::Premium => f.write_str("premium-only"),
            UnavailabilityReason::MembersOnly => f.write_str("members-only"),
            UnavailabilityReason::OfflineLivestream => f.write_str("offline stream"),
            UnavailabilityReason::IpBan => f.write_str("ip-ban"),
            UnavailabilityReason::VpnBan => f.write_str("vpn-ban"),
            UnavailabilityReason::Captcha => f.write_str("captcha"),
            UnavailabilityReason::TryAgain => f.write_str("try again"),
            UnavailabilityReason::Unplayable => f.write_str("unplayable"),
        }
    }
}
