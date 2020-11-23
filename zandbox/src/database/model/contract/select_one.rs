//!
//! The database contract SELECT one model.
//!

///
/// The database contract SELECT one input model.
///
#[derive(Debug)]
pub struct Input {
    /// The contract ETH address.
    pub eth_address: zksync_types::Address,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(eth_address: zksync_types::Address) -> Self {
        Self { eth_address }
    }
}

///
/// The database contract SELECT one output model.
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

    /// The contract ETH address.
    pub eth_address: Vec<u8>,
    /// The contract private key.
    pub eth_private_key: Vec<u8>,
}
