//!
//! The template resource GET request.
//!

use serde_derive::Deserialize;

///
/// The template resource GET request query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The account ID of the template.
    pub account_id: i64,
}
