//!
//! The expression operator.
//!

use std::fmt;

use serde_derive::Serialize;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Operator {
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
            Operator::Or => write!(f, "||"),
            Operator::Xor => write!(f, "^^"),
            Operator::And => write!(f, "&&"),

            Operator::Equal => write!(f, "=="),
            Operator::NotEqual => write!(f, "!="),
            Operator::GreaterEqual => write!(f, ">="),
            Operator::LesserEqual => write!(f, "<="),
            Operator::Greater => write!(f, ">"),
            Operator::Lesser => write!(f, "<"),

            Operator::Addition => write!(f, "+"),
            Operator::Subtraction => write!(f, "-"),
            Operator::Multiplication => write!(f, "*"),
            Operator::Division => write!(f, "/"),
            Operator::Remainder => write!(f, "%"),

            Operator::Casting => write!(f, "as"),

            Operator::Not => write!(f, "!"),

            Operator::Negation => write!(f, "-"),
        }
    }
}
