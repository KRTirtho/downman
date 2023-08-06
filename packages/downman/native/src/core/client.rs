use reqwest::{header::HeaderMap, Method, Url};

use super::{
    config::{BaseConfig, Config},
    response::HttpResponse,
};

pub struct HttpClient {
    base_config: Option<BaseConfig>,
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new(config: Option<BaseConfig>) -> Self {
        let mut client = reqwest::Client::builder();

        if let Some(config) = config.clone() {
            if let Some(timeout) = config.timeout_sec {
                client = client.timeout(std::time::Duration::from_secs(timeout as u64));
            }

            if let Some(max_redirects) = config.max_redirects {
                client = client.redirect(reqwest::redirect::Policy::limited(max_redirects));
            }

            if let Some(headers) = config.headers {
                client = client.default_headers(HeaderMap::try_from(&headers).unwrap());
            }
        }

        Self {
            base_config: config,
            client: client.build().unwrap(),
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
        let mut request = self.client.request(method, self.get_absolute_url(url));

        if let Some(config) = config {
            if let Some(timeout) = config.timeout_sec {
                request = request.timeout(std::time::Duration::from_secs(timeout as u64));
            }

            if let Some(headers) = config.headers {
                request = request.headers(HeaderMap::try_from(&headers).unwrap());
            }
        }

        if let Some(body) = body {
            request = request.body(body);
        }

        HttpResponse::from(self.client.execute(request.build().unwrap()).await.unwrap()).await
    }

    pub async fn get(&self, url: String, config: Option<Config>) -> HttpResponse {
        self.build_request(Method::GET, url, None, config).await
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

    pub async fn delete(&self, url: String, config: Option<Config>) -> HttpResponse {
        self.build_request(Method::DELETE, url, None, config).await
    }

    pub async fn head(&self, url: String, config: Option<Config>) -> HttpResponse {
        self.build_request(Method::HEAD, url, None, config).await
    }

    pub async fn patch(
        &self,
        url: String,
        body: Option<String>,
        config: Option<Config>,
    ) -> HttpResponse {
        self.build_request(Method::PATCH, url, body, config).await
    }

    pub async fn options(&self, url: String, config: Option<Config>) -> HttpResponse {
        self.build_request(Method::OPTIONS, url, None, config).await
    }
}
