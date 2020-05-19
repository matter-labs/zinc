//!
//! The semantic analyzer element dot contract storage data field access.
//!

#[derive(Debug, Clone)]
pub struct ContractField {
    /// The position of the element in the contract storage
    pub position: usize,
    /// The size of the contract storage field
    pub element_size: usize,
}

impl ContractField {
    pub fn new(position: usize, element_size: usize) -> Self {
        Self {
            position,
            element_size,
        }
    }
}
