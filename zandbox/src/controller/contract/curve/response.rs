//!
//! The contract resource GET `Curve` response.
//!

use serde_derive::Serialize;

///
/// The contract resource GET `Curve` response body.
///
pub type Body = Vec<Instance>;

///
/// The contract resource GET `Curve` response instance.
///
#[derive(Debug, Serialize)]
pub struct Instance {
    /// The contract account ID.
    pub account_id: i64,
    /// The contract project name.
    pub name: String,
    /// The contract version.
    pub version: String,
    /// The contract instance name.
    pub instance: String,
}

impl Instance {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(account_id: i64, name: String, version: String, instance: String) -> Self {
        Self {
            account_id,
            name,
            version,
            instance,
        }
    }
}
