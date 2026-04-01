pub mod text;

use crate::{client::innertube::endpoint::YTEndpoint, error::ExtractionError, locale::Language};
use serde::{
    de::{SeqAccess, Visitor},
    Deserialize,
};
use std::fmt::Debug;
use std::{fmt, marker::PhantomData};

/// Additional data needed for mapping YouTube responses
pub(crate) struct MapRespCtx<'a> {
    /// ID of the requested entity (Video ID, Channel ID, ...)
    pub id: &'a str,
    pub lang: Language,
    /// YouTube API endpoint this response belongs to (used for continuations)
    pub endpoint: YTEndpoint,
}

/// Implement this for YouTube API response structs that need to be mapped to
/// RusTube models.
pub(crate) trait MapResponse<T> {
    fn map_response(
        self,
        ctx: &MapRespCtx<'_>,
    ) -> std::result::Result<MapResult<T>, ExtractionError>;
}

/// Result of a deserializing/mapping operation.
/// Holds the desired `content` and warnings for non-fatal issues.
#[derive(Clone)]
pub(crate) struct MapResult<T> {
    pub content: T,
    pub warnings: Vec<String>,
}

impl<T: Default> Default for MapResult<T> {
    fn default() -> Self {
        Self {
            content: T::default(),
            warnings: Vec::new(),
        }
    }
}

impl<T: Debug> Debug for MapResult<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.content.fmt(f)
    }
}

/// Deserializes a list of arbitrary items into a `MapResult`,
/// creating warnings for items that could not be deserialized.
impl<'de, T: Deserialize<'de>> Deserialize<'de> for MapResult<Vec<T>> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        #[serde(untagged)]
        enum GoodOrError<T> {
            Good(T),
            Error(serde_json::Value),
        }

        struct SeqVisitor<T>(PhantomData<T>);

        impl<'de, T: Deserialize<'de>> Visitor<'de> for SeqVisitor<T> {
            type Value = MapResult<Vec<T>>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut values = Vec::with_capacity(seq.size_hint().unwrap_or_default());
                let mut warnings = Vec::new();

                while let Some(value) = seq.next_element()? {
                    match value {
                        GoodOrError::<T>::Good(value) => values.push(value),
                        GoodOrError::<T>::Error(value) => warnings.push(format!(
                            "error deserializing item: {}",
                            serde_json::to_string(&value).unwrap_or_default()
                        )),
                    }
                }
                Ok(MapResult {
                    content: values,
                    warnings,
                })
            }
        }

        deserializer.deserialize_seq(SeqVisitor(PhantomData::<T>))
    }
}
