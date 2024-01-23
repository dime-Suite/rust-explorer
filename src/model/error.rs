use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use super::store;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    EntityNotFound {
        entity: String,
        id: i64,
    },

    #[from]
    Store(store::error::Error),

    #[from]
    SeaQuery(#[serde_as(as = "DisplayFromStr")] sea_query::error::Error),

    #[from]
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
