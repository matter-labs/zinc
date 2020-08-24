//!
//! The Zinc server binary error.
//!

use std::io;

use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Database: {}", _0)]
    Database(sqlx::Error),
    #[fail(display = "server binding: {}", _0)]
    ServerBinding(io::Error),
    #[fail(display = "server runtime: {}", _0)]
    ServerRuntime(io::Error),
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}
