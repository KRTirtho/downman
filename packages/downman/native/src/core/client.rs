use flutter_rust_bridge::support::lazy_static;
use reqwest::{Body, Method, Request, Url};

use super::{
    config::{BaseConfig, Config},
    response::HttpResponse,
};

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

pub struct HttpClient {
    pub base_config: Option<BaseConfig>,
}

impl HttpClient {
    pub fn new(config: Option<BaseConfig>) -> Self {
        Self {
            base_config: config,
        }
    }

    fn get_absolute_url(&self, path: String) -> Url {
        if path.starts_with("http://") || path.starts_with("https://") {
            return Url::parse(&path).unwrap();
        }

        match self.base_config.clone().and_then(|f| f.base_url) {
            Some(base_url) => Url::parse(&base_url).unwrap().join(&path).unwrap(),
            None => Url::parse(&path).unwrap(),
        }
    }

    async fn build_request(
        &self,
        method: Method,
        url: String,
        body: Option<String>,
        config: Option<Config>,
    ) -> HttpResponse {
        let mut request = Request::new(method, self.get_absolute_url(url));

        if let Some(config) = config {
            let merged_config = self
                .base_config
                .clone()
                .and_then(|f| Some(f.merge_config(config.clone())));

            if let Some(config) = merged_config {
                if let Some(timeout) = config.timeout_sec {
                    request
                        .timeout_mut()
                        .replace(std::time::Duration::from_secs(timeout as u64));
                }

                let headers = config.get_headers();

                if let Some(headers) = headers {
                    request.headers_mut().extend(headers);
                }
            } else {
                if let Some(timeout) = config.timeout_sec {
                    request
                        .timeout_mut()
                        .replace(std::time::Duration::from_secs(timeout as u64));
                }

                let headers = config.get_headers();

                if let Some(headers) = headers {
                    request.headers_mut().extend(headers);
                }
            }
        }

        if let Some(body) = body {
            request.body_mut().replace(Body::from(body));
        }

        let res = CLIENT.execute(request).await.unwrap().into();

        HttpResponse::from(res).await
    }

    pub async fn get(&self, url: String, config: Option<Config>) -> HttpResponse {
        self.build_request(Method::GET, url, None, config).await
    }

    pub async fn delete(&self, url: String, config: Option<Config>) -> HttpResponse {
        self.build_request(Method::DELETE, url, None, config).await
    }

    pub async fn head(&self, url: String, config: Option<Config>) -> HttpResponse {
        self.build_request(Method::HEAD, url, None, config).await
    }

    pub async fn options(&self, url: String, config: Option<Config>) -> HttpResponse {
        self.build_request(Method::OPTIONS, url, None, config).await
    }

    pub async fn post(
        &self,
        url: String,
        body: Option<String>,
        config: Option<Config>,
    ) -> HttpResponse {
        self.build_request(Method::POST, url, body, config).await
    }

    pub async fn put(
        &self,
        url: String,
        body: Option<String>,
        config: Option<Config>,
    ) -> HttpResponse {
        self.build_request(Method::PUT, url, body, config).await
    }

    pub async fn patch(
        &self,
        url: String,
        body: Option<String>,
        config: Option<Config>,
    ) -> HttpResponse {
        self.build_request(Method::PATCH, url, body, config).await
    }
}
