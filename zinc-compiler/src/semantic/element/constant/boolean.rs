//!
//! The semantic analyzer constant boolean element.
//!

use std::fmt;

use zinc_lexical::Location;
use zinc_syntax::BooleanLiteral;

use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;

///
/// Simple wrapper around the `bool` constant.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Boolean {
    /// The location, where the value appears in the code.
    pub location: Location,
    /// The inner boolean value.
    pub inner: bool,
}

impl Boolean {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(location: Location, inner: bool) -> Self {
        Self { location, inner }
    }

    ///
    /// Checks whether the value is true.
    ///
    pub fn is_true(&self) -> bool {
        self.inner
    }

    ///
    /// Checks whether the value is false.
    ///
    pub fn is_false(&self) -> bool {
        !self.inner
    }

    ///
    /// Executes the `||` logical OR operator.
    ///
    pub fn or(self, other: Self) -> Self {
        let result = self.inner || other.inner;
        Self {
            location: self.location,
            inner: result,
        }
    }

    ///
    /// Executes the `^^` logical XOR operator.
    ///
    pub fn xor(self, other: Self) -> Self {
        let result = !self.inner && other.inner || self.inner && !other.inner;
        Self {
            location: self.location,
            inner: result,
        }
    }

    ///
    /// Executes the `&&` logical AND operator.
    ///
    pub fn and(self, other: Self) -> Self {
        let result = self.inner && other.inner;
        Self {
            location: self.location,
            inner: result,
        }
    }

    ///
    /// Executes the `==` equals comparison operator.
    ///
    pub fn equals(self, other: Self) -> Self {
        let result = self.inner == other.inner;
        Self {
            location: self.location,
            inner: result,
        }
    }

    ///
    /// Executes the `!=` not-equals comparison operator.
    ///
    pub fn not_equals(self, other: Self) -> Self {
        let result = self.inner != other.inner;
        Self {
            location: self.location,
            inner: result,
        }
    }

    ///
    /// Executes the `!` logical NOT operator.
    ///
    pub fn not(self) -> Self {
        let result = !self.inner;
        Self {
            location: self.location,
            inner: result,
        }
    }
}

impl ITyped for Boolean {
    fn r#type(&self) -> Type {
        Type::boolean(None)
    }

    fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.r#type() == other.r#type()
    }
}

impl From<BooleanLiteral> for Boolean {
    fn from(literal: BooleanLiteral) -> Self {
        Self {
            location: literal.location,
            inner: literal.into(),
        }
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}'", self.inner)
    }
}
