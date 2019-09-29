//!
//! The symbol lexeme.
//!

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
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
    DoubleDot,
    DoubleDotEquals,
}

impl From<&str> for Symbol {
    fn from(input: &str) -> Self {
        match input {
            "(" => Self::ParenthesisLeft,
            ")" => Self::ParenthesisRight,
            "[" => Self::BracketSquareLeft,
            "]" => Self::BracketSquareRight,
            "{" => Self::BracketCurlyLeft,
            "}" => Self::BracketCurlyRight,
            "." => Self::Dot,
            ":" => Self::Colon,
            ";" => Self::Semicolon,
            "," => Self::Comma,
            "=" => Self::Equals,
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Asterisk,
            "/" => Self::Slash,
            "%" => Self::Percent,
            "\\" => Self::Backslash,
            "!" => Self::ExclamationMark,
            "<" => Self::LesserThan,
            ">" => Self::GreaterThan,

            "==" => Self::DoubleEquals,
            "!=" => Self::ExclamationMarkEquals,
            "<=" => Self::LesserThanEquals,
            ">=" => Self::GreaterThanEquals,
            "&&" => Self::DoubleAmpersand,
            "||" => Self::DoubleVerticalBar,
            "^^" => Self::DoubleCircumflex,
            ".." => Self::DoubleDot,
            "..=" => Self::DoubleDotEquals,

            _ => panic!("Always processed by branches above and never gets here"),
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
            Self::DoubleDot => write!(f, ".."),
            Self::DoubleDotEquals => write!(f, "..="),
        }
    }
}
