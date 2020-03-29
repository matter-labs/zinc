//!
//! The semantic analyzer element access.
//!

use crate::semantic::element::r#type::Type;

#[derive(Debug, Clone)]
pub struct AccessData {
    pub offset: usize,
    pub element_size: usize,
    pub total_size: usize,
    pub sliced_type: Type,
}

impl AccessData {
    pub fn new(offset: usize, element_size: usize, total_size: usize, sliced_type: Type) -> Self {
        Self {
            offset,
            element_size,
            total_size,
            sliced_type,
        }
    }
}
