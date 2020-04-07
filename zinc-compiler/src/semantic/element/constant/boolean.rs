//!
//! The semantic analyzer constant boolean element.
//!

use std::fmt;

use crate::semantic::element::r#type::Type;
use crate::syntax::tree::literal::boolean::Literal as BooleanLiteral;

///
/// Simple wrapper around the `bool` value.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Boolean {
    pub inner: bool,
}

impl Boolean {
    pub fn new(inner: bool) -> Self {
        Self { inner }
    }

    pub fn r#type(&self) -> Type {
        Type::boolean()
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.r#type() == other.r#type()
    }

    pub fn or(self, other: Self) -> Self {
        let result = self.inner || other.inner;
        Self { inner: result }
    }

    pub fn xor(self, other: Self) -> Self {
        let result = !self.inner && other.inner || self.inner && !other.inner;
        Self { inner: result }
    }

    pub fn and(self, other: Self) -> Self {
        let result = self.inner && other.inner;
        Self { inner: result }
    }

    pub fn equals(self, other: Self) -> Self {
        let result = self.inner == other.inner;
        Self { inner: result }
    }

    pub fn not_equals(self, other: Self) -> Self {
        let result = self.inner != other.inner;
        Self { inner: result }
    }

    pub fn not(self) -> Self {
        let result = !self.inner;
        Self { inner: result }
    }
}

impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        Self::new(value)
    }
}

impl From<BooleanLiteral> for Boolean {
    fn from(value: BooleanLiteral) -> Self {
        let value: bool = value.into();
        Self::from(value)
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "boolean constant '{}'", self.inner)
    }
}
