//!
//! The program source resource GET request.
//!

use serde_derive::Deserialize;

///
/// The program source resource GET query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The program unique ID.
    pub id: i32,
}
