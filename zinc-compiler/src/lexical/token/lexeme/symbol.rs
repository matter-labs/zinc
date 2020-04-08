//!
//! The lexical token symbol lexeme.
//!

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    // one char
    ParenthesisLeft,
    ParenthesisRight,
    BracketSquareLeft,
    BracketSquareRight,
    BracketCurlyLeft,
    BracketCurlyRight,
    Underscore,
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
    VerticalBar,
    Ampersand,
    Circumflex,
    Tilde,
    ExclamationMark,
    Lesser,
    Greater,

    // two chars
    PlusEquals,
    MinusEquals,
    AsteriskEquals,
    SlashEquals,
    PercentEquals,
    VerticalBarEquals,
    AmpersandEquals,
    CircumflexEquals,
    DoubleColon,
    DoubleEquals,
    ExclamationMarkEquals,
    LesserEquals,
    GreaterEquals,
    DoubleAmpersand,
    DoubleVerticalBar,
    DoubleCircumflex,
    DoubleLesser,
    DoubleGreater,
    DoubleDot,
    EqualsGreater,
    MinusGreater,

    // three chars
    DoubleDotEquals,
    DoubleLesserEquals,
    DoubleGreaterEquals,
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
            Self::Underscore => write!(f, "_"),
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
            Self::VerticalBar => write!(f, "|"),
            Self::Ampersand => write!(f, "&"),
            Self::Circumflex => write!(f, "^"),
            Self::Tilde => write!(f, "~"),
            Self::ExclamationMark => write!(f, "!"),
            Self::Lesser => write!(f, "<"),
            Self::Greater => write!(f, ">"),

            Self::PlusEquals => write!(f, "+="),
            Self::MinusEquals => write!(f, "-="),
            Self::AsteriskEquals => write!(f, "*="),
            Self::SlashEquals => write!(f, "/="),
            Self::PercentEquals => write!(f, "%="),
            Self::VerticalBarEquals => write!(f, "|="),
            Self::AmpersandEquals => write!(f, "&="),
            Self::CircumflexEquals => write!(f, "^="),
            Self::DoubleColon => write!(f, "::"),
            Self::DoubleEquals => write!(f, "=="),
            Self::ExclamationMarkEquals => write!(f, "!="),
            Self::LesserEquals => write!(f, "<="),
            Self::GreaterEquals => write!(f, ">="),
            Self::DoubleAmpersand => write!(f, "&&"),
            Self::DoubleVerticalBar => write!(f, "||"),
            Self::DoubleCircumflex => write!(f, "^^"),
            Self::DoubleLesser => write!(f, "<<"),
            Self::DoubleGreater => write!(f, ">>"),
            Self::DoubleDot => write!(f, ".."),
            Self::EqualsGreater => write!(f, "=>"),
            Self::MinusGreater => write!(f, "->"),

            Self::DoubleDotEquals => write!(f, "..="),
            Self::DoubleLesserEquals => write!(f, "<<="),
            Self::DoubleGreaterEquals => write!(f, ">>="),
        }
    }
}
