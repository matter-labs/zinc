//!
//! The virtual machine contract input.
//!

use zinc_build::Value as BuildValue;
use zinc_zksync::TransactionMsg;

///
/// The virtual machine contract input.
///
pub struct Input {
    /// The contract method arguments, which is witness for now.
    pub arguments: BuildValue,
    /// The contract storage after executing a method.
    pub storage: BuildValue,
    /// The contract method name which is called.
    pub method_name: String,
    /// The contract input transaction.
    pub transaction: TransactionMsg,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        arguments: BuildValue,
        storage: BuildValue,
        method_name: String,
        transaction: TransactionMsg,
    ) -> Self {
        Self {
            arguments,
            storage,
            method_name,
            transaction,
        }
    }
}
