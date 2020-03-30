//!
//! The semantic analyzer element access.
//!

use crate::semantic::element::r#type::Type;

#[derive(Debug, Clone)]
pub struct AccessData {
    pub position: usize,
    pub offset: usize,
    pub element_size: usize,
    pub total_size: usize,
}

impl AccessData {
    pub fn new(
        position: usize,
        offset: usize,
        element_size: usize,
        total_size: usize,
        _sliced_type: Type,
    ) -> Self {
        Self {
            position,
            offset,
            element_size,
            total_size,
        }
    }
}
