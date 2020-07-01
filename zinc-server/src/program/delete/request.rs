//!
//! The program resource DELETE request.
//!

use serde_derive::Deserialize;

///
/// The program resource DELETE request.
///
#[derive(Debug, Deserialize)]
pub struct Request {
    /// The name of the program to be deleted.
    pub name: String,
}
