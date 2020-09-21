//!
//! The contract resource PUT query error.
//!

use std::fmt;

use actix_web::http::StatusCode;
use actix_web::ResponseError;

use zinc_build::ValueError as BuildValueError;
use zinc_vm::RuntimeError;

///
/// The contract run feature PUT query error.
///
#[derive(Debug)]
pub enum Error {
    ContractNotFound,
    MethodNotFound,
    MethodArgumentsNotFound,
    MethodIsMutable,
    InvalidInput(BuildValueError),
    InvalidStorage(BuildValueError),
    InvalidStorageSize { expected: usize, found: usize },
    RuntimeError(RuntimeError),
    Database(sqlx::Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::ContractNotFound => StatusCode::NOT_FOUND,
            Self::MethodNotFound => StatusCode::UNPROCESSABLE_ENTITY,
            Self::MethodArgumentsNotFound => StatusCode::UNPROCESSABLE_ENTITY,
            Self::MethodIsMutable => StatusCode::BAD_REQUEST,
            Self::InvalidInput(_) => StatusCode::BAD_REQUEST,
            Self::InvalidStorage(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidStorageSize { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Self::RuntimeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
            Self::ContractNotFound => format!("Contract not found"),
            Self::MethodNotFound => format!("Method not found"),
            Self::MethodArgumentsNotFound => format!("Method input arguments not found"),
            Self::MethodIsMutable => format!("Method is mutable: use 'call' instead"),
            Self::InvalidInput(inner) => format!("Input: {}", inner),
            Self::InvalidStorage(inner) => format!("Contract storage is invalid: {}", inner),
            Self::InvalidStorageSize { expected, found } => format!(
                "Contract storage size invalid: expected {}, found {}",
                expected, found
            ),
            Self::RuntimeError(inner) => format!("Runtime: {:?}", inner),
            Self::Database(inner) => format!("Database: {:?}", inner),
        };

        log::warn!("{}", error);
        write!(f, "{}", error)
    }
}
