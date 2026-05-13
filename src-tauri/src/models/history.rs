#[derive(Debug)]
pub struct HistoryRecord {
    pub id: i64,

    pub method: String,
    pub url: String,

    pub request_headers: String,
    pub request_body: String,

    pub status: i64,

    pub response_headers: String,
    pub response_body: String,

    pub created_at: String,
}