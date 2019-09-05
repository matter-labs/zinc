//!
//! The expression operator.
//!

use std::fmt;

use serde_derive::Serialize;

#[derive(Debug, Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Operator {
    // assignment
    Assignment,

    // range
    Range,

    // binary logical
    Or,
    Xor,
    And,

    // comparison
    Equal,
    NotEqual,
    GreaterEqual,
    LesserEqual,
    Greater,
    Lesser,

    // binary arithmetic
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Remainder,

    // type casting
    Casting,

    // unary logical
    Not,

    // unary arithmetic
    Negation,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Assignment => write!(f, "="),

            Self::Range => write!(f, ".."),

            Self::Or => write!(f, "||"),
            Self::Xor => write!(f, "^^"),
            Self::And => write!(f, "&&"),

            Self::Equal => write!(f, "=="),
            Self::NotEqual => write!(f, "!="),
            Self::GreaterEqual => write!(f, ">="),
            Self::LesserEqual => write!(f, "<="),
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
        }
    }
}
