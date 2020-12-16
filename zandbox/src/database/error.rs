//!
//! The Zandbox database error.
//!

use thiserror::Error;

///
/// The Zandbox database error.
///
#[derive(Debug, Error)]
pub enum Error {
    /// The record does not exist in the database.
    #[error("{entity} not found")]
    NotFound {
        /// The name of the missing entity.
        entity: String,
    },
    /// The record already exists in the database.
    #[error("{entity} already exists")]
    AlreadyExists {
        /// The name of the duplicate entity.
        entity: String,
    },
    /// The inner database error.
    #[error("{0}")]
    Other(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Self::Other(error)
    }
}

impl From<(sqlx::Error, &'static str)> for Error {
    fn from((error, entity): (sqlx::Error, &'static str)) -> Self {
        match error {
            sqlx::Error::RowNotFound => Self::NotFound {
                entity: entity.to_owned(),
            },
            sqlx::Error::Database(inner) if inner.code().unwrap_or_default() == "23505" => {
                Self::AlreadyExists {
                    entity: entity.to_owned(),
                }
            }
            other => Self::Other(other),
        }
    }
}
