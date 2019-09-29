//!
//! The operator expression element.
//!

use std::fmt;

use crate::lexical::Location;
use crate::syntax::OperatorExpressionObject;

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    pub location: Location,
    pub object: OperatorExpressionObject,
}

impl Element {
    pub fn new(location: Location, object: OperatorExpressionObject) -> Self {
        Self { location, object }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.object)
    }
}
