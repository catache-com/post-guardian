use axum::extract::FromRef;
use serde::{Deserialize, Serialize};

#[derive(FromRef, Serialize, Deserialize, Clone, Debug)]
pub struct PostgresDatabaseConfig {
    #[from_ref(skip)]
    pub postgres_db_name: String,

    #[from_ref(skip)]
    pub postgres_db_host: String,

    #[from_ref(skip)]
    pub postgres_db_port: u16,

    #[from_ref(skip)]
    pub postgres_db_username: String,

    #[from_ref(skip)]
    pub postgres_db_password: String,

    pub postgres_db_max_connections: usize,
}
