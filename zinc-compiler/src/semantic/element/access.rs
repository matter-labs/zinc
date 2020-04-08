//!
//! The semantic analyzer element access.
//!

#[derive(Debug, Clone)]
pub struct Index {
    pub element_size: usize,
    pub total_size: usize,
}

impl Index {
    pub fn new(element_size: usize, total_size: usize) -> Self {
        Self {
            element_size,
            total_size,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub position: usize,
    pub offset: usize,
    pub element_size: usize,
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
