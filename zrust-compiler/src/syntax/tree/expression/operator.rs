//!
//! The expression operator.
//!

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    // assignment
    Assignment,

    // range
    Range,
    RangeInclusive,

    // binary logical
    Or,
    Xor,
    And,

    // comparison
    Equals,
    NotEquals,
    GreaterEquals,
    LesserEquals,
    Greater,
    Lesser,

    // binary arithmetic
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Remainder,

    // type semantic.casting
    Casting,

    // unary logical
    Not,

    // unary arithmetic
    Negation,

    // access
    Indexing,
    Field,
    Call,
    Path,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Assignment => write!(f, "="),

            Self::Range => write!(f, ".."),
            Self::RangeInclusive => write!(f, "..="),

            Self::Or => write!(f, "||"),
            Self::Xor => write!(f, "^^"),
            Self::And => write!(f, "&&"),

            Self::Equals => write!(f, "=="),
            Self::NotEquals => write!(f, "!="),
            Self::GreaterEquals => write!(f, ">="),
            Self::LesserEquals => write!(f, "<="),
            Self::Greater => write!(f, ">"),
            Self::Lesser => write!(f, "<"),

            Self::Addition => write!(f, "+"),
            Self::Subtraction => write!(f, "-"),
            Self::Multiplication => write!(f, "*"),
            Self::Division => write!(f, "/"),
            Self::Remainder => write!(f, "%"),

            Self::Casting => write!(f, "as"),

            Self::Not => write!(f, "!"),

            Self::Negation => write!(f, "-"),

            Self::Indexing => write!(f, "[]"),
            Self::Field => write!(f, "."),
            Self::Call => write!(f, "()"),
            Self::Path => write!(f, "::"),
        }
    }
}
