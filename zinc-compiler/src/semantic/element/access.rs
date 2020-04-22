//!
//! The semantic analyzer element access.
//!

use crate::semantic::element::Element;

///
/// Array access data.
///
#[derive(Debug, Clone)]
pub struct Index {
    /// The array element size
    pub element_size: usize,
    /// The array total size
    pub total_size: usize,
    /// The offset if the index is known at compile-time
    pub offset: Option<usize>,
}

impl Index {
    pub fn new(element_size: usize, total_size: usize, offset: Option<usize>) -> Self {
        Self {
            element_size,
            total_size,
            offset,
        }
    }
}

///
/// Tuple or structure field, or namespace method access data.
///
pub enum FieldVariant {
    /// Field access via the dot `.` operator
    Field(Field),
    /// Method call via the dot `.` operator
    Method(Element),
}

#[derive(Debug, Clone)]
pub struct Field {
    /// The position of the element in tuple
    pub position: usize,
    /// The offset of the element in tuple
    pub offset: usize,
    /// The size of the tuple element
    pub element_size: usize,
    /// The total size of the tuple
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
