use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    #[error("User not found")]
    OrderNotFound,
    #[error("Failed to open connection with database: {0}")]
    ConnectDatabase(String),
    #[error("Failed to execute the query: {0}")]
    Execute(String),
}

pub type Result<T> = std::result::Result<T, Error>;
