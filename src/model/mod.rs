mod store;
pub mod orderbook;
mod base;
mod error;

use anyhow::Result;
use sqlx::{Pool, Postgres};
pub use store::*;

use crate::model::store::Database;

#[derive(Clone)]
pub struct ModelManager {
	db: Database,
}

impl ModelManager {
	pub async fn new(max_connections: u32, db_uri: &str) -> Result<Self> {
		let db = Database::open(max_connections, db_uri).await?;

		Ok(ModelManager { db })
	}

	pub(in crate::model) fn db(&self) -> &Pool<Postgres> {
		&self.db.pool
	}
}