//!
//! The semantic analyzer place memory type.
//!

///
/// The virtual machine memory type descriptor.
///
#[derive(Debug, Clone, Copy)]
pub enum MemoryType {
    /// Data allocated on the VM data stack.
    Stack,
    /// The contract storage field.
    ContractStorage {
        /// The field index in the contract storage.
        index: usize,
    },
}
