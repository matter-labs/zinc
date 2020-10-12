//!
//! The semantic analyzer array type element.
//!

use std::fmt;

use crate::semantic::element::r#type::Type;
use zinc_lexical::Location;

///
/// The semantic analyzer array type element.
///
#[derive(Debug, Clone)]
pub struct Array {
    /// The type location in the code.
    pub location: Option<Location>,
    /// The array element type.
    pub r#type: Box<Type>,
    /// The array size.
    pub size: usize,
}

impl Array {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(location: Option<Location>, r#type: Box<Type>, size: usize) -> Self {
        Self {
            location,
            r#type,
            size,
        }
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}; {}]", self.r#type, self.size)
    }
}
