//!
//! The inferred integer type.
//!

///
/// The inferred integer type.
///
#[derive(Debug, PartialEq)]
pub struct Type {
    /// Whether the inferred type is signed.
    pub is_signed: bool,
    /// The inferred type bitlength.
    pub bitlength: usize,
}

impl Type {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(is_signed: bool, bitlength: usize) -> Self {
        Self {
            is_signed,
            bitlength,
        }
    }
}
