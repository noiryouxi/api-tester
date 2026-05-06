use crate::engine::http_engine;
use crate::models::http::{HttpRequest, HttpResponse};

/// Tauri command에서 사용하는 공통 Result 타입
/// - 현재는 String 에러를 사용하지만, 나중에 커스텀 에러 타입으로 확장 가능
type CommandResult<T> = Result<T, String>;

/// 프론트엔드에서 호출되는 HTTP 요청 실행 커맨드
/// 
/// 역할:
/// - HttpRequest를 받아서 http_engine에 전달
/// - 실행 결과를 그대로 반환하되, 에러 메시지에 컨텍스트를 추가
#[tauri::command]
pub async fn send_request(req: HttpRequest) -> CommandResult<HttpResponse> {
    // 어떤 URL로 요청이 가는지 확인 (디버깅용)
    println!("Sending request to: {}", req.url);

    // 실제 HTTP 요청 실행
    let result = http_engine::execute(req).await;

    // 에러 발생 시 stderr에 로그 출력 (디버깅용)
    if let Err(ref e) = result {
        eprintln!("Request failed: {}", e);
    }

    // 프론트엔드로 반환할 때는 에러 메시지에 컨텍스트를 추가
    result.map_err(|e| format!("HTTP request failed: {}", e))
}