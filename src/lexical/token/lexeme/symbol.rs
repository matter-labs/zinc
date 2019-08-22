//!
//! The symbol lexeme.
//!

use std::fmt;

use serde_derive::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Symbol {
    // simple
    ParenthesisLeft,
    ParenthesisRight,
    BracketSquareLeft,
    BracketSquareRight,
    BracketCurlyLeft,
    BracketCurlyRight,
    Dot,
    Colon,
    Semicolon,
    Comma,
    Equals,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Backslash,
    ExclamationMark,
    LesserThan,
    GreaterThan,

    // complex
    DoubleEquals,
    ExclamationMarkEquals,
    LesserThanEquals,
    GreaterThanEquals,
    DoubleAmpersand,
    DoubleVerticalBar,
    DoubleCircumflex,
}

impl From<&[u8]> for Symbol {
    fn from(bytes: &[u8]) -> Self {
        match bytes {
            b"(" => Symbol::ParenthesisLeft,
            b")" => Symbol::ParenthesisRight,
            b"[" => Symbol::BracketSquareLeft,
            b"]" => Symbol::BracketSquareRight,
            b"{" => Symbol::BracketCurlyLeft,
            b"}" => Symbol::BracketCurlyRight,
            b"." => Symbol::Dot,
            b":" => Symbol::Colon,
            b";" => Symbol::Semicolon,
            b"," => Symbol::Comma,
            b"=" => Symbol::Equals,
            b"+" => Symbol::Plus,
            b"-" => Symbol::Minus,
            b"*" => Symbol::Asterisk,
            b"/" => Symbol::Slash,
            b"%" => Symbol::Percent,
            b"\\" => Symbol::Backslash,
            b"!" => Symbol::ExclamationMark,
            b"<" => Symbol::LesserThan,
            b">" => Symbol::GreaterThan,

            b"==" => Symbol::DoubleEquals,
            b"!=" => Symbol::ExclamationMarkEquals,
            b"<=" => Symbol::LesserThanEquals,
            b">=" => Symbol::GreaterThanEquals,
            b"&&" => Symbol::DoubleAmpersand,
            b"||" => Symbol::DoubleVerticalBar,
            b"^^" => Symbol::DoubleCircumflex,

            _unknown => unreachable!(),
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::ParenthesisLeft => write!(f, "("),
            Symbol::ParenthesisRight => write!(f, ")"),
            Symbol::BracketSquareLeft => write!(f, "["),
            Symbol::BracketSquareRight => write!(f, "]"),
            Symbol::BracketCurlyLeft => write!(f, "{{"),
            Symbol::BracketCurlyRight => write!(f, "}}"),
            Symbol::Dot => write!(f, "."),
            Symbol::Colon => write!(f, ":"),
            Symbol::Semicolon => write!(f, ";"),
            Symbol::Comma => write!(f, ","),
            Symbol::Equals => write!(f, "="),
            Symbol::Plus => write!(f, "+"),
            Symbol::Minus => write!(f, "-"),
            Symbol::Asterisk => write!(f, "*"),
            Symbol::Slash => write!(f, "/"),
            Symbol::Percent => write!(f, "%"),
            Symbol::Backslash => write!(f, "\\"),
            Symbol::ExclamationMark => write!(f, "!"),
            Symbol::LesserThan => write!(f, "<"),
            Symbol::GreaterThan => write!(f, ">"),

            Symbol::DoubleEquals => write!(f, "=="),
            Symbol::ExclamationMarkEquals => write!(f, "!="),
            Symbol::LesserThanEquals => write!(f, "<="),
            Symbol::GreaterThanEquals => write!(f, ">="),
            Symbol::DoubleAmpersand => write!(f, "&&"),
            Symbol::DoubleVerticalBar => write!(f, "||"),
            Symbol::DoubleCircumflex => write!(f, "^^"),
        }
    }
}
