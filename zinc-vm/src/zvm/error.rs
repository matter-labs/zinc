//!
//! The Zinc virtual machine binary error.
//!

use std::io;

use failure::Fail;
use hex::FromHexError;

use zinc_bytecode::TemplateValueError;
use zinc_vm::RuntimeError;
use zinc_vm::VerificationError;

///
/// The Zinc virtual machine error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The file input output error.
    #[fail(display = "{}: {}", path, error)]
    IO {
        /// The inner `std` error.
        error: io::Error,
        /// The path where the error has happened.
        path: String,
    },

    /// The bytecode execution runtime error.
    #[fail(display = "runtime error: {}", _0)]
    Runtime(RuntimeError),

    /// The proof verification error.
    #[fail(display = "failed to verify")]
    Verification(VerificationError),

    /// The JSON template file decoding error.
    #[fail(display = "failed to parse json: {}", _0)]
    JsonDecoding(serde_json::Error),

    /// The JSON template file data does not match the bytecode program input/output types metadata.
    #[fail(
        display = "invalid json structure: {}\nNote: remove the file ./data/witness.json so the compiler may recreate it",
        _0
    )]
    JsonValue(TemplateValueError),

    /// The bytecode deserialization error.
    #[fail(display = "failed to decode program: {}", _0)]
    ProgramDecoding(String),

    /// The hexadecimal data decoding error. Is caused by invalid proofs and keys.
    #[fail(display = "failed to decode {} hex-code: {}", context, error)]
    HexDecoding {
        /// The hexadecimal content description.
        context: String,
        /// The inner `hex` error.
        error: FromHexError,
    },

    /// The PostgreSQL library error.
    #[fail(display = "PostgreSQL: {}", _0)]
    Postgresql(zinc_postgres::Error),
}

impl From<RuntimeError> for Error {
    fn from(error: RuntimeError) -> Self {
        Error::Runtime(error)
    }
}

impl From<VerificationError> for Error {
    fn from(error: VerificationError) -> Self {
        Error::Verification(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::JsonDecoding(error)
    }
}

impl From<TemplateValueError> for Error {
    fn from(error: TemplateValueError) -> Self {
        Error::JsonValue(error)
    }
}

impl From<zinc_postgres::Error> for Error {
    fn from(error: zinc_postgres::Error) -> Self {
        Error::Postgresql(error)
    }
}

///
/// The trait for providing the path to IO errors.
///
pub trait IErrorPath<T> {
    ///
    /// Is used to simplify the `.map_err(...)` boilerplate code.
    ///
    fn error_with_path<P, F>(self, path: F) -> Result<T, Error>
    where
        P: Into<String>,
        F: FnOnce() -> P;
}

impl<T> IErrorPath<T> for Result<T, io::Error> {
    fn error_with_path<P, F>(self, path_fn: F) -> Result<T, Error>
    where
        P: Into<String>,
        F: FnOnce() -> P,
    {
        self.map_err(|error| Error::IO {
            error,
            path: path_fn().into(),
        })
    }
}
