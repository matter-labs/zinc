//!
//! The program output template resource GET response.
//!

use serde_derive::Serialize;
use serde_json::Value as JsonValue;

use crate::status::Status;

///
/// The program output template resource GET response.
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
    pub fn new_success(template: JsonValue) -> Self {
        Self::Success(Success::new(template))
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
    /// The program entry output template.
    pub template: JsonValue,
}

impl Success {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(template: JsonValue) -> Self {
        Self {
            status: Status::Ok,
            template,
        }
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
