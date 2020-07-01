//!
//! The program resource GET request.
//!

use serde_derive::Deserialize;

///
/// The program resource GET request.
///
#[derive(Debug, Deserialize)]
pub struct Request {
    /// The name of the requested program.
    pub name: String,
}
