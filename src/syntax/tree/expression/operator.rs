//!
//! The expression operator.
//!

use serde_derive::Serialize;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Operator {
    LogicalOr,
    LogicalXor,
    LogicalAnd,
    LogicalNot,

    ComparisonEqual,
    ComparisonNotEqual,
    ComparisonGreaterEqual,
    ComparisonLesserEqual,
    ComparisonGreater,
    ComparisonLesser,

    Addition,
    Subtraction,
    Negation,
    Multiplication,
    Division,
    Remainder,
}
