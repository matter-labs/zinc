//!
//! The program input template resource GET response error.
//!

use std::fmt;

use actix_web::http::StatusCode;
use actix_web::ResponseError;

///
/// The program input template resource GET response error.
///
#[derive(Debug)]
pub enum Error {
    NotFound,
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
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
            Self::NotFound => write!(f, "Not found"),
        }
    }
}
