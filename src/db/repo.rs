use serde_derive::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool};

use crate::configs::db_config::DbConfig;

#[derive(Deserialize, Serialize, Debug)]
pub struct Repository {
    config: DbConfig
}

impl Repository {
    fn init_db_config() -> DbConfig {
        let mut db_config: DbConfig = DbConfig::from_env();
        db_config.get_connection_url();
        return db_config;
    }

    pub fn new () -> Repository {
        let db_config: DbConfig = Self::init_db_config();
        Repository {
            config: db_config
        }
    }
    pub async fn start_connection_pool(&mut self) -> Pool<sqlx::Postgres> {
        let max_conn: u32 = self.config.get_max_connections();
        let url: &str = self.config.get_connection_url();
        let pool: Pool<sqlx::Postgres> = PgPoolOptions::new()
            .max_connections(max_conn)
            .connect(url)
            .await
            .expect("Unable to get connection pool database");
        return pool;
    }
}