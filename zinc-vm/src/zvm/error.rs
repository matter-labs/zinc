//!
//! The Zinc virtual machine binary error.
//!

use thiserror::Error;

///
/// The Zinc virtual machine error.
///
#[derive(Debug, Error)]
pub enum Error {
    /// The file input output error.
    #[error("{path}: {error}")]
    IO {
        /// The inner `std` error.
        error: std::io::Error,
        /// The path where the error has happened.
        path: String,
    },

    /// The bytecode execution runtime error.
    #[error("runtime error: {0}")]
    Runtime(#[from] zinc_vm::Error),

    /// The proof verification error.
    #[error("failed to verify")]
    Verification(#[from] zinc_vm::VerificationError),

    /// The JSON template file decoding error.
    #[error("failed to parse json: {0}")]
    JsonDecoding(#[from] serde_json::Error),

    /// The JSON template file data does not match the bytecode application input/output types metadata.
    #[error(
        "invalid json structure: {0}\nNote: remove the JSON file so the compiler may recreate it"
    )]
    JsonInput(#[from] anyhow::Error),

    /// The bytecode deserialization error.
    #[error("failed to decode an application: {0}")]
    ApplicationDecoding(String),

    /// The input data is invalid.
    #[error("the input data is invalid: expected `{expected}`, found `{found}`")]
    InputDataInvalid {
        /// The expected project type.
        expected: String,
        /// The found project type.
        found: String,
    },

    /// The method name is not specified.
    #[error("method name is missing")]
    MethodNameNotFound,

    /// The method does not exist in the contract.
    #[error("method `{name}` not found")]
    MethodNotFound { name: String },

    /// The method arguments are not present in the input data.
    #[error("method `{name}` arguments not found")]
    MethodArgumentsNotFound { name: String },

    /// The transaction JSON is invalid.
    #[error("transaction `{found}` is invalid: {inner}")]
    InvalidTransaction {
        inner: zinc_types::TransactionError,
        found: serde_json::Value,
    },

    /// The contract storage JSON is invalid.
    #[error("contract storage must be an array, but found `{found}`")]
    InvalidContractStorageFormat { found: serde_json::Value },

    /// The library cannot be run as a standalone application.
    #[error("libraries cannot be run as they have no entry points")]
    CannotRunLibrary,
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

impl<T> IErrorPath<T> for Result<T, std::io::Error> {
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
