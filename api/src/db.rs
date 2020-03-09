use std::env;

use deadpool_postgres::{Config, Pool};
use tokio_postgres::NoTls;

use crate::Error;

#[derive(Clone)]
pub struct DB {
    pool: Pool,
}

impl DB {
    pub fn new() -> Result<DB, deadpool_postgres::config::ConfigError> {
        let cfg = Config {
            user: env::var("POSTGRES_USER").ok(),
            password: env::var("POSTGRES_PW").ok(),
            dbname: env::var("POSTGRES_DB").ok(),
            options: None,
            application_name: None,
            ssl_mode: None,
            host: env::var("POSTGRES_HOST").ok(),
            hosts: None,
            port: env::var("POSTGRES_PORT").ok().map(|p| u16::from_str_radix(&p, 10).expect("Failed to convert port to integer")),
            ports: None,
            connect_timeout: None,
            keepalives: None,
            keepalives_idle: None,
            target_session_attrs: None,
            channel_binding: None,
            manager: None,
            pool: None,
        };
        let pool = cfg.create_pool(NoTls)?;

        Ok(DB {
            pool,
        })
    }

    pub async fn get(&self) -> Result<deadpool_postgres::Client, Error> {
        self.pool.get().await.map_err(Error::from)
    }

    pub async fn count_rows(&self, table: &str, client: &deadpool_postgres::Client) -> Result<Option<i64>, Error> {
        let stmt = client.prepare(&format!("SELECT COUNT(*) FROM {}", table)).await?;
        let rows = client.query(&stmt, &[]).await?;

        if rows.len() < 1 {
            Ok(None)
        } else {
            Ok(Some(rows[0].get(0)))
        }
    }
}
