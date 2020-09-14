//!
//! The semantic analyzer place memory type.
//!

use crate::semantic::scope::memory_type::MemoryType as VariableItemMemoryType;

///
/// The virtual machine memory type descriptor.
///
#[derive(Debug, Clone)]
pub enum MemoryType {
    /// Data allocated on the VM data stack.
    Stack,
    /// The contract data field allocated in the contract storage.
    ContractStorage,
}

impl From<VariableItemMemoryType> for MemoryType {
    fn from(value: VariableItemMemoryType) -> Self {
        match value {
            VariableItemMemoryType::Stack => Self::Stack,
            VariableItemMemoryType::ContractInstance => Self::ContractStorage,
            VariableItemMemoryType::ContractStorage { .. } => Self::ContractStorage,
        }
    }
}
