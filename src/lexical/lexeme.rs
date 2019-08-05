//!
//! The lexeme.
//!

use crate::lexical::Identifier;
use crate::lexical::Keyword;

pub enum Lexeme {
    Keyword(Keyword),
    Identifier(Identifier),
}
