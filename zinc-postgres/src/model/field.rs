//!
//! The PostgreSQL contract storage field model.
//!

use serde_json::Value as JsonValue;

///
/// The PostgreSQL contract storage field model.
///
pub struct Field {
    /// The field index in the contract storage.
    pub index: u16,
    /// The contract account ID referencing `contracts.account_id`.
    pub account_id: u32,
    /// The field value in JSON representation.
    pub value: JsonValue,
}

impl Field {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(index: u16, account_id: u32, value: JsonValue) -> Self {
        Self {
            index,
            account_id,
            value,
        }
    }
}
