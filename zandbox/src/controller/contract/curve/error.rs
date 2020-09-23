//!
//! The contract resource GET `Curve` response error.
//!

use std::fmt;

use actix_web::http::StatusCode;
use actix_web::ResponseError;

///
/// The contract resource GET `Curve` response error.
///
#[derive(Debug)]
pub enum Error {
    /// The PostgreSQL database error.
    Database(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(inner: sqlx::Error) -> Self {
        Self::Database(inner)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Database(..) => StatusCode::SERVICE_UNAVAILABLE,
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
        let error = match self {
            Self::Database(inner) => format!("Database: {:?}", inner),
        };

        log::warn!("{}", error);
        write!(f, "{}", error)
    }
}
