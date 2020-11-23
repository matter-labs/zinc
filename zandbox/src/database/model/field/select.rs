//!
//! The database contract storage field SELECT model.
//!

///
/// The database contract storage field SELECT input model.
///
#[derive(Debug)]
pub struct Input {
    /// The contract account ID referencing `contracts.account_id`.
    pub account_id: zksync_types::AccountId,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(account_id: zksync_types::AccountId) -> Self {
        Self { account_id }
    }
}

///
/// The database contract storage field SELECT output model.
///
#[derive(Debug, sqlx::FromRow)]
pub struct Output {
    /// The field name.
    pub name: String,
    /// The field value in JSON representation.
    pub value: serde_json::Value,
}
