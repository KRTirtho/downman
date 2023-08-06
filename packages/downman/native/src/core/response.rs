use reqwest::Response;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
    pub url: String,
}

impl HttpResponse {
    pub async fn from(response: Response) -> Self {
        let status = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .map(|(k, v)| (k.as_str().to_string(), v.to_str().unwrap().to_string()))
            .collect::<Vec<(String, String)>>();
        let url = response.url().to_string();
        let body = match response.bytes().await {
            Ok(bytes) => Some(bytes.to_vec()),
            Err(err) => {
                println!("Error reading response body");
                println!("{:?}", err);
                None
            }
        };

        Self {
            status,
            headers,
            body,
            url,
        }
    }
}
