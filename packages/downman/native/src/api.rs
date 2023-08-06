use flutter_rust_bridge::{RustOpaque, SyncReturn};
use tokio::runtime::Runtime;

use crate::core::{
    client::HttpClient,
    config::{BaseConfig, Config},
    response::HttpResponse,
};

pub fn http_client_new(config: Option<BaseConfig>) -> SyncReturn<RustOpaque<HttpClient>> {
    let client = HttpClient::new(config);
    SyncReturn(RustOpaque::new(client))
}

pub fn ___why_http_client() -> HttpClient {
    HttpClient::new(None)
}

pub fn http_client_get(
    client: RustOpaque<HttpClient>,
    url: String,
    config: Option<Config>,
) -> HttpResponse {
    Runtime::new()
        .unwrap()
        .block_on(async { client.get(url, config).await })
}

pub fn http_client_post(
    client: RustOpaque<HttpClient>,
    url: String,
    body: Option<String>,
    config: Option<Config>,
) -> HttpResponse {
    Runtime::new()
        .unwrap()
        .block_on(async { client.post(url, body, config).await })
}

pub fn http_client_put(
    client: RustOpaque<HttpClient>,
    url: String,
    body: Option<String>,
    config: Option<Config>,
) -> HttpResponse {
    Runtime::new()
        .unwrap()
        .block_on(async { client.put(url, body, config).await })
}

pub fn http_client_patch(
    client: RustOpaque<HttpClient>,
    url: String,
    body: Option<String>,
    config: Option<Config>,
) -> HttpResponse {
    Runtime::new()
        .unwrap()
        .block_on(async { client.patch(url, body, config).await })
}

pub fn http_client_delete(
    client: RustOpaque<HttpClient>,
    url: String,
    config: Option<Config>,
) -> HttpResponse {
    Runtime::new()
        .unwrap()
        .block_on(async { client.delete(url, config).await })
}

pub fn http_client_options(
    client: RustOpaque<HttpClient>,
    url: String,
    config: Option<Config>,
) -> HttpResponse {
    Runtime::new()
        .unwrap()
        .block_on(async { client.options(url, config).await })
}

pub fn http_client_head(
    client: RustOpaque<HttpClient>,
    url: String,
    config: Option<Config>,
) -> HttpResponse {
    Runtime::new()
        .unwrap()
        .block_on(async { client.head(url, config).await })
}
