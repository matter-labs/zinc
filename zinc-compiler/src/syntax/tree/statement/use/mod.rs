//!
//! The use statement.
//!

mod builder;

pub use self::builder::Builder;

use crate::lexical::Location;
use crate::syntax::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Use {
    pub location: Location,
    pub path: Expression,
}

impl Use {
    pub fn new(location: Location, path: Expression) -> Self {
        Self { location, path }
    }
}
