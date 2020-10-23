//!
//! The lexical word parser output.
//!

use crate::token::lexeme::Lexeme;

///
/// The lexical word parser output.
///
#[derive(Debug, PartialEq)]
pub struct Output {
    /// The number of characters in the word.
    pub size: usize,
    /// The word lexeme data.
    pub word: Lexeme,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(size: usize, word: Lexeme) -> Self {
        Self { size, word }
    }
}
