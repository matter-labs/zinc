//!
//! The interpreter integer value.
//!

use std::fmt;

use num_bigint::BigInt;
use num_traits::One;
use serde_derive::Serialize;

use crate::interpreter::Boolean;
use crate::interpreter::Value;
use crate::interpreter::ValueError;
use crate::syntax::TypeVariant;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Integer {
    #[serde(skip_serializing)]
    data: BigInt,
    is_signed: bool,
    bitlength: usize,
}

impl Integer {
    pub fn new(data: BigInt, is_signed: bool, bitlength: usize) -> Self {
        Self {
            data,
            is_signed,
            bitlength,
        }
    }

    pub fn is_signed(&self) -> bool {
        self.is_signed
    }

    pub fn bitlength(&self) -> usize {
        self.bitlength
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.is_signed == other.is_signed && self.bitlength == other.bitlength
    }

    pub fn greater_equal(&self, other: &Self) -> Result<Boolean, ValueError> {
        if !self.has_the_same_type_as(other) {
            return Err(ValueError::OperandTypesMismatch(
                Value::Integer(self.clone()),
                Value::Integer(other.clone()),
            ));
        }

        Ok(Boolean::new(self.data >= other.data))
    }

    pub fn lesser_equal(&self, other: &Self) -> Result<Boolean, ValueError> {
        if !self.has_the_same_type_as(other) {
            return Err(ValueError::OperandTypesMismatch(
                Value::Integer(self.clone()),
                Value::Integer(other.clone()),
            ));
        }

        Ok(Boolean::new(self.data <= other.data))
    }

    pub fn greater(&self, other: &Self) -> Result<Boolean, ValueError> {
        if !self.has_the_same_type_as(other) {
            return Err(ValueError::OperandTypesMismatch(
                Value::Integer(self.clone()),
                Value::Integer(other.clone()),
            ));
        }

        Ok(Boolean::new(self.data > other.data))
    }

    pub fn lesser(&self, other: &Self) -> Result<Boolean, ValueError> {
        if !self.has_the_same_type_as(other) {
            return Err(ValueError::OperandTypesMismatch(
                Value::Integer(self.clone()),
                Value::Integer(other.clone()),
            ));
        }

        Ok(Boolean::new(self.data < other.data))
    }

    pub fn add(self, other: Self) -> Result<Self, ValueError> {
        if !self.has_the_same_type_as(&other) {
            return Err(ValueError::OperandTypesMismatch(
                Value::Integer(self),
                Value::Integer(other),
            ));
        }

        Ok(Integer::new(
            self.data + other.data,
            self.is_signed,
            self.bitlength,
        ))
    }

    pub fn subtract(self, other: Self) -> Result<Self, ValueError> {
        if !self.has_the_same_type_as(&other) {
            return Err(ValueError::OperandTypesMismatch(
                Value::Integer(self),
                Value::Integer(other),
            ));
        }

        Ok(Integer::new(
            self.data - other.data,
            self.is_signed,
            self.bitlength,
        ))
    }

    pub fn multiply(self, other: Self) -> Result<Self, ValueError> {
        if !self.has_the_same_type_as(&other) {
            return Err(ValueError::OperandTypesMismatch(
                Value::Integer(self),
                Value::Integer(other),
            ));
        }

        Ok(Integer::new(
            self.data * other.data,
            self.is_signed,
            self.bitlength,
        ))
    }

    pub fn divide(self, other: Self) -> Result<Self, ValueError> {
        if !self.has_the_same_type_as(&other) {
            return Err(ValueError::OperandTypesMismatch(
                Value::Integer(self),
                Value::Integer(other),
            ));
        }

        Ok(Integer::new(
            self.data / other.data,
            self.is_signed,
            self.bitlength,
        ))
    }

    pub fn modulo(self, other: Self) -> Result<Self, ValueError> {
        if !self.has_the_same_type_as(&other) {
            return Err(ValueError::OperandTypesMismatch(
                Value::Integer(self),
                Value::Integer(other),
            ));
        }

        Ok(Integer::new(
            self.data % other.data,
            self.is_signed,
            self.bitlength,
        ))
    }

    pub fn negate(self) -> Self {
        Integer::new(-self.data, true, self.bitlength)
    }

    pub fn cast(mut self, type_variant: TypeVariant) -> Result<Self, ValueError> {
        match (self.is_signed, self.bitlength, type_variant) {
            (false, b1, TypeVariant::Uint { bitlength: b2 }) if b1 >= 248 || b1 >= b2 => {
                return Err(ValueError::CastingToLesserOrEqualBitlength(b1, b2));
            }
            (false, b1, TypeVariant::Int { bitlength: b2 }) if b1 >= 240 || b1 + 8 >= b2 => {
                return Err(ValueError::CastingToLesserOrEqualBitlength(b1, b2));
            }
            (true, b1, TypeVariant::Int { bitlength: b2 }) if b1 >= 248 || b1 >= b2 => {
                return Err(ValueError::CastingToLesserOrEqualBitlength(b1, b2));
            }
            (true, b1, TypeVariant::Uint { bitlength: b2 }) if b1 >= 248 || b1 >= b2 => {
                if b1 >= b2 {
                    return Err(ValueError::CastingToLesserOrEqualBitlength(b1, b2));
                }
            }
            (false, b1, TypeVariant::Uint { bitlength: b2 }) if b1 >= 254 => {
                return Err(ValueError::CastingToLesserOrEqualBitlength(b1, b2));
            }
            (false, b1, TypeVariant::Int { bitlength: b2 }) if b1 >= 254 => {
                return Err(ValueError::CastingToLesserOrEqualBitlength(b1, b2));
            }
            (_, _, type_variant) => {
                let (is_signed, bitlength) = match type_variant {
                    TypeVariant::Uint { bitlength } => (false, bitlength),
                    TypeVariant::Int { bitlength } => (true, bitlength),
                    TypeVariant::Field => (false, 254),
                    type_variant => {
                        return Err(ValueError::CastingToInvalidType(self, type_variant))
                    }
                };
                self.is_signed = is_signed;
                self.bitlength = bitlength;
            }
        }

        Ok(self)
    }

    pub fn inc(self) -> Result<Self, ValueError> {
        Ok(Integer::new(
            self.data + BigInt::one(),
            self.is_signed,
            self.bitlength,
        ))
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r#type = match (self.is_signed, self.bitlength) {
            (true, bitlength) => format!("int{}", bitlength),
            (false, 254) => "field".to_owned(),
            (false, bitlength) => format!("uint{}", bitlength),
        };
        write!(f, "{}: {}", self.data, r#type)
    }
}
