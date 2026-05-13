use sqlx::{Pool, Sqlite};

use crate::models::http::{
    HttpRequest,
    HttpResponse,
};

pub async fn save_history(
    db: &Pool<Sqlite>,
    req: &HttpRequest,
    res: &HttpResponse,
) -> Result<(), String> {

    let request_headers = serde_json::to_string(
        &req.headers.clone().unwrap_or_default()
    ).unwrap_or_default();

    let request_body = req.body
    .clone()
    .unwrap_or_default();

    let response_headers = serde_json::to_string(
        &res.headers
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
    .bind(res.status as i64)
    .bind(response_headers)
    .bind(&res.body)
    .execute(db)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}