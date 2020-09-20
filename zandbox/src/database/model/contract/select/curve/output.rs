//!
//! The database contract Curve SELECT output model.
//!

///
/// The database contract Curve SELECT output model.
///
#[derive(Debug, sqlx::FromRow)]
pub struct Output {
    /// The contract account ID.
    pub account_id: i64,
    /// The contract project name.
    pub name: String,
    /// The contract version.
    pub version: String,
    /// The contract instance name.
    pub instance: String,
}

impl Output {
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
