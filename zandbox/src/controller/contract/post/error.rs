//!
//! The contract resource POST response error.
//!

use std::fmt;

use actix_web::http::StatusCode;
use actix_web::ResponseError;

use zinc_build::ValueError as BuildValueError;
use zinc_vm::RuntimeError;

///
/// The contract resource POST response error.
///
#[derive(Debug)]
pub enum Error {
    InvalidBytecode(String),
    NotAContract,
    ConstructorNotFound,
    InvalidInput(BuildValueError),
    RuntimeError(RuntimeError),
    Database(sqlx::Error),
    InvalidStorage,
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidBytecode(_) => StatusCode::BAD_REQUEST,
            Self::NotAContract => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ConstructorNotFound => StatusCode::UNPROCESSABLE_ENTITY,
            Self::InvalidInput(_) => StatusCode::BAD_REQUEST,
            Self::RuntimeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidStorage => StatusCode::INTERNAL_SERVER_ERROR,
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
            Self::InvalidBytecode(inner) => write!(f, "Invalid bytecode: {}", inner),
            Self::NotAContract => write!(f, "Not a contract"),
            Self::ConstructorNotFound => write!(f, "Constructor not found"),
            Self::InvalidInput(inner) => write!(f, "Input: {}", inner),
            Self::RuntimeError(inner) => write!(f, "Runtime: {:?}", inner),
            Self::Database(inner) => write!(f, "Database: {:?}", inner),
            Self::InvalidStorage => write!(f, "Contract storage is broken"),
        }
    }
}
