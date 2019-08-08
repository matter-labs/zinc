//!
//! The token lexeme.
//!

use std::fmt;

use serde_derive::Serialize;

use crate::lexical::Comment;
use crate::lexical::Delimiter;
use crate::lexical::Identifier;
use crate::lexical::Keyword;
use crate::lexical::Literal;
use crate::lexical::Operator;
use crate::lexical::Punctuation;

#[derive(Debug, Serialize)]
pub enum Lexeme {
    Keyword(Keyword),
    Identifier(Identifier),
    Delimiter(Delimiter),
    Punctuation(Punctuation),
    Literal(Literal),
    Operator(Operator),
    Comment(Comment),
}

impl fmt::Display for Lexeme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
