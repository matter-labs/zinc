//!
//! The semantic analyzer constant boolean element.
//!

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type;
use crate::syntax::tree::literal::boolean::Literal as BooleanLiteral;

///
/// Simple wrapper around the `bool` value.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Boolean {
    pub location: Location,
    pub inner: bool,
}

impl Boolean {
    pub fn new(location: Location, inner: bool) -> Self {
        Self { location, inner }
    }

    pub fn is_true(&self) -> bool {
        self.inner
    }

    pub fn is_false(&self) -> bool {
        !self.inner
    }

    pub fn r#type(&self) -> Type {
        Type::boolean(None)
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.r#type() == other.r#type()
    }

    pub fn or(self, other: Self) -> Self {
        let result = self.inner || other.inner;
        Self {
            location: self.location,
            inner: result,
        }
    }

    pub fn xor(self, other: Self) -> Self {
        let result = !self.inner && other.inner || self.inner && !other.inner;
        Self {
            location: self.location,
            inner: result,
        }
    }

    pub fn and(self, other: Self) -> Self {
        let result = self.inner && other.inner;
        Self {
            location: self.location,
            inner: result,
        }
    }

    pub fn equals(self, other: Self) -> Self {
        let result = self.inner == other.inner;
        Self {
            location: self.location,
            inner: result,
        }
    }

    pub fn not_equals(self, other: Self) -> Self {
        let result = self.inner != other.inner;
        Self {
            location: self.location,
            inner: result,
        }
    }

    pub fn not(self) -> Self {
        let result = !self.inner;
        Self {
            location: self.location,
            inner: result,
        }
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'{}'", self.inner)
    }
}
