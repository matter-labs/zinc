//!
//! The semantic analyzer element dot stack data field access.
//!

#[derive(Debug, Clone)]
pub struct StackField {
    /// The name of the tuple or structure element
    pub name: String,
    /// The position of the element in the tuple or structure
    pub position: usize,
    /// The offset of the element in the tuple or structure
    pub offset: usize,
    /// The size of the tuple or structure element
    pub element_size: usize,
    /// The total size of the tuple or structure
    pub total_size: usize,
}

impl StackField {
    pub fn new(
        name: String,
        position: usize,
        offset: usize,
        element_size: usize,
        total_size: usize,
    ) -> Self {
        Self {
            name,
            position,
            offset,
            element_size,
            total_size,
        }
    }
}
