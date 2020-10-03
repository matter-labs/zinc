//!
//! The semantic analyzer element index access.
//!

///
/// Array access data.
///
#[derive(Debug, Clone)]
pub struct Index {
    /// The array element size.
    pub element_size: usize,
    /// The length of the result slice.
    pub slice_length: usize,
    /// The array total size.
    pub total_size: usize,
    /// The offset if the index is known at compile-time.
    pub offset: Option<usize>,
}

impl Index {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        element_size: usize,
        slice_length: usize,
        total_size: usize,
        offset: Option<usize>,
    ) -> Self {
        Self {
            element_size,
            slice_length,
            total_size,
            offset,
        }
    }
}
