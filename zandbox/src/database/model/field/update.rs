//!
//! The database contract storage field UPDATE model.
//!

use zksync_types::AccountId;

///
/// The database contract storage field UPDATE input model.
///
#[derive(Debug)]
pub struct Input {
    /// The contract account ID referencing `contracts.account_id`.
    pub account_id: AccountId,
    /// The field index in the contract storage.
    pub index: i16,
    /// The field value in JSON representation.
    pub value: serde_json::Value,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(account_id: AccountId, index: i16, value: serde_json::Value) -> Self {
        Self {
            account_id,
            index,
            value,
        }
    }
}
