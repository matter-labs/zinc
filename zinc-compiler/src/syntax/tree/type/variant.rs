//!
//! The type variant.
//!

use crate::syntax::Expression;
use crate::syntax::IntegerLiteral;

#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    Unit,
    Boolean,
    IntegerUnsigned {
        bitlength: usize,
    },
    IntegerSigned {
        bitlength: usize,
    },
    Field,
    Array {
        type_variant: Box<Self>,
        size: IntegerLiteral,
    },
    Tuple {
        type_variants: Vec<Self>,
    },
    Reference {
        inner: Box<Self>,
    },
    Alias {
        path: Expression,
    },
}

impl Variant {
    pub fn new_unit() -> Self {
        Self::Unit
    }

    pub fn new_boolean() -> Self {
        Self::Boolean
    }

    pub fn new_integer(is_signed: bool, bitlength: usize) -> Self {
        if is_signed {
            Self::new_integer_signed(bitlength)
        } else {
            Self::new_integer_unsigned(bitlength)
        }
    }

    pub fn new_integer_unsigned(bitlength: usize) -> Self {
        Self::IntegerUnsigned { bitlength }
    }

    pub fn new_integer_signed(bitlength: usize) -> Self {
        Self::IntegerSigned { bitlength }
    }

    pub fn new_field() -> Self {
        Self::Field
    }

    pub fn new_array(type_variant: Self, size: IntegerLiteral) -> Self {
        Self::Array {
            type_variant: Box::new(type_variant),
            size,
        }
    }

    pub fn new_tuple(type_variants: Vec<Self>) -> Self {
        Self::Tuple { type_variants }
    }

    pub fn new_reference(inner: Self) -> Self {
        Self::Reference {
            inner: Box::new(inner),
        }
    }

    pub fn new_alias(path: Expression) -> Self {
        Self::Alias { path }
    }
}
