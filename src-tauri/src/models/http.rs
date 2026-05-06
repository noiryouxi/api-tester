use reqwest::Method;
use std::collections::HashMap;

/// HTTP 요청을 표현하는 공통 데이터 구조
///
/// 설계 의도:
/// - 특정 API에 종속되지 않는 범용 HTTP 모델
/// - 프론트(Tauri) ↔ 백엔드(Rust) 간 데이터 전달용
/// - reqwest에 바로 매핑 가능하도록 구성
pub struct HttpRequest {
    /// HTTP 메서드 (GET, POST 등)
    /// 
    /// String 대신 reqwest::Method 사용:
    /// - 유효하지 않은 메서드 방지 (타입 안정성)
    /// - reqwest와 바로 호환 가능
    pub method: Method,

    /// 요청 대상 URL
    /// 
    /// ex) https://api.example.com/users
    pub url: String,

    /// 요청 헤더
    /// 
    /// - Key-Value 형태
    /// - Authorization, Content-Type 등 확장 가능
    /// - Option으로 둬서 없는 경우 메모리/처리 낭비 방지
    pub headers: Option<HashMap<String, String>>,

    /// 요청 바디
    /// 
    /// Vec<u8> 선택 이유:
    /// - 텍스트(JSON, XML) + 바이너리(file) 모두 대응 가능
    /// - String보다 범용적
    ///
    /// 주의:
    /// - JSON 직렬화는 상위 레이어에서 처리하는 구조
    pub body: Option<Vec<u8>>,
}

/// HTTP 응답을 표현하는 구조
///
/// 설계 의도:
/// - UI에서 바로 쓰기 쉬운 단순한 형태 유지
/// - 최소 정보(status + body)만 포함
pub struct HttpResponse {
    /// HTTP 상태 코드
    /// 
    /// ex) 200, 404, 500
    pub status: u16,

    /// 응답 바디 (문자열)
    /// 
    /// 현재 정책:
    /// - text 기반 응답만 처리
    /// 
    /// 확장 가능:
    /// - Vec<u8>로 변경 (binary 대응)
    /// - enum으로 분기 (Text / Json / Bytes)
    pub body: String,
}