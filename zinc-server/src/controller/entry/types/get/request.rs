//!
//! The program entry templates resource GET request.
//!

use serde_derive::Deserialize;

///
/// The program entry templates resource GET query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The entry unique ID.
    pub id: i32,
}
