//!
//! The lexical string literal parser output.
//!

///
/// The lexical string literal parser output.
///
#[derive(Debug, PartialEq)]
pub struct Output {
    /// The number of characters in the string.
    pub size: usize,
    /// The string data.
    pub string: String,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(size: usize, string: String) -> Self {
        Self { size, string }
    }
}
