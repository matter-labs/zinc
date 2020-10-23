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
    /// Whether the field is an `std::collections::MTreeMap`, which is treated specially.
    /// For this type the `StorageLoad` instruction is not created, so the storage field index
    /// remains on the evaluation stack in order to be used by the `MTreeMap` methods as the
    /// instance argument, which is simply the map's position in the contract storage.
    pub is_mtreemap: bool,
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
        is_mtreemap: bool,
    ) -> Self {
        Self {
            name,
            offset,
            position,
            element_size,
            total_size,
            is_immutable,
            is_mtreemap,
        }
    }
}
