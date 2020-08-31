//!
//! The database contract storage field UPDATE input model.
//!

use serde_json::Value as JsonValue;

///
/// The database contract storage field UPDATE input model.
///
pub struct Input {
    /// The field index in the contract storage.
    pub index: i16,
    /// The contract account ID referencing `contracts.account_id`.
    pub contract_id: i64,
    /// The field value in JSON representation.
    pub value: JsonValue,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(index: i16, contract_id: i64, value: JsonValue) -> Self {
        Self {
            index,
            contract_id,
            value,
        }
    }
}
