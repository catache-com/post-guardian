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
        error!("error getting a postgres connection from the pool: {:?}", e);
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
pub async fn list_tables(
    Extension(postgres_pool): Extension<Arc<Mutex<Pool>>>,
) -> Result<Vec<String>, ServerError> {
    let pool = postgres_pool.lock().await;

    let connection = pool.get().await.map_err(|e| {
        error!("error getting a postgres connection from the pool: {:?}", e);
        ServerError::internal_server_error()
    })?;

    let rows = connection
        .query(
            "SELECT tablename FROM pg_catalog.pg_tables
        WHERE schemaname != 'pg_catalog' AND schemaname != 'information_schema'",
            &[],
        )
        .await
        .map_err(|e| {
            error!("error listing table names: {:?}", e);
            ServerError::internal_server_error()
        })?;

    let table_names: Vec<String> = rows.iter().map(|row| row.get("tablename")).collect();

    Ok(table_names)
}

#[instrument(level = "trace", skip_all)]
pub async fn update_table_schema() -> Result<(), ServerError> {
    Ok(())
}

#[instrument(level = "trace", skip_all)]
pub async fn drop_table() -> Result<(), ServerError> {
    Ok(())
}
