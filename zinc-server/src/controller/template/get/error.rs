//!
//! The template resource GET error.
//!

use std::fmt;

use actix_web::http::StatusCode;
use actix_web::ResponseError;

///
/// The template resource GET error.
///
#[derive(Debug)]
pub enum Error {
    Database(sqlx::Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Database(sqlx::Error::RowNotFound) => StatusCode::NOT_FOUND,
            Self::Database(_inner) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(sqlx::Error::RowNotFound) => write!(f, "Database: not found"),
            Self::Database(inner) => write!(f, "Database: {}", inner),
        }
    }
}
