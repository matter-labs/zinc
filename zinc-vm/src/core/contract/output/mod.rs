//!
//! The virtual machine contract output.
//!

pub mod transfer;

use self::transfer::Transfer;

///
/// The virtual machine contract output.
///
#[derive(Debug)]
pub struct Output {
    /// The contract method output result, which is the public data for now.
    pub result: zinc_build::Value,
    /// The contract storage after executing a method.
    pub storage: zinc_build::Value,
    /// The transfers executed in the contract method.
    pub transfers: Vec<Transfer>,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        result: zinc_build::Value,
        storage: zinc_build::Value,
        transfers: Vec<Transfer>,
    ) -> Self {
        Self {
            result,
            storage,
            transfers,
        }
    }
}
