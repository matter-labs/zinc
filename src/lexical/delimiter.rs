//!
//! The delimiter lexeme.
//!

use std::convert::TryFrom;

use crate::lexical::Operator;

#[derive(Debug)]
pub enum Delimiter {
    BracketCurlyOpen,
    BracketCurlyClose,
    BracketSquareOpen,
    BracketSquareClose,
    BracketRoundOpen,
    BracketRoundClose,
    BracketAngleOpen,
    BracketAngleClose,
}

impl Delimiter {
    pub fn to_operator(&self) -> Option<Operator> {
        Some(match self {
            Delimiter::BracketRoundOpen => Operator::ParenthesisOpen,
            Delimiter::BracketRoundClose => Operator::ParenthesisClose,
            Delimiter::BracketAngleOpen => Operator::ComparisonLesser,
            Delimiter::BracketAngleClose => Operator::ComparisonGreater,
            _ => return None,
        })
    }
}

impl TryFrom<u8> for Delimiter {
    type Error = u8;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        Ok(match byte {
            b'{' => Delimiter::BracketCurlyOpen,
            b'}' => Delimiter::BracketCurlyClose,
            b'[' => Delimiter::BracketSquareOpen,
            b']' => Delimiter::BracketSquareClose,
            b'(' => Delimiter::BracketRoundOpen,
            b')' => Delimiter::BracketRoundClose,
            b'<' => Delimiter::BracketAngleOpen,
            b'>' => Delimiter::BracketAngleClose,
            _ => return Err(byte),
        })
    }
}
