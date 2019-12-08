//!
//! The semantic analyzer integer value element.
//!

mod error;

pub use self::error::Error;

use std::fmt;

use crate::semantic::Type;

#[derive(Clone, PartialEq)]
pub struct Integer {
    pub is_signed: bool,
    pub bitlength: usize,
}

impl Integer {
    pub fn new(is_signed: bool, bitlength: usize) -> Self {
        Self {
            is_signed,
            bitlength,
        }
    }

    pub fn r#type(&self) -> Type {
        match (self.is_signed, self.bitlength) {
            (false, crate::BITLENGTH_FIELD) => Type::Field,
            (is_signed, bitlength) => Type::new_integer(is_signed, bitlength),
        }
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.is_signed == other.is_signed && self.bitlength == other.bitlength
    }

    pub fn greater_equals(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorGreaterEqualsOperandTypesMismatch(
                self.r#type(),
                other.r#type(),
            ));
        }

        Ok(())
    }

    pub fn lesser_equals(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorLesserEqualsOperandTypesMismatch(
                self.r#type(),
                other.r#type(),
            ));
        }

        Ok(())
    }

    pub fn greater(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorGreaterOperandTypesMismatch(
                self.r#type(),
                other.r#type(),
            ));
        }

        Ok(())
    }

    pub fn lesser(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorLesserOperandTypesMismatch(
                self.r#type(),
                other.r#type(),
            ));
        }

        Ok(())
    }

    pub fn add(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorAdditionOperandTypesMismatch(
                self.r#type(),
                other.r#type(),
            ));
        }

        Ok(())
    }

    pub fn subtract(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorSubtractionOperandTypesMismatch(
                self.r#type(),
                other.r#type(),
            ));
        }

        Ok(())
    }

    pub fn multiply(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorMultiplicationOperandTypesMismatch(
                self.r#type(),
                other.r#type(),
            ));
        }

        Ok(())
    }

    pub fn divide(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorDivisionOperandTypesMismatch(
                self.r#type(),
                other.r#type(),
            ));
        }

        Ok(())
    }

    pub fn remainder(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorRemainderOperandTypesMismatch(
                self.r#type(),
                other.r#type(),
            ));
        }

        Ok(())
    }

    pub fn negate(&self) -> Result<(), Error> {
        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::Negation(self.bitlength));
        }

        Ok(())
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Type::new_integer(self.is_signed, self.bitlength))
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
