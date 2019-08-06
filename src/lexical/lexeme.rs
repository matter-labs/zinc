//!
//! The lexeme.
//!

use crate::lexical::Delimiter;
use crate::lexical::Identifier;
use crate::lexical::Keyword;
use crate::lexical::Literal;
use crate::lexical::Punctuation;

#[derive(Debug)]
pub enum Lexeme {
    Keyword(Keyword),
    Identifier(Identifier),
    Delimiter(Delimiter),
    Punctuation(Punctuation),
    Literal(Literal),
}
