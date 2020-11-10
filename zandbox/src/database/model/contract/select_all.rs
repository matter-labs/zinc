//!
//! The database contract SELECT all model.
//!

///
/// The database contract SELECT all output model.
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

    /// The contract source code.
    pub source_code: serde_json::Value,
    /// The contract bytecode.
    pub bytecode: Vec<u8>,
    /// The contract verifying key.
    pub verifying_key: Vec<u8>,

    /// The contract ETH address.
    pub eth_address: Vec<u8>,
    /// The contract private key.
    pub eth_private_key: Vec<u8>,
}
