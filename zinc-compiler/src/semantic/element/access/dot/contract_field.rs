//!
//! The semantic analyzer element dot contract storage data field access.
//!

///
/// The contract field dot access data.
///
/// Contains all the necessary information to generate instructions for getting
/// data from the contract storage.
///
#[derive(Debug, Clone)]
pub struct ContractField {
    /// The name of the tuple or structure element.
    pub name: String,
    /// The position of the element in the contract storage.
    pub position: usize,
    /// The offset of the element in the tuple or structure.
    pub offset: usize,
    /// The size of the contract storage field.
    pub element_size: usize,
    /// The total size of the contract storage.
    pub total_size: usize,
    /// Whether the field is immutable.
    pub is_immutable: bool,
}

impl ContractField {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        name: String,
        position: usize,
        offset: usize,
        element_size: usize,
        total_size: usize,
        is_immutable: bool,
    ) -> Self {
        Self {
            name,
            offset,
            position,
            element_size,
            total_size,
            is_immutable,
        }
    }
}
