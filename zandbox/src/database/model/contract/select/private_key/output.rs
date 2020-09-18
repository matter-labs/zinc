//!
//! The database contract private key SELECT output model.
//!

///
/// The database contract private key SELECT output model.
///
#[derive(Debug, sqlx::FromRow)]
pub struct Output {
    /// The contract private key.
    pub eth_private_key: Vec<u8>,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(eth_private_key: Vec<u8>) -> Self {
        Self { eth_private_key }
    }
}
