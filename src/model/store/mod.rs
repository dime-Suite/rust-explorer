pub mod error;

// use super::
use anyhow::Ok;
use log::info;
use sqlx::postgres::PgPoolOptions;

/// SQLite database
#[derive(Debug, Clone)]
pub struct Database {
    /// Sqlite connection pool
    pub pool: sqlx::Pool<sqlx::Postgres>,
}

impl Database {
    pub async fn open(max_connections: u32, db_uri: &str) -> anyhow::Result<Self> {
        print!("Connecting to database...");
        let connection_pool = Self {
            pool: PgPoolOptions::new()
                .max_connections(max_connections)
                .connect(db_uri)
                .await
                .map_err(|err| {
                    anyhow::anyhow!("Failed to open connection with database: {}", err)
                    // Error::ConnectDatabase(format!(
                    //     "Failed to open connection with database: {}",
                    //     err
                    // ))
                })?,
        };

        info!("Database connection established.");
        match connection_pool {
            _ => {}
        }
        Ok(connection_pool)
    }

    //TODO: triggers
}
