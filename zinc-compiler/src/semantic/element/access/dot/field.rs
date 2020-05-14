//!
//! The semantic analyzer element dot field access.
//!

#[derive(Debug, Clone)]
pub struct Field {
    /// The position of the element in the tuple or structure
    pub position: usize,
    /// The offset of the element in the tuple or structure
    pub offset: usize,
    /// The size of the tuple or structure element
    pub element_size: usize,
    /// The total size of the tuple or structure
    pub total_size: usize,
}

impl Field {
    pub fn new(position: usize, offset: usize, element_size: usize, total_size: usize) -> Self {
        Self {
            position,
            offset,
            element_size,
            total_size,
        }
    }
}
