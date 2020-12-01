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
    pub result: zinc_types::Value,
    /// The contract storage after executing a method.
    pub storages: HashMap<BigInt, zinc_types::Value>,
    /// The transfers executed in the contract method.
    pub transfers: Vec<zinc_types::TransactionMsg>,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        result: zinc_types::Value,
        storages: HashMap<BigInt, zinc_types::Value>,
        transfers: Vec<zinc_types::TransactionMsg>,
    ) -> Self {
        Self {
            result,
            storages,
            transfers,
        }
    }
}
