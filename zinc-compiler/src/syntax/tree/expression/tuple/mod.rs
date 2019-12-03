//!
//! The tuple expression.
//!

mod builder;

pub use self::builder::Builder;

use crate::lexical::Location;
use crate::syntax;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub elements: Vec<syntax::Expression>,
}

impl Expression {
    pub fn new(location: Location, elements: Vec<syntax::Expression>) -> Self {
        Self { location, elements }
    }
}
