//!
//! The program resource PUT response error.
//!

use std::fmt;

use actix_web::http::StatusCode;
use actix_web::ResponseError;

///
/// The program resource PUT response error.
///
#[derive(Debug)]
pub enum Error {
    Compiler(String),
    MongoDb(zinc_mongo::Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Compiler(_) => StatusCode::UNPROCESSABLE_ENTITY,
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
            Self::Compiler(inner) => write!(f, "{}", inner),
            Self::MongoDb(inner) => write!(f, "MongoDB: {}", inner),
        }
    }
}
