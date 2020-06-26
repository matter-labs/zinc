//!
//! The Zinc virtual machine binary error.
//!

use std::io;

use failure::Fail;
use hex::FromHexError;

use zinc_bytecode::TemplateValueError;
use zinc_vm::RuntimeError;
use zinc_vm::VerificationError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{}: {}", path, error)]
    IO { error: io::Error, path: String },

    #[fail(display = "runtime error: {}", _0)]
    Runtime(RuntimeError),

    #[fail(display = "failed to verify")]
    Verification(VerificationError),

    #[fail(display = "failed to parse json: {}", _0)]
    JsonDecoding(serde_json::Error),

    #[fail(
        display = "invalid json structure: {}\nNote: remove the file ./data/witness.json so the compiler may recreate it",
        _0
    )]
    JsonValue(TemplateValueError),

    #[fail(display = "failed to decode program: {}", _0)]
    ProgramDecoding(String),

    #[fail(display = "failed to decode {} hex-code: {}", context, error)]
    HexDecoding {
        context: String,
        error: FromHexError,
    },
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

pub trait IErrorPath<T> {
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
