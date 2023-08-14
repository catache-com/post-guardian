use crate::models;
use anyhow::Result;
use deadpool_postgres::{Manager, ManagerConfig, Pool};
use models::PostgresDatabaseConfig;
use tokio_postgres::{Config, NoTls};

pub async fn init_postgres_connection(postgres_config: &PostgresDatabaseConfig) -> Result<Pool> {
    let mut pg_config = Config::new();
    pg_config
        .user(&postgres_config.postgres_db_username)
        .password(&postgres_config.postgres_db_password)
        .host(&postgres_config.postgres_db_host)
        .port(postgres_config.postgres_db_port)
        .dbname(&postgres_config.postgres_db_name);

    let manager = Manager::from_config(pg_config, NoTls, ManagerConfig::default());
    let pool = Pool::builder(manager)
        .max_size(postgres_config.postgres_db_max_connections)
        .build()?;
    Ok(pool)
}

pub struct PostgresGuardian {
    pool: Pool,
}

impl PostgresGuardian {
    pub async fn new(postgres_config: &PostgresDatabaseConfig) -> Result<Self> {
        let pool = init_postgres_connection(postgres_config).await?;
        Ok(PostgresGuardian { pool })
    }
}
