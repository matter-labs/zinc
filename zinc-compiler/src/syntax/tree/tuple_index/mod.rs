//!
//! The tuple index integer.
//!

pub mod builder;

use crate::lexical::token::location::Location;
use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;

#[derive(Debug, Clone, PartialEq)]
pub struct TupleIndex {
    pub location: Location,
    pub literal: IntegerLiteral,
}

impl TupleIndex {
    pub fn new(location: Location, literal: IntegerLiteral) -> Self {
        Self { location, literal }
    }
}
