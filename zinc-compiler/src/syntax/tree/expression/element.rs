//!
//! The expression element.
//!

use crate::lexical::Location;
use crate::syntax::tree::expression::object::Object as ExpressionObject;

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    pub location: Location,
    pub object: ExpressionObject,
}

impl Element {
    pub fn new(location: Location, object: ExpressionObject) -> Self {
        Self { location, object }
    }
}
