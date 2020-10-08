//!
//! The database contract storage field SELECT model.
//!

use serde_json::Value as JsonValue;

use zksync_types::AccountId;

///
/// The database contract storage field SELECT input model.
///
#[derive(Debug)]
pub struct Input {
    /// The contract account ID referencing `contracts.account_id`.
    pub account_id: AccountId,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(account_id: AccountId) -> Self {
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
    pub value: JsonValue,
}
