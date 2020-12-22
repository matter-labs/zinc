//!
//! The virtual machine contract initializer.
//!

///
/// The virtual machine contract initializer.
///
#[derive(Debug)]
pub struct Initializer {
    /// The contract project name.
    pub name: String,
    /// The contract project version.
    pub version: semver::Version,
    /// The contract instance private key.
    pub eth_private_key: zksync_types::H256,
    /// The contract instance address.
    pub eth_address: zksync_types::Address,
    /// The contract field types.
    pub field_types: Vec<zinc_types::ContractFieldType>,
}

impl Initializer {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        name: String,
        version: semver::Version,
        eth_private_key: zksync_types::H256,
        eth_address: zksync_types::Address,
        field_types: Vec<zinc_types::ContractFieldType>,
    ) -> Self {
        Self {
            name,
            version,
            eth_private_key,
            eth_address,
            field_types,
        }
    }
}
