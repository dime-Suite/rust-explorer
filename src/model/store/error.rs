use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum Error {
    #[error("Failed to open connection with database: {0}")]
    ConnectDatabase(String),
}

pub type Result<T> = std::result::Result<T, Error>;
