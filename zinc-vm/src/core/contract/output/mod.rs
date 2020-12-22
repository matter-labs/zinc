//!
//! The virtual machine contract output.
//!

pub mod initializer;

use std::collections::HashMap;

use num::BigInt;

use self::initializer::Initializer;

///
/// The virtual machine contract output.
///
#[derive(Debug)]
pub struct Output {
    /// The contract method output result, which is the public data for now.
    pub result: zinc_types::Value,
    /// The contract storage after executing a method.
    pub storages: HashMap<BigInt, zinc_types::Value>,
    /// The transfers executed during the method execution.
    pub transfers: Vec<zinc_types::TransactionMsg>,
    /// The contract initializers created during the method execution.
    pub initializers: Vec<Initializer>,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        result: zinc_types::Value,
        storages: HashMap<BigInt, zinc_types::Value>,
        transfers: Vec<zinc_types::TransactionMsg>,
        initializers: Vec<Initializer>,
    ) -> Self {
        Self {
            result,
            storages,
            transfers,
            initializers,
        }
    }
}
