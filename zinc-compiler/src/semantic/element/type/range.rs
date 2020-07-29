//!
//! The semantic analyzer range type element.
//!

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type;

///
/// The semantic analyzer range type element.
///
#[derive(Debug, Clone)]
pub struct Range {
    /// The type location in the code.
    pub location: Option<Location>,
    /// The range bounds type.
    pub r#type: Box<Type>,
}

impl Range {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(location: Option<Location>, r#type: Box<Type>) -> Self {
        Self { location, r#type }
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{0}..{0}", self.r#type)
    }
}
