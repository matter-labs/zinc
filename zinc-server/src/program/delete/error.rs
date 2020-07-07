//!
//! The program resource DELETE error.
//!

use std::fmt;

use actix_web::http::StatusCode;
use actix_web::ResponseError;

///
/// The program resource DELETE error.
///
#[derive(Debug)]
pub enum Error {
    NotFound,
    MongoDb(zinc_mongo::Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::MongoDb(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
            Self::MongoDb(inner) => write!(f, "MongoDB: {}", inner),
        }
    }
}
