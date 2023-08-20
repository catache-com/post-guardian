use crate::models::server::ServerError;
use tracing::instrument;

#[instrument(level = "trace", skip_all)]
pub async fn new_table() -> Result<(), ServerError> {
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
