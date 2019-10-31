//!
//! The semantic analyzer element integer value.
//!

mod error;

pub use self::error::Error;

use std::fmt;

use crate::lexical::IntegerLiteral;
use crate::semantic;
use crate::syntax::TypeVariant;

#[derive(Clone)]
pub struct Integer {
    pub is_signed: bool,
    pub bitlength: usize,
}

impl Integer {
    pub fn new_from_usize(bitlength: usize) -> Self {
        Self {
            is_signed: false,
            bitlength,
        }
    }

    pub fn new_from_literal(literal: IntegerLiteral, bitlength: Option<usize>) -> Self {
        let bitlength = bitlength.unwrap_or(
            semantic::infer_integer_literal(&literal).expect("Integer semantic.inference error"),
        );

        Self {
            is_signed: false,
            bitlength,
        }
    }

    pub fn type_variant(&self) -> TypeVariant {
        match (self.is_signed, self.bitlength) {
            (false, crate::BITLENGTH_FIELD) => TypeVariant::new_field(),
            (is_signed, bitlength) if bitlength < crate::BITLENGTH_FIELD => {
                TypeVariant::new_integer(is_signed, bitlength)
            }
            (..) => panic!("Always checked by the branches above"),
        }
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.is_signed == other.is_signed && self.bitlength == other.bitlength
    }

    pub fn add(self, other: Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        Ok(self)
    }

    pub fn subtract(self, other: Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        Ok(self)
    }

    pub fn multiply(self, other: Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        Ok(self)
    }

    pub fn divide(self, other: Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        Ok(self)
    }

    pub fn modulo(self, other: Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        Ok(self)
    }

    pub fn negate(mut self) -> Result<Self, Error> {
        self.is_signed = true;
        Ok(self)
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.type_variant())
    }
}

impl PartialEq<Self> for Integer {
    fn eq(&self, other: &Self) -> bool {
        self.has_the_same_type_as(other)
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl fmt::Debug for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}
