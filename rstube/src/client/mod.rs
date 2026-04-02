pub(crate) mod innertube;
pub mod search;
pub mod video;

use crate::{
    client::{
        innertube::{config::ClientContext, InnerTube},
        search::SearchBuilder,
        video::VideoBuilder,
    },
    locale::{Country, Language},
};
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Serialize)]
struct QBody<T> {
    context: ClientContext,
    #[serde(flatten)]
    body: T,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct QVideo<'a> {
    video_id: &'a str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct QContinuation<'a> {
    pub(crate) continuation: &'a str,
}

#[derive(Clone, Debug)]
pub struct RequestOpts {
    pub language: Language,
    pub country: Country,
}

impl Default for RequestOpts {
    fn default() -> Self {
        Self {
            language: Language::EnglishUS,
            country: Country::UnitedStates,
        }
    }
}

#[derive(Clone)]
pub struct RusTubeQuery {
    client: RusTube,
    opts: RequestOpts,
}

impl RusTubeQuery {
    pub fn lang(mut self, lang: Language) -> Self {
        self.opts.language = lang;
        self
    }

    pub fn location(mut self, loc: Country) -> Self {
        self.opts.country = loc;
        self
    }
}

impl RusTubeQuery {
    pub fn video(&self, id: impl Into<String>) -> VideoBuilder {
        VideoBuilder::new(self.client.clone(), id.into(), self.opts.clone())
    }

    pub fn search(&self, query: impl Into<String>) -> SearchBuilder {
        SearchBuilder::new(self.client.clone(), query.into(), self.opts.clone())
    }
}

#[derive(Debug, Clone)]
pub struct RusTube {
    pub(crate) inner: Arc<InnerTube>,
}

impl RusTube {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(InnerTube::new()),
        }
    }

    pub fn query(&self) -> RusTubeQuery {
        RusTubeQuery {
            client: self.clone(),
            opts: RequestOpts::default(),
        }
    }
}

impl Default for RusTube {
    fn default() -> Self {
        Self::new()
    }
}
