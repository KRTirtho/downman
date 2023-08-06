use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BaseConfig {
    pub base_url: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub timeout_sec: Option<usize>,
    pub max_redirects: Option<usize>,
    pub max_retries: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub headers: Option<HashMap<String, String>>,
    pub timeout_sec: Option<usize>,
}
