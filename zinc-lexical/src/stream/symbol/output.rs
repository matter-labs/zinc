//!
//! The lexical symbol parser output.
//!

use crate::token::lexeme::symbol::Symbol;

///
/// The lexical symbol parser output.
///
#[derive(Debug, PartialEq)]
pub struct Output {
    /// The number of characters in the symbol.
    pub size: usize,
    /// The symbol data.
    pub symbol: Symbol,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(size: usize, symbol: Symbol) -> Self {
        Self { size, symbol }
    }
}
