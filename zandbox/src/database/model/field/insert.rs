//!
//! The database contract storage field INSERT model.
//!

///
/// The database contract storage field INSERT input model.
///
#[derive(Debug)]
pub struct Input {
    /// The contract account ID referencing `contracts.account_id`.
    pub account_id: zksync_types::AccountId,
    /// The field index in the contract storage.
    pub index: i16,
    /// The field name.
    pub name: String,
    /// The field value in JSON representation.
    pub value: serde_json::Value,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        account_id: zksync_types::AccountId,
        index: i16,
        name: String,
        value: serde_json::Value,
    ) -> Self {
        Self {
            account_id,
            index,
            name,
            value,
        }
    }
}
