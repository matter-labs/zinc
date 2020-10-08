//!
//! The contract resource PUT `query` error.
//!

use std::fmt;

use actix_web::http::StatusCode;
use actix_web::ResponseError;

use zinc_build::ValueError as BuildValueError;
use zinc_vm::RuntimeError;

///
/// The contract run feature PUT `query` error.
///
#[derive(Debug)]
pub enum Error {
    /// The contract with the specified address is not found in the server cache.
    ContractNotFound(String),
    /// The contract with the specified address is locked.
    ContractLocked(String),
    /// The specified method does not exist in the contract.
    MethodNotFound(String),
    /// The mutable method must be called via the `call` endpoint.
    MethodIsMutable(String),
    /// The method was specified in the query, but its arguments was not sent in the body.
    MethodArgumentsNotFound(String),
    /// Invalid contract method arguments.
    InvalidInput(BuildValueError),

    /// The virtual machine contract method runtime error.
    RuntimeError(RuntimeError),
    /// The PostgreSQL database error.
    Database(sqlx::Error),
    /// The ZkSync server client error.
    ZkSyncClient(zksync::error::ClientError),
}

impl From<sqlx::Error> for Error {
    fn from(inner: sqlx::Error) -> Self {
        Self::Database(inner)
    }
}

impl From<zksync::error::ClientError> for Error {
    fn from(inner: zksync::error::ClientError) -> Self {
        Self::ZkSyncClient(inner)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::ContractNotFound(..) => StatusCode::NOT_FOUND,
            Self::ContractLocked(..) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::MethodNotFound(..) => StatusCode::BAD_REQUEST,
            Self::MethodIsMutable(..) => StatusCode::BAD_REQUEST,
            Self::MethodArgumentsNotFound(..) => StatusCode::BAD_REQUEST,
            Self::InvalidInput(..) => StatusCode::BAD_REQUEST,

            Self::RuntimeError(..) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Database(..) => StatusCode::SERVICE_UNAVAILABLE,
            Self::ZkSyncClient(..) => StatusCode::SERVICE_UNAVAILABLE,
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
            Self::ContractNotFound(address) => {
                format!("Contract with address {} not found", address)
            }
            Self::ContractLocked(address) => format!("Contract with address {} is locked", address),
            Self::MethodNotFound(name) => format!("Method `{}` not found", name),
            Self::MethodIsMutable(name) => {
                format!("Method `{}` is mutable: use 'call' instead", name)
            }
            Self::MethodArgumentsNotFound(name) => {
                format!("Method `{}` input arguments missing in the request", name)
            }
            Self::InvalidInput(inner) => format!("Input: {}", inner),

            Self::RuntimeError(inner) => format!("Runtime: {:?}", inner),
            Self::Database(inner) => format!("Database: {:?}", inner),
            Self::ZkSyncClient(inner) => format!("ZkSync: {:?}", inner),
        };

        log::warn!("{}", error);
        write!(f, "{}", error)
    }
}
