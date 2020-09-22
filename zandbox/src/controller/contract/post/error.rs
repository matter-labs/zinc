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
    Web3(zksync::web3::Error),
    ZkSync(zksync::error::ClientError),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidBytecode(..) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::NotAContract => StatusCode::UNPROCESSABLE_ENTITY,
            Self::ConstructorNotFound => StatusCode::UNPROCESSABLE_ENTITY,
            Self::InvalidInput(..) => StatusCode::BAD_REQUEST,

            Self::RuntimeError(..) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Database(..) => StatusCode::SERVICE_UNAVAILABLE,
            Self::Web3(..) => StatusCode::SERVICE_UNAVAILABLE,
            Self::ZkSync(..) => StatusCode::SERVICE_UNAVAILABLE,
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
            Self::InvalidBytecode(inner) => format!("Invalid bytecode: {}", inner),
            Self::NotAContract => "Not a contract".to_owned(),
            Self::ConstructorNotFound => "Constructor not found".to_owned(),
            Self::InvalidInput(inner) => format!("Input: {}", inner),

            Self::RuntimeError(inner) => format!("Runtime: {:?}", inner),
            Self::Database(inner) => format!("Database: {:?}", inner),
            Self::Web3(inner) => format!("ZkSync Web3: {:?}", inner),
            Self::ZkSync(inner) => format!("ZkSync: {:?}", inner),
        };

        log::warn!("{}", error);
        write!(f, "{}", error)
    }
}
