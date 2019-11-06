//!
//! The semantic analyzer element integer value.
//!

mod error;

pub use self::error::Error;

use std::fmt;

use num_bigint::BigInt;
use num_traits::Zero;

use zrust_bytecode::Push;

use crate::lexical::IntegerLiteral;
use crate::semantic;
use crate::syntax::TypeVariant;

#[derive(Clone)]
pub struct Integer {
    pub value: BigInt,
    pub is_signed: bool,
    pub bitlength: usize,
}

impl Integer {
    pub fn new_from_usize(bitlength: usize) -> Self {
        Self {
            value: BigInt::zero(),
            is_signed: false,
            bitlength,
        }
    }

    pub fn new_from_literal(
        literal: IntegerLiteral,
        bitlength: Option<usize>,
    ) -> Result<Self, Error> {
        let (value, inferred_bitlength) =
            semantic::infer_integer_literal(&literal).map_err(Error::Inference)?;
        let bitlength = bitlength.unwrap_or(inferred_bitlength);

        Ok(Self {
            value,
            is_signed: false,
            bitlength,
        })
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

        let result = Self {
            value: self.value + other.value,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
        };

        Ok(result)
    }

    pub fn subtract(self, other: Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        let result = Self {
            value: self.value - other.value,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
        };

        Ok(result)
    }

    pub fn multiply(self, other: Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        let result = Self {
            value: self.value * other.value,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
        };

        Ok(result)
    }

    pub fn divide(self, other: Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        let result = Self {
            value: self.value / other.value,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
        };

        Ok(result)
    }

    pub fn modulo(self, other: Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        let result = Self {
            value: self.value % other.value,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
        };

        Ok(result)
    }

    pub fn cast(self, to: TypeVariant) -> Result<Self, Error> {
        let from = self.type_variant();
        semantic::validate_casting(&from, &to).map_err(Error::Casting)?;
        let (is_signed, bitlength) = match to {
            TypeVariant::IntegerUnsigned { bitlength } => (false, bitlength),
            TypeVariant::IntegerSigned { bitlength } => (true, bitlength),
            TypeVariant::Field => (false, crate::BITLENGTH_FIELD),
            _ => panic!("Always checked by some branches above"),
        };

        let result = Self {
            value: self.value,
            is_signed,
            bitlength,
        };

        Ok(result)
    }

    pub fn negate(self) -> Result<Self, Error> {
        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::Negation(self.bitlength));
        }

        let result = Self {
            value: -self.value,
            is_signed: true,
            bitlength: self.bitlength,
        };

        Ok(result)
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.value, self.type_variant())
    }
}

impl PartialEq<Self> for Integer {
    fn eq(&self, other: &Self) -> bool {
        self.has_the_same_type_as(other)
    }
}

impl Into<Push> for Integer {
    fn into(self) -> Push {
        Push::new(self.value, self.is_signed, self.bitlength)
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
