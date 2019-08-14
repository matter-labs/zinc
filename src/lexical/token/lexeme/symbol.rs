//!
//! The symbol lexeme.
//!

use std::fmt;

use serde_derive::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Symbol {
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

    DoubleEquals,
    ExclamationEquals,
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
            b"!=" => Symbol::ExclamationEquals,
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
        write!(
            f,
            "{}",
            match self {
                Symbol::ParenthesisLeft => "(",
                Symbol::ParenthesisRight => ")",
                Symbol::BracketSquareLeft => "[",
                Symbol::BracketSquareRight => "]",
                Symbol::BracketCurlyLeft => "{",
                Symbol::BracketCurlyRight => "}",

                Symbol::Dot => ".",
                Symbol::Colon => ":",
                Symbol::Semicolon => ";",
                Symbol::Comma => ",",
                Symbol::Equals => "=",
                Symbol::Plus => "+",
                Symbol::Minus => "-",
                Symbol::Asterisk => "*",
                Symbol::Slash => "/",
                Symbol::Percent => "%",
                Symbol::Backslash => "\\",
                Symbol::ExclamationMark => "!",
                Symbol::LesserThan => "<",
                Symbol::GreaterThan => ">",

                Symbol::DoubleEquals => "==",
                Symbol::ExclamationEquals => "!=",
                Symbol::LesserThanEquals => "<=",
                Symbol::GreaterThanEquals => ">=",
                Symbol::DoubleAmpersand => "&&",
                Symbol::DoubleVerticalBar => "||",
                Symbol::DoubleCircumflex => "^^",
            }
        )
    }
}
