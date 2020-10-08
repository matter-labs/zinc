//!
//! The database contract storage field INSERT model.
//!

use serde_json::Value as JsonValue;

use zksync_types::AccountId;

///
/// The database contract storage field INSERT input model.
///
#[derive(Debug)]
pub struct Input {
    /// The contract account ID referencing `contracts.account_id`.
    pub account_id: AccountId,
    /// The field index in the contract storage.
    pub index: i16,
    /// The field name.
    pub name: String,
    /// The field value in JSON representation.
    pub value: JsonValue,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(account_id: AccountId, index: i16, name: String, value: JsonValue) -> Self {
        Self {
            account_id,
            index,
            name,
            value,
        }
    }
}
