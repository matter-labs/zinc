//!
//! The operator lexeme.
//!

use std::convert::TryFrom;

use failure::Fail;

use crate::lexical::Delimiter;

#[derive(Debug)]
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

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "unknown")]
    Unknown,
}

impl TryFrom<&[u8]> for Operator {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        match bytes {
            b"(" => Ok(Operator::ParenthesisOpen),
            b")" => Ok(Operator::ParenthesisClose),

            b"=" => Ok(Operator::Assignment),

            b"." => Ok(Operator::Dot),

            b"+" => Ok(Operator::ArithmeticAddition),
            b"-" => Ok(Operator::ArithmeticSubtractionOrArithmeticNegation),
            b"*" => Ok(Operator::ArithmeticMultiplication),
            b"/" => Ok(Operator::ArithmeticDivision),
            b"%" => Ok(Operator::ArithmeticRemainder),
            b"\\" => Ok(Operator::ArithmeticInversion),

            b"&&" => Ok(Operator::BooleanAnd),
            b"||" => Ok(Operator::BooleanOr),
            b"^^" => Ok(Operator::BooleanXor),
            b"!" => Ok(Operator::BooleanNot),

            b"==" => Ok(Operator::ComparisonEqual),
            b"!=" => Ok(Operator::ComparisonNotEqual),
            b"<=" => Ok(Operator::ComparisonLesserEqual),
            b">=" => Ok(Operator::ComparisonGreaterEqual),
            b"<" => Ok(Operator::ComparisonLesser),
            b">" => Ok(Operator::ComparisonGreater),

            _unknown => Err(Error::Unknown),
        }
    }
}
