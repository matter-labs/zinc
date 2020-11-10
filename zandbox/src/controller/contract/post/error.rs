//!
//! The contract resource POST response error.
//!

use std::fmt;

use actix_web::http::StatusCode;
use actix_web::ResponseError;

///
/// The contract resource POST response error.
///
#[derive(Debug)]
pub enum Error {
    /// The uploaded bytecode is malformed.
    InvalidBytecode(String),
    /// The uploaded application is not a contract.
    NotAContract,
    /// The contract has no constructor.
    ConstructorNotFound,
    /// Invalid contract method arguments.
    InvalidInput(anyhow::Error),

    /// The virtual machine constructor runtime error.
    VirtualMachine(zinc_vm::Error),
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
            Self::InvalidBytecode(..) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::NotAContract => StatusCode::UNPROCESSABLE_ENTITY,
            Self::ConstructorNotFound => StatusCode::UNPROCESSABLE_ENTITY,
            Self::InvalidInput(..) => StatusCode::BAD_REQUEST,

            Self::VirtualMachine(..) => StatusCode::UNPROCESSABLE_ENTITY,
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
            Self::InvalidBytecode(inner) => format!("Invalid bytecode: {}", inner),
            Self::NotAContract => "Not a contract".to_owned(),
            Self::ConstructorNotFound => "Constructor not found".to_owned(),
            Self::InvalidInput(inner) => format!("Input: {}", inner),

            Self::VirtualMachine(inner) => format!("Runtime: {:?}", inner),
            Self::Database(inner) => format!("Database: {:?}", inner),
        };

        log::warn!("{}", error);
        write!(f, "{}", error)
    }
}
