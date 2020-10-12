//!
//! The semantic analyzer constant unit element.
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
    pub location: Location,
}

impl Unit {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(location: Location) -> Self {
        Self { location }
    }
}

impl ITyped for Unit {
    fn r#type(&self) -> Type {
        Type::unit(None)
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
