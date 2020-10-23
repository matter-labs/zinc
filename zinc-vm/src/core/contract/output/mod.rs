//!
//! The virtual machine contract output.
//!

pub mod transfer;

use zinc_build::Value as BuildValue;

use self::transfer::Transfer;

///
/// The virtual machine contract output.
///
pub struct Output {
    /// The contract method output result, which is public data for now.
    pub result: BuildValue,
    /// The contract storage after executing a method.
    pub storage: BuildValue,
    /// The transfers executed in the contract method.
    pub transfers: Vec<Transfer>,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(result: BuildValue, storage: BuildValue, transfers: Vec<Transfer>) -> Self {
        Self {
            result,
            storage,
            transfers,
        }
    }
}
