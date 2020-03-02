//!
//! The member integer.
//!

pub mod builder;

use crate::lexical::Location;
use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;

#[derive(Debug, Clone, PartialEq)]
pub struct MemberInteger {
    pub location: Location,
    pub literal: IntegerLiteral,
}

impl MemberInteger {
    pub fn new(location: Location, literal: IntegerLiteral) -> Self {
        Self { location, literal }
    }
}
