//!
//! The database contract SELECT Curve model.
//!

///
/// The database contract SELECT Curve output model.
///
#[derive(Debug, sqlx::FromRow)]
pub struct Output {
    /// The contract ETH address.
    pub address: Vec<u8>,

    /// The contract project name.
    pub name: String,
    /// The contract version.
    pub version: String,
    /// The contract instance name.
    pub instance: String,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(address: Vec<u8>, name: String, version: String, instance: String) -> Self {
        Self {
            address,
            name,
            version,
            instance,
        }
    }
}
