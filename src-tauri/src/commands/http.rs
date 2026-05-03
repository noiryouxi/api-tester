use crate::models::http::HttpRequest;
use crate::engine::http_engine;

#[tauri::command]
pub async fn send_request(req: HttpRequest)
    -> Result<crate::models::http::HttpResponse, String>
{
    http_engine::execute(req).await
}