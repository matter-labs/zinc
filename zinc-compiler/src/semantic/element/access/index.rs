//!
//! The semantic analyzer element index access.
//!

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
