//!
//! The symbol lexeme.
//!

use std::fmt;

use serde_derive::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub enum Symbol {
    BracketCurlyOpen,
    BracketCurlyClose,
    BracketSquareOpen,
    BracketSquareClose,
    BracketRoundOpen,
    BracketRoundClose,

    Assignment,

    Dot,
    Colon,
    Semicolon,
    Comma,

    ArithmeticAddition,
    ArithmeticSubtractionOrArithmeticNegation,
    ArithmeticMultiplication,
    ArithmeticDivision,
    ArithmeticRemainder,
    ArithmeticInversion,

    ComparisonEqual,
    ComparisonNotEqual,
    ComparisonLesserEqual,
    ComparisonGreaterEqual,
    ComparisonLesser,
    ComparisonGreater,

    BooleanAnd,
    BooleanOr,
    BooleanXor,
    BooleanNot,
}

impl From<&[u8]> for Symbol {
    fn from(bytes: &[u8]) -> Self {
        match bytes {
            b"{" => Symbol::BracketCurlyOpen,
            b"}" => Symbol::BracketCurlyClose,
            b"[" => Symbol::BracketSquareOpen,
            b"]" => Symbol::BracketSquareClose,
            b"(" => Symbol::BracketRoundOpen,
            b")" => Symbol::BracketRoundClose,

            b"=" => Symbol::Assignment,

            b"." => Symbol::Dot,
            b":" => Symbol::Colon,
            b";" => Symbol::Semicolon,
            b"," => Symbol::Comma,

            b"+" => Symbol::ArithmeticAddition,
            b"-" => Symbol::ArithmeticSubtractionOrArithmeticNegation,
            b"*" => Symbol::ArithmeticMultiplication,
            b"/" => Symbol::ArithmeticDivision,
            b"%" => Symbol::ArithmeticRemainder,
            b"\\" => Symbol::ArithmeticInversion,

            b"&&" => Symbol::BooleanAnd,
            b"||" => Symbol::BooleanOr,
            b"^^" => Symbol::BooleanXor,
            b"!" => Symbol::BooleanNot,

            b"==" => Symbol::ComparisonEqual,
            b"!=" => Symbol::ComparisonNotEqual,
            b"<=" => Symbol::ComparisonLesserEqual,
            b">=" => Symbol::ComparisonGreaterEqual,
            b"<" => Symbol::ComparisonLesser,
            b">" => Symbol::ComparisonGreater,

            _unknown => panic!("State machine bug"),
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Symbol::BracketCurlyOpen => "{",
                Symbol::BracketCurlyClose => "}",
                Symbol::BracketSquareOpen => "[",
                Symbol::BracketSquareClose => "]",
                Symbol::BracketRoundOpen => "(",
                Symbol::BracketRoundClose => ")",

                Symbol::Assignment => "=",

                Symbol::Dot => ".",
                Symbol::Colon => ":",
                Symbol::Semicolon => ";",
                Symbol::Comma => ",",

                Symbol::ArithmeticAddition => "+",
                Symbol::ArithmeticSubtractionOrArithmeticNegation => "-",
                Symbol::ArithmeticMultiplication => "*",
                Symbol::ArithmeticDivision => "/",
                Symbol::ArithmeticRemainder => "%",
                Symbol::ArithmeticInversion => "\\",

                Symbol::BooleanAnd => "&&",
                Symbol::BooleanOr => "||",
                Symbol::BooleanXor => "^^",
                Symbol::BooleanNot => "!",

                Symbol::ComparisonEqual => "==",
                Symbol::ComparisonNotEqual => "!=",
                Symbol::ComparisonLesserEqual => "<=",
                Symbol::ComparisonGreaterEqual => ">=",
                Symbol::ComparisonLesser => "<",
                Symbol::ComparisonGreater => ">",
            }
        )
    }
}
