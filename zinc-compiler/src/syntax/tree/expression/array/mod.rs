//!
//! The array expression.
//!

mod builder;

pub use self::builder::Builder;

use crate::lexical::Location;
use crate::syntax;
use crate::syntax::IntegerLiteral;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub elements: Vec<syntax::Expression>,
    pub repeats_count: Option<IntegerLiteral>,
}

impl Expression {
    pub fn new(
        location: Location,
        elements: Vec<syntax::Expression>,
        repeats_count: Option<IntegerLiteral>,
    ) -> Self {
        Self {
            location,
            elements,
            repeats_count,
        }
    }
}
