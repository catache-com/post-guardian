pub mod handle_table;

use axum::http::StatusCode;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
