use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// HTTP 요청을 표현하는 공통 데이터 구조
///
/// 설계 의도:
/// - 특정 API에 종속되지 않는 범용 HTTP 모델
/// - 프론트(Tauri) ↔ 백엔드(Rust) 간 데이터 전달용
/// - reqwest에 바로 매핑 가능하도록 구성
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpRequest {
    /// HTTP 메서드 (GET, POST 등)
    ///
    /// String 사용 이유:
    /// - reqwest::Method는 Tauri JSON deserialize 불가
    /// - 프론트에서 안전하게 전달 가능
    pub method: String,

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
    /// String 선택 이유:
    /// - JSON / text 기반 API 테스트에 최적
    /// - Tauri IPC에서 가장 안정적인 타입
    ///
    /// 주의:
    /// - JSON 직렬화/역직렬화는 상위 레이어에서 처리
    pub body: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpResponse {
    /// HTTP 상태 코드
    ///
    /// ex) 200, 404, 500
    pub status: u16,

    /// HTTP 상태 텍스트
    ///
    /// ex) OK, Not Found, Internal Server Error
    pub status_text: String,

    /// 응답 헤더
    ///
    /// - Key-Value 형태
    /// - 서버가 반환한 메타데이터 저장
    pub headers: HashMap<String, String>,

    /// 응답 바디 (문자열)
    ///
    /// 현재 정책:
    /// - text 기반 응답만 처리
    ///
    /// 확장 가능:
    /// - JSON parsing 추가
    /// - binary 대응 시 Vec<u8> 또는 base64로 확장 가능
    pub body: String,
}