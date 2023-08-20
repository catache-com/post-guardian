pub mod handle_table;
mod requests;

use axum::http::StatusCode;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
