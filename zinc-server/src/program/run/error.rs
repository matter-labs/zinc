//!
//! The program run feature POST response error.
//!

use std::fmt;

use actix_web::http::StatusCode;
use actix_web::ResponseError;

use zinc_bytecode::TemplateValueError;
use zinc_vm::RuntimeError;

///
/// The program run feature POST response error.
///
#[derive(Debug)]
pub enum Error {
    NotFound,
    InputError(TemplateValueError),
    RuntimeError(RuntimeError),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::InputError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::RuntimeError(_) => StatusCode::UNPROCESSABLE_ENTITY,
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
            Self::InputError(inner) => write!(f, "Invalid input: {}", inner),
            Self::RuntimeError(inner) => write!(f, "Runtime error: {}", inner),
        }
    }
}
