//!
//! The Zinc server response status.
//!

use serde_derive::Serialize;

///
/// The response status.
///
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    /// The application success.
    Ok,
    /// The application error, which is returned in the adjacent `error` field.
    Error,
}
