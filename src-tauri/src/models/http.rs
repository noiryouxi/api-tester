use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct HttpRequest {
    pub url: String,
    pub method: String,
    pub body: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}