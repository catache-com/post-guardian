use crate::models::server::ServerError;
use tracing::instrument;

#[instrument(level = "trace", skip_all)]
pub async fn new_table() -> Result<(), ServerError> {
    Ok(())
}
