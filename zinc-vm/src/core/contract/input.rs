//!
//! The virtual machine contract input.
//!

use zinc_types::TransactionMsg;

///
/// The virtual machine contract input.
///
pub struct Input {
    /// The contract method arguments, which is witness for now.
    pub arguments: zinc_types::Value,
    /// The contract storage after executing a method.
    pub storage: zinc_types::Value,
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
        arguments: zinc_types::Value,
        storage: zinc_types::Value,
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
