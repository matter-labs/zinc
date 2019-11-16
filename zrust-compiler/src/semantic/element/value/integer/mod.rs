//!
//! The semantic analyzer element integer value.
//!

mod error;

pub use self::error::Error;

use std::convert::TryFrom;
use std::fmt;

use num_bigint::BigInt;
use num_traits::Zero;

use zrust_bytecode::Push;

use crate::lexical::IntegerLiteral;
use crate::semantic;
use crate::syntax::TypeVariant;

#[derive(Clone, PartialEq)]
pub struct Integer {
    pub value: Option<BigInt>,
    pub is_signed: bool,
    pub bitlength: usize,
}

impl Integer {
    pub fn new(is_signed: bool, bitlength: usize) -> Self {
        Self {
            value: None,
            is_signed,
            bitlength,
        }
    }

    pub fn type_variant(&self) -> TypeVariant {
        match (self.is_signed, self.bitlength) {
            (false, crate::BITLENGTH_FIELD) => TypeVariant::new_field(),
            (is_signed, bitlength) if bitlength < crate::BITLENGTH_FIELD => {
                TypeVariant::new_integer(is_signed, bitlength)
            }
            _ => panic!("Always checked by the branches above"),
        }
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.is_signed == other.is_signed && self.bitlength == other.bitlength
    }

    pub fn greater_equals(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        Ok(())
    }

    pub fn lesser_equals(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        Ok(())
    }

    pub fn greater(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        Ok(())
    }

    pub fn lesser(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        Ok(())
    }

    pub fn add(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        Ok(())
    }

    pub fn subtract(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        Ok(())
    }

    pub fn multiply(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        Ok(())
    }

    pub fn divide(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        if let Some(ref other) = other.value {
            if other.is_zero() {
                return Err(Error::DivisionByZero);
            }
        }

        Ok(())
    }

    pub fn modulo(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        if let Some(ref other) = other.value {
            if other.is_zero() {
                return Err(Error::DivisionByZero);
            }
        }

        Ok(())
    }

    pub fn cast(&self, to: &TypeVariant) -> Result<(bool, usize), Error> {
        let from = self.type_variant();
        semantic::validate_casting(&from, &to).map_err(Error::Casting)?;
        let (is_signed, bitlength) = match to {
            TypeVariant::IntegerUnsigned { bitlength } => (false, *bitlength),
            TypeVariant::IntegerSigned { bitlength } => (true, *bitlength),
            TypeVariant::Field => (false, crate::BITLENGTH_FIELD),
            _ => panic!("Always checked by some branches above"),
        };

        Ok((is_signed, bitlength))
    }

    pub fn negate(&self) -> Result<(), Error> {
        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::Negation(self.bitlength));
        }

        Ok(())
    }

    pub fn to_push(&self) -> Push {
        Push::new(
            self.value.to_owned().expect("Must contain a value"),
            self.is_signed,
            self.bitlength,
        )
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.type_variant())
    }
}

impl TryFrom<IntegerLiteral> for Integer {
    type Error = Error;

    fn try_from(value: IntegerLiteral) -> Result<Self, Self::Error> {
        let (value, bitlength) =
            semantic::infer_integer_literal(&value).map_err(Error::Inference)?;

        Ok(Self {
            value: Some(value),
            is_signed: false,
            bitlength,
        })
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
