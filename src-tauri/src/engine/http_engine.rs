use reqwest::Client;
use crate::models::http::{HttpRequest, HttpResponse};

pub async fn execute(req: HttpRequest) -> Result<HttpResponse, String> {
    let client = Client::new();

    // HTTP method 분기
    let builder = match req.method.as_str() {
        "GET" => client.get(&req.url),
        "POST" => client.post(&req.url),
        "PUT" => client.put(&req.url),
        "DELETE" => client.delete(&req.url),
        _ => return Err("Unsupported HTTP method".into()),
    };

    // body 처리
    let builder = if let Some(body) = req.body {
        builder.body(body)
    } else {
        builder
    };

    // 요청 실행
    let res = builder
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = res.status().as_u16();
    let body = res.text().await.map_err(|e| e.to_string())?;

    Ok(HttpResponse { status, body })
}