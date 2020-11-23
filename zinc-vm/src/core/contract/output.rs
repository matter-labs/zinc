//!
//! The virtual machine contract output.
//!

use std::collections::HashMap;

use num::BigInt;

///
/// The virtual machine contract output.
///
#[derive(Debug)]
pub struct Output {
    /// The contract method output result, which is the public data for now.
    pub result: zinc_build::Value,
    /// The contract storage after executing a method.
    pub storages: HashMap<BigInt, zinc_build::Value>,
    /// The transfers executed in the contract method.
    pub transfers: Vec<zinc_zksync::TransactionMsg>,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        result: zinc_build::Value,
        storages: HashMap<BigInt, zinc_build::Value>,
        transfers: Vec<zinc_zksync::TransactionMsg>,
    ) -> Self {
        Self {
            result,
            storages,
            transfers,
        }
    }
}
