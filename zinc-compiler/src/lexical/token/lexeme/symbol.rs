//!
//! The lexical token symbol lexeme.
//!

use std::fmt;

///
/// The minimal logical character group, which is usually a delimiter, operator, or special symbol.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    /// The ( character
    ParenthesisLeft,
    /// The ) character
    ParenthesisRight,
    /// The [ character
    BracketSquareLeft,
    /// The ] character
    BracketSquareRight,
    /// The { character
    BracketCurlyLeft,
    /// The } character
    BracketCurlyRight,
    /// The _ character
    Underscore,
    /// The . character
    Dot,
    /// The : character
    Colon,
    /// The ; character
    Semicolon,
    /// The , character
    Comma,
    /// The = character
    Equals,
    /// The + character
    Plus,
    /// The - character
    Minus,
    /// The * character
    Asterisk,
    /// The / character
    Slash,
    /// The % character
    Percent,
    /// The | character
    VerticalBar,
    /// The & character
    Ampersand,
    /// The ^ character
    Circumflex,
    /// The ` character
    Tilde,
    /// The ! character
    ExclamationMark,
    /// The < character
    Lesser,
    /// The > character
    Greater,
    /// The # character
    Number,

    /// The += character group
    PlusEquals,
    /// The -= character group
    MinusEquals,
    /// The *= character group
    AsteriskEquals,
    /// The /= character group
    SlashEquals,
    /// The %= character group
    PercentEquals,
    /// The |= character group
    VerticalBarEquals,
    /// The &= character group
    AmpersandEquals,
    /// The ^= character group
    CircumflexEquals,
    /// The :: character group
    DoubleColon,
    /// The == character group
    DoubleEquals,
    /// The != character group
    ExclamationMarkEquals,
    /// The <= character group
    LesserEquals,
    /// The >= character group
    GreaterEquals,
    /// The && character group
    DoubleAmpersand,
    /// The || character group
    DoubleVerticalBar,
    /// The ^^ character group
    DoubleCircumflex,
    /// The << character group
    DoubleLesser,
    /// The >> character group
    DoubleGreater,
    /// The .. character group
    DoubleDot,
    /// The => character group
    EqualsGreater,
    /// The -> character group
    MinusGreater,

    /// The ..= character group
    DoubleDotEquals,
    /// The <<= character group
    DoubleLesserEquals,
    /// The >>= character group
    DoubleGreaterEquals,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
            Self::Number => write!(f, "#"),

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
