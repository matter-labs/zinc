//!
//! The Zinc server binary error.
//!

use std::io;

use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "PostgreSQL: {}", _0)]
    Postgresql(zinc_postgres::Error),
    #[fail(display = "server binding: {}", _0)]
    ServerBinding(io::Error),
    #[fail(display = "server runtime: {}", _0)]
    ServerRuntime(io::Error),
}

impl From<zinc_postgres::Error> for Error {
    fn from(error: zinc_postgres::Error) -> Self {
        Self::Postgresql(error)
    }
}
