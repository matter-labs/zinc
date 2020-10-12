//!
//! The semantic analyzer tuple type element.
//!

use std::fmt;

use crate::semantic::element::r#type::Type;
use zinc_lexical::Location;

///
/// The semantic analyzer tuple type element.
///
#[derive(Debug, Clone)]
pub struct Tuple {
    /// The type location in the code.
    pub location: Option<Location>,
    /// The tuple element types.
    pub types: Vec<Type>,
}

impl Tuple {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(location: Option<Location>, types: Vec<Type>) -> Self {
        Self { location, types }
    }
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({})",
            self.types
                .iter()
                .map(|r#type| r#type.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
