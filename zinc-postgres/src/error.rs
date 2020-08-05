//!
//! The Zinc PostgreSQL error.
//!

use failure::Fail;

type PostgresqlError = sqlx::error::Error;

///
/// The Zinc PostgreSQL wrapper error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The PostgreSQL error.
    #[fail(display = "PostgreSQL: {}", _0)]
    Postgresql(PostgresqlError),
    /// The record is not present in the database.
    #[fail(display = "record not found")]
    RecordNotFound,
}

impl From<PostgresqlError> for Error {
    fn from(inner: PostgresqlError) -> Self {
        Self::Postgresql(inner)
    }
}
