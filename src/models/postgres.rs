use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostgresDatabaseConfig {
    pub postgres_db_name: String,

    pub postgres_db_host: String,

    pub postgres_db_port: u16,

    pub postgres_db_username: String,

    pub postgres_db_password: String,

    pub postgres_db_max_connections: usize,
}
