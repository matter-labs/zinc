//!
//! The program resource DELETE request.
//!

use serde_derive::Deserialize;

///
/// The program resource DELETE query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The name of the program to be deleted.
    pub name: String,
}
