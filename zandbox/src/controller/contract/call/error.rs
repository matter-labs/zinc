//!
//! The contract resource POST call error.
//!

use std::collections::HashMap;
use std::fmt;

use actix_web::http::StatusCode;
use actix_web::ResponseError;
use serde_json::Map as JsonMap;
use serde_json::Value as JsonValue;

use zksync::zksync_models::node::tx::TxHash;

use zinc_build::ValueError as BuildValueError;
use zinc_vm::RuntimeError;

///
/// The contract resource POST call error.
///
#[derive(Debug)]
pub enum Error {
    /// The contract with the specified ID is not found in the server cache.
    ContractNotFound,
    /// The specified method does not exist in the contract.
    MethodNotFound,
    /// The immutable method must be used via the `query` endpoint.
    MethodIsImmutable,
    /// Invalid contract method arguments.
    InvalidInput(BuildValueError),

    /// The virtual machine contract method runtime error.
    RuntimeError(RuntimeError),
    /// The PostgreSQL database error.
    Database(sqlx::Error),
    /// The ZkSync server error.
    ZkSync(zksync::error::ClientError),
    /// The ZkSync transfer errors.
    TransferFailure { reasons: HashMap<TxHash, String> },
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::ContractNotFound => StatusCode::NOT_FOUND,
            Self::MethodNotFound => StatusCode::UNPROCESSABLE_ENTITY,
            Self::MethodIsImmutable => StatusCode::BAD_REQUEST,
            Self::InvalidInput(_) => StatusCode::BAD_REQUEST,

            Self::RuntimeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ZkSync(_) => StatusCode::SERVICE_UNAVAILABLE,
            Self::TransferFailure { .. } => StatusCode::UNPROCESSABLE_ENTITY,
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
            Self::MethodIsImmutable => format!("Method is immutable: use 'query' instead"),
            Self::InvalidInput(inner) => format!("Input: {}", inner),

            Self::RuntimeError(inner) => format!("Runtime: {:?}", inner),
            Self::Database(inner) => format!("Database: {:?}", inner),
            Self::ZkSync(inner) => format!("ZkSync: {:?}", inner),
            Self::TransferFailure { reasons } => {
                let reasons: JsonMap<String, JsonValue> = reasons
                    .iter()
                    .map(|(hash, reason)| {
                        let mut hash = hash.to_string();
                        hash.drain(.."sync-tx:".len());
                        (hash, JsonValue::String(reason.to_owned()))
                    })
                    .collect();
                let reasons = serde_json::to_string_pretty(&reasons)
                    .expect(zinc_const::panic::DATA_CONVERSION);

                format!("Transfer failures: {}", reasons)
            }
        };

        log::warn!("{}", error);
        write!(f, "{}", error)
    }
}
