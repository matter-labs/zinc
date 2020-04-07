//!
//! The semantic analyzer integer value element.
//!

mod tests;

pub mod error;

use std::fmt;

use crate::semantic::element::r#type::enumeration::Enumeration;
use crate::semantic::element::r#type::Type;

use self::error::Error;

///
/// Integer values consist of the value, sign, and bitlength.
/// If a value belongs to an enumeration, the enumeration type is stored in `enumeration`.
/// Enumeration uniquely defines the value type, even if the sign and bitlength are the same.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Integer {
    pub is_signed: bool,
    pub bitlength: usize,
    pub enumeration: Option<Enumeration>,
}

impl Integer {
    pub fn new(is_signed: bool, bitlength: usize) -> Self {
        Self {
            is_signed,
            bitlength,
            enumeration: None,
        }
    }

    pub fn set_enumeration(&mut self, enumeration: Enumeration) {
        self.enumeration = Some(enumeration);
    }

    pub fn r#type(&self) -> Type {
        match self.enumeration {
            Some(ref enumeration) => Type::Enumeration(enumeration.to_owned()),
            None => Type::scalar(self.is_signed, self.bitlength),
        }
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.is_signed == other.is_signed
            && self.bitlength == other.bitlength
            && match (self.enumeration.as_ref(), other.enumeration.as_ref()) {
                (Some(enumeration_1), Some(enumeration_2)) => enumeration_1 == enumeration_2,
                (None, None) => true,
                _ => false,
            }
    }

    pub fn equals(self, other: Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchEquals {
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        Ok(())
    }

    pub fn not_equals(self, other: Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchNotEquals {
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        Ok(())
    }

    pub fn greater_equals(self, other: Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchGreaterEquals {
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        Ok(())
    }

    pub fn lesser_equals(self, other: Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchLesserEquals {
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        Ok(())
    }

    pub fn greater(self, other: Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchGreater {
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        Ok(())
    }

    pub fn lesser(self, other: Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchLesser {
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        Ok(())
    }

    pub fn bitwise_or(self, other: Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchBitwiseOr {
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldBitwise);
        }

        Ok(self)
    }

    pub fn bitwise_xor(self, other: Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchBitwiseXor {
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldBitwise);
        }

        Ok(self)
    }

    pub fn bitwise_and(self, other: Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchBitwiseAnd {
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldBitwise);
        }

        Ok(self)
    }

    pub fn bitwise_shift_left(self, other: Self) -> Result<Self, Error> {
        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldBitwise);
        }

        if other.is_signed {
            return Err(
                Error::OperatorBitwiseShiftRightSecondOperatorExpectedUnsigned {
                    found: other.to_string(),
                },
            );
        }

        Ok(self)
    }

    pub fn bitwise_shift_right(self, other: Self) -> Result<Self, Error> {
        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldBitwise);
        }

        if other.is_signed {
            return Err(
                Error::OperatorBitwiseShiftRightSecondOperatorExpectedUnsigned {
                    found: other.to_string(),
                },
            );
        }

        Ok(self)
    }

    pub fn add(self, other: Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchAddition {
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        Ok(self)
    }

    pub fn subtract(self, other: Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchSubtraction {
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        Ok(self)
    }

    pub fn multiply(self, other: Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchMultiplication {
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        Ok(self)
    }

    pub fn divide(self, other: Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchDivision {
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldDivision);
        }

        Ok(self)
    }

    pub fn remainder(self, other: Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchRemainder {
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldRemainder);
        }

        Ok(self)
    }

    pub fn cast(mut self, is_signed: bool, bitlength: usize) -> Result<Self, Error> {
        self.is_signed = is_signed;
        self.bitlength = bitlength;
        self.enumeration = None;

        Ok(self)
    }

    pub fn bitwise_not(self) -> Result<Self, Error> {
        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldBitwise);
        }

        Ok(self)
    }

    pub fn negate(mut self) -> Result<Self, Error> {
        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldNegation);
        }

        self.is_signed = true;

        Ok(self)
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<integer> of type '{}'", self.r#type())
    }
}
