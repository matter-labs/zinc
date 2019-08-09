//!
//! The operator lexeme.
//!

use serde_derive::Serialize;

use crate::lexical::Delimiter;

#[derive(Debug, Serialize)]
pub enum Operator {
    ParenthesisOpen,
    ParenthesisClose,

    Assignment,

    Dot,

    ArithmeticAddition,
    ArithmeticSubtractionOrArithmeticNegation,
    ArithmeticMultiplication,
    ArithmeticDivision,
    ArithmeticRemainder,
    ArithmeticInversion,

    BooleanAnd,
    BooleanOr,
    BooleanXor,
    BooleanNot,

    ComparisonEqual,
    ComparisonNotEqual,
    ComparisonLesserEqual,
    ComparisonGreaterEqual,
    ComparisonLesser,
    ComparisonGreater,
}

impl Operator {
    pub fn to_delimiter(&self) -> Option<Delimiter> {
        Some(match self {
            Operator::ParenthesisOpen => Delimiter::BracketRoundOpen,
            Operator::ParenthesisClose => Delimiter::BracketRoundClose,
            Operator::ComparisonLesser => Delimiter::BracketAngleOpen,
            Operator::ComparisonGreater => Delimiter::BracketAngleClose,
            _ => return None,
        })
    }
}

impl From<&[u8]> for Operator {
    fn from(bytes: &[u8]) -> Self {
        match bytes {
            b"(" => Operator::ParenthesisOpen,
            b")" => Operator::ParenthesisClose,

            b"=" => Operator::Assignment,

            b"." => Operator::Dot,

            b"+" => Operator::ArithmeticAddition,
            b"-" => Operator::ArithmeticSubtractionOrArithmeticNegation,
            b"*" => Operator::ArithmeticMultiplication,
            b"/" => Operator::ArithmeticDivision,
            b"%" => Operator::ArithmeticRemainder,
            b"\\" => Operator::ArithmeticInversion,

            b"&&" => Operator::BooleanAnd,
            b"||" => Operator::BooleanOr,
            b"^^" => Operator::BooleanXor,
            b"!" => Operator::BooleanNot,

            b"==" => Operator::ComparisonEqual,
            b"!=" => Operator::ComparisonNotEqual,
            b"<=" => Operator::ComparisonLesserEqual,
            b">=" => Operator::ComparisonGreaterEqual,
            b"<" => Operator::ComparisonLesser,
            b">" => Operator::ComparisonGreater,

            _unknown => panic!("State machine bug"),
        }
    }
}
