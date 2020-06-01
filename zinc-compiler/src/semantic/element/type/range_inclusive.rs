//!
//! The semantic analyzer range inclusive type element.
//!

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type;

#[derive(Debug, Clone)]
pub struct RangeInclusive {
    pub location: Option<Location>,
    pub r#type: Box<Type>,
}

impl RangeInclusive {
    pub fn new(location: Option<Location>, r#type: Box<Type>) -> Self {
        Self { location, r#type }
    }
}

impl fmt::Display for RangeInclusive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{0}..={0}", self.r#type)
    }
}
