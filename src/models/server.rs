use axum::http::StatusCode;

#[derive(Debug)]
pub struct ServerError {
    pub message: Option<String>,
    pub status_code: StatusCode,
}
