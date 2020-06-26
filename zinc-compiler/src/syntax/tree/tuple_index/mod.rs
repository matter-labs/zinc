//!
//! The tuple index.
//!

pub mod builder;

use crate::lexical::token::location::Location;
use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;

///
/// The tuple index.
///
#[derive(Debug, Clone, PartialEq)]
pub struct TupleIndex {
    /// The location of the syntax construction.
    pub location: Location,
    /// The tuple index integer literal.
    pub literal: IntegerLiteral,
}

impl TupleIndex {
    ///
    /// Creates a tuple index.
    ///
    pub fn new(location: Location, literal: IntegerLiteral) -> Self {
        Self { location, literal }
    }
}
