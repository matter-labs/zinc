//!
//! The lexical integer literal parser output.
//!

use crate::token::lexeme::literal::integer::Integer;

///
/// The lexical integer literal parser output.
///
#[derive(Debug, PartialEq)]
pub struct Output {
    /// The number of characters in the integer.
    pub size: usize,
    /// The integer data.
    pub integer: Integer,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(size: usize, integer: Integer) -> Self {
        Self { size, integer }
    }
}
