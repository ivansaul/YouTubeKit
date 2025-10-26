use crate::models::ClientType;
use common::error::Result;
use reqwest::{header::HeaderMap, Response};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

#[derive(Debug)]
pub struct Innertube {
    client_type: ClientType,
    client: reqwest::Client,
}

impl Innertube {
    pub fn new(client_type: ClientType) -> Self {
        Self {
            client_type,
            client: reqwest::Client::new(),
        }
    }

    pub async fn player<T: DeserializeOwned>(&self, video_id: &str) -> Result<T> {
        let body = json!({"videoId": video_id});
        let response = self
            .call_api("player", None, body.into())
            .await?
            .json::<T>()
            .await?;
        Ok(response)
    }

    pub async fn search<T: DeserializeOwned>(&self, query: &str) -> Result<T> {
        let body = json!({"query": query});
        let response = self
            .call_api("search", None, body.into())
            .await?
            .json::<T>()
            .await?;
        Ok(response)
    }

    fn base_url(&self) -> &str {
        "https://www.youtube.com/youtubei/v1"
    }
}

impl Innertube {
    fn context(&self) -> Value {
        self.client_type.context()
    }
    fn headers(&self) -> HeaderMap {
        self.client_type.headers()
    }

    async fn call_api(
        &self,
        endpoint: &str,
        query: Option<&[(&str, &str)]>,
        body: Option<Value>,
    ) -> Result<Response> {
        let endpoint_url = format!("{}/{}", self.base_url(), endpoint);

        dbg!(self.headers());
        dbg!(self.context());

        let mut base_body = self.context();

        if let (Some(obj), Some(obj_body)) = (
            base_body.as_object_mut(),
            body.unwrap_or_default().as_object(),
        ) {
            obj.extend(obj_body.clone());
        }

        let response = self
            .client
            .post(endpoint_url)
            .headers(self.headers())
            .query(&query)
            .json(&base_body)
            .send()
            .await?;

        Ok(response)
    }
}
