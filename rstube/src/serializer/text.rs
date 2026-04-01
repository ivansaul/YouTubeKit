use serde::{Deserialize, Deserializer};
use serde_with::{serde_as, DeserializeAs};

/// # Text
///
/// The YouTube API has multiple ways of outputting text. This deserializer
/// is an attempt to unify them.
///
/// ```json
/// {
///   "text": "Hello World"
/// }
/// ```
///
/// ```json
/// {
///   "simpleText": "Hello World"
/// }
/// ```
///
/// Multiple "runs" aka components of text should be joined together
/// ```json
/// {
///   "runs": [
///     {"text": "Hello"},
///     {"text": " World"},
///   ]
/// }
/// ```
///

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum Text {
    Simple {
        #[serde(alias = "simpleText", alias = "content")]
        text: String,
    },
    Multiple {
        #[serde_as(as = "Vec<Text>")]
        runs: Vec<String>,
    },
    Str(String),
}

impl<'de> DeserializeAs<'de, String> for Text {
    fn deserialize_as<D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let text = Text::deserialize(deserializer)?;
        match text {
            Text::Simple { text } | Text::Str(text) => Ok(text),
            Text::Multiple { runs } => Ok(runs.join("")),
        }
    }
}

impl<'de> DeserializeAs<'de, Vec<String>> for Text {
    fn deserialize_as<D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let text = Text::deserialize(deserializer)?;
        match text {
            Text::Simple { text } | Text::Str(text) => Ok(vec![text]),
            Text::Multiple { runs } => Ok(runs),
        }
    }
}
