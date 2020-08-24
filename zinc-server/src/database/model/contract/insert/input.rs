//!
//! The database contract instance INSERT input model.
//!

///
/// The database contract instance INSERT input model.
///
pub struct Input {
    /// The contract instance account ID.
    pub account_id: i64,
    /// The template ID referencing `templates.id`.
    pub template_id: i64,
    /// The contract instance owner ETH address.
    pub eth_address: Vec<u8>,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(account_id: i64, template_id: i64, eth_address: Vec<u8>) -> Self {
        Self {
            account_id,
            template_id,
            eth_address,
        }
    }
}
