//!
//! The type variant.
//!

use crate::syntax::tree::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    Unit,
    Boolean,
    IntegerUnsigned { bitlength: usize },
    IntegerSigned { bitlength: usize },
    Field,
    Array { inner: Box<Self>, size: Expression },
    Tuple { inners: Vec<Self> },
    Alias { path: Expression },
}

impl Variant {
    pub fn unit() -> Self {
        Self::Unit
    }

    pub fn boolean() -> Self {
        Self::Boolean
    }

    pub fn integer(is_signed: bool, bitlength: usize) -> Self {
        if is_signed {
            Self::integer_signed(bitlength)
        } else {
            Self::integer_unsigned(bitlength)
        }
    }

    pub fn integer_unsigned(bitlength: usize) -> Self {
        Self::IntegerUnsigned { bitlength }
    }

    pub fn integer_signed(bitlength: usize) -> Self {
        Self::IntegerSigned { bitlength }
    }

    pub fn field() -> Self {
        Self::Field
    }

    pub fn array(inner: Self, size: Expression) -> Self {
        Self::Array {
            inner: Box::new(inner),
            size,
        }
    }

    pub fn tuple(inners: Vec<Self>) -> Self {
        Self::Tuple { inners }
    }

    pub fn alias(path: Expression) -> Self {
        Self::Alias { path }
    }
}
