//!
//! The semantic analyzer test function element.
//!

#[cfg(test)]
mod tests;

use std::fmt;

use zinc_lexical::Location;

///
/// The semantic analyzer test function element.
///
#[derive(Debug, Clone)]
pub struct Function {
    /// The function location in the code.
    pub location: Location,
    /// The function identifier.
    pub identifier: String,
    /// The unique function ID, allocated during the semantic analysis,
    pub type_id: usize,
}

impl Function {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(location: Location, identifier: String, type_id: usize) -> Self {
        Self {
            location,
            identifier,
            type_id,
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn {}()", self.identifier,)
    }
}
