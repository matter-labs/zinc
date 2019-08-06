//!
//! The operator lexeme.
//!

use std::str::FromStr;

use failure::Fail;

#[derive(Debug)]
pub enum Operator {
    ParenthesesOpen,
    ParenthesesClose,

    Assignment,

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
    ComparisonGreaterEqual,
    ComparisonGreater,
    ComparisonLesserEqual,
    ComparisonLesser,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "unknown")]
    Unknown,
}

impl FromStr for Operator {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "(" => Ok(Operator::ParenthesesOpen),
            ")" => Ok(Operator::ParenthesesClose),

            "=" => Ok(Operator::Assignment),

            "+" => Ok(Operator::ArithmeticAddition),
            "-" => Ok(Operator::ArithmeticSubtractionOrArithmeticNegation),
            "*" => Ok(Operator::ArithmeticMultiplication),
            "/" => Ok(Operator::ArithmeticDivision),
            "%" => Ok(Operator::ArithmeticRemainder),
            "\\" => Ok(Operator::ArithmeticInversion),

            "&&" => Ok(Operator::BooleanAnd),
            "||" => Ok(Operator::BooleanOr),
            "^^" => Ok(Operator::BooleanXor),
            "!" => Ok(Operator::BooleanNot),

            "==" => Ok(Operator::ComparisonEqual),
            ">=" => Ok(Operator::ComparisonGreaterEqual),
            ">" => Ok(Operator::ComparisonGreater),
            "<=" => Ok(Operator::ComparisonLesserEqual),
            "<" => Ok(Operator::ComparisonLesser),

            _unknown => Err(Error::Unknown),
        }
    }
}
