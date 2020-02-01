//!
//! The semantic analyzer element access.
//!

use crate::semantic::Value;

pub struct IndexResult {
    pub offset: usize,
    pub element_size: usize,
    pub total_size: usize,
    pub sliced_value: Option<Value>,
}

impl IndexResult {
    pub fn new(
        offset: usize,
        element_size: usize,
        total_size: usize,
        sliced_value: Option<Value>,
    ) -> Self {
        Self {
            offset,
            element_size,
            total_size,
            sliced_value,
        }
    }
}

pub struct FieldResult {
    pub offset: usize,
    pub element_size: usize,
    pub total_size: usize,
    pub sliced_value: Option<Value>,
}

impl FieldResult {
    pub fn new(
        offset: usize,
        element_size: usize,
        total_size: usize,
        sliced_value: Option<Value>,
    ) -> Self {
        Self {
            offset,
            element_size,
            total_size,
            sliced_value,
        }
    }
}
