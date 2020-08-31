//!
//! The contract method templates resource GET request.
//!

use serde_derive::Deserialize;

///
/// The contract method templates resource GET query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The method contract ID.
    pub contract_id: i64,
    /// The method unique name within the template.
    pub name: String,
}
