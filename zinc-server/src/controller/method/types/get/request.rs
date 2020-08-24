//!
//! The contract method templates resource GET request.
//!

use serde_derive::Deserialize;

///
/// The contract method templates resource GET query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The method template ID.
    pub template_id: i64,
    /// The method unique name within the template.
    pub name: String,
}
