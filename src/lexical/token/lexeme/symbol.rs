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
            b"(" => Self::ParenthesisLeft,
            b")" => Self::ParenthesisRight,
            b"[" => Self::BracketSquareLeft,
            b"]" => Self::BracketSquareRight,
            b"{" => Self::BracketCurlyLeft,
            b"}" => Self::BracketCurlyRight,
            b"." => Self::Dot,
            b":" => Self::Colon,
            b";" => Self::Semicolon,
            b"," => Self::Comma,
            b"=" => Self::Equals,
            b"+" => Self::Plus,
            b"-" => Self::Minus,
            b"*" => Self::Asterisk,
            b"/" => Self::Slash,
            b"%" => Self::Percent,
            b"\\" => Self::Backslash,
            b"!" => Self::ExclamationMark,
            b"<" => Self::LesserThan,
            b">" => Self::GreaterThan,

            b"==" => Self::DoubleEquals,
            b"!=" => Self::ExclamationMarkEquals,
            b"<=" => Self::LesserThanEquals,
            b">=" => Self::GreaterThanEquals,
            b"&&" => Self::DoubleAmpersand,
            b"||" => Self::DoubleVerticalBar,
            b"^^" => Self::DoubleCircumflex,

            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ParenthesisLeft => write!(f, "("),
            Self::ParenthesisRight => write!(f, ")"),
            Self::BracketSquareLeft => write!(f, "["),
            Self::BracketSquareRight => write!(f, "]"),
            Self::BracketCurlyLeft => write!(f, "{{"),
            Self::BracketCurlyRight => write!(f, "}}"),
            Self::Dot => write!(f, "."),
            Self::Colon => write!(f, ":"),
            Self::Semicolon => write!(f, ";"),
            Self::Comma => write!(f, ","),
            Self::Equals => write!(f, "="),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Asterisk => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Percent => write!(f, "%"),
            Self::Backslash => write!(f, "\\"),
            Self::ExclamationMark => write!(f, "!"),
            Self::LesserThan => write!(f, "<"),
            Self::GreaterThan => write!(f, ">"),

            Self::DoubleEquals => write!(f, "=="),
            Self::ExclamationMarkEquals => write!(f, "!="),
            Self::LesserThanEquals => write!(f, "<="),
            Self::GreaterThanEquals => write!(f, ">="),
            Self::DoubleAmpersand => write!(f, "&&"),
            Self::DoubleVerticalBar => write!(f, "||"),
            Self::DoubleCircumflex => write!(f, "^^"),
        }
    }
}
