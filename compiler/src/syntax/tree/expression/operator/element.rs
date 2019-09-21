//!
//! The expression element.
//!

use std::fmt;

use serde_derive::Serialize;

use crate::lexical::Location;
use crate::syntax::OperatorExpressionObject;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Element {
    #[serde(skip_serializing)]
    pub location: Location,
    #[serde(flatten)]
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
