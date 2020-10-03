//!
//! The integer type inference result.
//!

use crate::inference::r#type::Type;

///
/// The binary operation type inference result.
///
#[derive(Debug, PartialEq)]
pub struct Binary {
    /// The first operand inferred type, if the operand is a literal.
    pub first: Option<Type>,
    /// The second operand inferred type, if the operand is a literal.
    pub second: Option<Type>,
}

impl Binary {
    ///
    /// A shortcut constructor.
    ///
    pub fn none() -> Self {
        Self {
            first: None,
            second: None,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn first(first: Type) -> Self {
        Self {
            first: Some(first),
            second: None,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn second(second: Type) -> Self {
        Self {
            first: None,
            second: Some(second),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn both(first: Type, second: Type) -> Self {
        Self {
            first: Some(first),
            second: Some(second),
        }
    }
}
