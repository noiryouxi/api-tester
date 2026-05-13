use reqwest::{Client, Method, Response};
use crate::models::http::{HttpRequest, HttpResponse};
use std::collections::HashMap;

use sqlx::{
    Pool,
    Sqlite,
};

/// HTTP 요청을 실행하는 핵심 함수
///
/// 역할:
/// - HttpRequest를 받아 reqwest 요청으로 변환
/// - headers / body를 조건적으로 적용
/// - 실제 네트워크 요청 수행
/// - 응답을 HttpResponse로 변환
pub async fn execute(
    req: HttpRequest,
    db: &Pool<Sqlite>,
) -> Result<HttpResponse, String> {

    // reqwest 클라이언트 생성
    // (필요하면 timeout, proxy, retry 등 설정 가능)
    let client = Client::new();

    let method: Method = req.method
        .parse()
        .map_err(|e| format!("Invalid HTTP method: {}", e))?;

    // method + url 기반으로 요청 빌더 생성
    let mut builder = client.request(method, &req.url);

    // headers 적용
    // - HashMap<String, String> 형태를 순회하면서 header 추가
    // - 인증 토큰, Content-Type 등 확장 포인트
    if let Some(ref headers) = req.headers {
        for (key, value) in headers {
            builder = builder.header(key, value);
        }
    }

    // body 적용
    // - 현재는 Vec<u8> 기반 (텍스트 + 바이너리 모두 대응)
    // - JSON은 상위 레이어에서 serialize 후 전달하는 구조
    if let Some(ref body) = req.body {
        builder = builder.body(body.clone());
    }

    println!("Method: {:?}", req.method);
    println!("URL: {}", req.url);
    
    // 실제 HTTP 요청 실행
    // - 네트워크 에러 발생 시 String으로 변환해서 전달
    let response = builder
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    // 응답을 공통 HttpResponse 구조로 변환
    let http_response = build_response(response).await?;

    // -----------------------------
    // SQLite history 저장
    // -----------------------------

    let request_headers = serde_json::to_string(
        &req.headers.unwrap_or_default()
    ).unwrap_or_default();

    let request_body = req.body
    .clone()
    .unwrap_or_default();

    let response_headers = serde_json::to_string(
        &http_response.headers
    ).unwrap_or_default();

    sqlx::query(
        r#"
        INSERT INTO http_history (
            method,
            url,
            request_headers,
            request_body,
            status,
            response_headers,
            response_body
        )
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&req.method)
    .bind(&req.url)
    .bind(request_headers)
    .bind(request_body)
    .bind(http_response.status as i64)
    .bind(response_headers)
    .bind(&http_response.body)
    .execute(db)
    .await
    .map_err(|e| format!("Failed to save history: {}", e))?;

    Ok(http_response)
}

/// reqwest::Response → HttpResponse 변환
///
/// 역할:
/// - 상태 코드 추출
/// - body를 텍스트로 읽기
/// 
/// 주의:
/// - 현재는 text()만 사용 (binary 응답은 손실됨)
/// - 필요하면 bytes() 또는 content-type 기반 분기 가능
async fn build_response(res: Response) -> Result<HttpResponse, String> {
    // status
    let status = res.status();

    // headers 변환
    let mut headers = HashMap::new();
    for (key, value) in res.headers().iter() {
        headers.insert(
            key.to_string(),
            value.to_str().unwrap_or("").to_string(),
        );
    }

    // body
    let body = res
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    println!("status: {:?}", status);
    println!("Body: {:?}", body);   

    Ok(HttpResponse {
        status: status.as_u16(),

        status_text: status
            .canonical_reason()
            .unwrap_or("")
            .to_string(),

        headers,

        body,
    })
}