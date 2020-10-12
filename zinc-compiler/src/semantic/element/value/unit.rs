//!
//! The semantic analyzer unit value element.
//!

use std::fmt;

use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;
use zinc_lexical::Location;

///
/// Simple wrapper around the `()` unit constant.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    /// The unit value location in the code.
    pub location: Option<Location>,
}

impl Unit {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(location: Option<Location>) -> Self {
        Self { location }
    }
}

impl ITyped for Unit {
    fn r#type(&self) -> Type {
        Type::unit(self.location)
    }

    fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.r#type() == other.r#type()
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'()'")
    }
}
