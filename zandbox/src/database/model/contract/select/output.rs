//!
//! The database contract SELECT output model.
//!

///
/// The database contract SELECT output model.
///
#[derive(Debug, sqlx::FromRow)]
pub struct Output {
    /// The contract account ID.
    pub account_id: i64,
    /// The contract name.
    pub name: String,
    /// The contract version.
    pub version: String,
    /// The contract bytecode.
    pub bytecode: Vec<u8>,
    /// The contract ETH address.
    pub eth_address: Vec<u8>,
    /// The contract private key.
    pub eth_private_key: Vec<u8>,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        account_id: i64,
        name: String,
        version: String,
        bytecode: Vec<u8>,
        eth_address: Vec<u8>,
        eth_private_key: Vec<u8>,
    ) -> Self {
        Self {
            account_id,
            name,
            version,
            bytecode,
            eth_address,
            eth_private_key,
        }
    }
}
