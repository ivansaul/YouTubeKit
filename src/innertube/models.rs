use crate::{error::Result, innertube::config::*};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use strum::AsRefStr;

#[derive(Debug, AsRefStr, Default)]
pub enum ClientType {
    #[default]
    WEB,
    IOS,
}

impl ClientType {
    pub fn context(&self) -> Value {
        self.get_client_config()
            .map(|cfg| cfg.context)
            .unwrap_or_default()
    }

    pub fn headers(&self) -> HeaderMap {
        let mut map = HeaderMap::new();

        for (k, v) in self
            .get_client_config()
            .map(|cfg| cfg.header)
            .unwrap_or_default()
        {
            if let (Ok(name), Ok(value)) = (
                HeaderName::from_bytes(k.as_bytes()),
                HeaderValue::from_str(&v),
            ) {
                map.insert(name, value);
            }
        }

        map
    }
}

impl ClientType {
    fn get_client_config(&self) -> Option<ClientConfig> {
        let json = DEFAULT_CLIENTS_CACHE
            .get_or_init(|| self.load_default_clients_json())
            .as_ref()
            .ok()?;
        let value = json.get(self.as_ref())?;
        serde_json::from_value(value.clone()).ok()
    }

    fn load_default_clients_json(&self) -> Result<Value> {
        Ok(serde_json::from_str(DEFAULT_CLIENTS_JSON)?)
    }
}

#[derive(Debug, Deserialize)]
struct ClientConfig {
    #[serde(rename = "innertube_context")]
    context: Value,
    header: HashMap<String, String>,
}
