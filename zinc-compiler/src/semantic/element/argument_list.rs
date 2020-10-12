//!
//! The semantic analyzer argument list element.
//!

use std::fmt;

use crate::semantic::element::Element;
use zinc_lexical::Location;

///
/// A function argument list.
///
#[derive(Debug, Clone)]
pub struct ArgumentList {
    /// The argument list location in the code.
    pub location: Location,
    /// The argument list semantic elements.
    pub arguments: Vec<Element>,
}

impl ArgumentList {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(location: Location, arguments: Vec<Element>) -> Self {
        Self {
            location,
            arguments,
        }
    }
}

impl fmt::Display for ArgumentList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.arguments
                .iter()
                .map(|element| element.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
