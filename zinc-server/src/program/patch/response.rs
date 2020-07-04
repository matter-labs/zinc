//!
//! The program resource PATCH response.
//!

use serde_derive::Serialize;

use crate::status::Status;

///
/// The program resource PATCH response.
///
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Response {
    /// The success data variant.
    Success(Success),
    /// The error data variant.
    Error(Error),
}

impl Response {
    ///
    /// A shortcut constructor.
    ///
    pub fn new_success() -> Self {
        Self::Success(Success::new())
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_error(error: String) -> Self {
        Self::Error(Error::new(error))
    }
}

///
/// The success data variant.
///
#[derive(Debug, Serialize)]
pub struct Success {
    /// The response status.
    pub status: Status,
}

impl Success {
    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self { status: Status::Ok }
    }
}

///
/// The error data variant.
///
#[derive(Debug, Serialize)]
pub struct Error {
    /// The response status.
    pub status: Status,
    /// The response error contents.
    pub error: String,
}

impl Error {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(error: String) -> Self {
        Self {
            status: Status::Error,
            error,
        }
    }
}
