use std::collections::HashMap;

use reqwest::header::HeaderMap;

#[derive(Debug, Clone)]
pub struct BaseConfig {
    pub base_url: Option<String>,
    pub headers: Option<Vec<(String, String)>>,
    pub timeout_sec: Option<usize>,
}

impl BaseConfig {
    pub fn merge_config(&self, config: Config) -> Self {
        Self {
            base_url: self.base_url.clone(),
            headers: self.headers.clone().and_then(|mut f| {
                if let Some(headers) = config.headers {
                    f.extend(headers);
                }
                Some(f)
            }),
            timeout_sec: config.timeout_sec,
        }
    }

    pub fn get_headers(&self) -> Option<HeaderMap> {
        if let Some(headers) = self.headers.clone() {
            let map = headers
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<HashMap<String, String>>();

            HeaderMap::try_from(&map).ok()
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub headers: Option<Vec<(String, String)>>,
    pub timeout_sec: Option<usize>,
}

impl Config {
    pub fn get_headers(&self) -> Option<HeaderMap> {
        if let Some(headers) = self.headers.clone() {
            let map = headers
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<HashMap<String, String>>();

            HeaderMap::try_from(&map).ok()
        } else {
            None
        }
    }
}
