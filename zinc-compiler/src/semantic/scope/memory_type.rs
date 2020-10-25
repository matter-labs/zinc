//!
//! The semantic analyzer scope variable item memory item.
//!

///
/// The virtual machine memory type descriptor.
///
#[derive(Debug, Clone, Copy)]
pub enum MemoryType {
    /// Data allocated on the VM data stack.
    Stack,
    /// The contract instance represented in its entries by the `self` argument.
    ContractInstance,
    /// The contract data field allocated in the contract storage.
    ContractStorage {
        /// The field index in the contract storage.
        index: usize,
    },
}
