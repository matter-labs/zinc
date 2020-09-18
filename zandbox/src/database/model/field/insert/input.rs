//!
//! The database contract storage field INSERT input model.
//!

use serde_json::Value as JsonValue;

///
/// The database contract storage field INSERT input model.
///
#[derive(Debug)]
pub struct Input {
    /// The field index in the contract storage.
    pub index: i16,
    /// The contract account ID referencing `contracts.account_id`.
    pub account_id: i64,
    /// The field name.
    pub name: String,
    /// The field value in JSON representation.
    pub value: JsonValue,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(account_id: i64, index: i16, name: String, value: JsonValue) -> Self {
        Self {
            account_id,
            index,
            name,
            value,
        }
    }
}
