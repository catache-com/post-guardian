use crate::handlers::requests::NewTableRequest;
use crate::models::server::ServerError;
use axum::{Extension, Json};
use deadpool_postgres::Pool;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, instrument};

#[instrument(level = "trace", skip_all)]
pub async fn new_table(
    Extension(postgres_pool): Extension<Arc<Mutex<Pool>>>,
    Json(payload): Json<NewTableRequest>,
) -> Result<(), ServerError> {
    let pool = postgres_pool.lock().await;

    let connection = pool.get().await.map_err(|e| {
        error!("error setting up postgres connection: {:?}", e);
        ServerError::internal_server_error()
    })?;

    connection
        .execute("CREATE TABLE {}", &[&payload.table_name])
        .await
        .map_err(|e| {
            error!("error creating a new table: {:?}", e);
            ServerError::internal_server_error()
        })?;

    Ok(())
}

#[instrument(level = "trace", skip_all)]
pub async fn table_info() -> Result<(), ServerError> {
    Ok(())
}

#[instrument(level = "trace", skip_all)]
pub async fn list_tables() -> Result<(), ServerError> {
    Ok(())
}

#[instrument(level = "trace", skip_all)]
pub async fn update_table_schema() -> Result<(), ServerError> {
    Ok(())
}

#[instrument(level = "trace", skip_all)]
pub async fn drop_table() -> Result<(), ServerError> {
    Ok(())
}
