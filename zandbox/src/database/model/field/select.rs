//!
//! The database contract storage field SELECT model.
//!

use serde_json::Value as JsonValue;

use zksync::web3::types::Address;

///
/// The database contract storage field SELECT input model.
///
#[derive(Debug)]
pub struct Input {
    /// The contract ETH address referencing `contracts.address`.
    pub address: Address,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(address: Address) -> Self {
        Self { address }
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
