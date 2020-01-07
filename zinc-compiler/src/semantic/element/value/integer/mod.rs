//!
//! The semantic analyzer integer value element.
//!

mod error;

pub use self::error::Error;

use std::fmt;

use crate::semantic::Type;

#[derive(Debug, Clone, PartialEq)]
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

    pub fn equals(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchEquals(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        Ok(())
    }

    pub fn not_equals(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchNotEquals(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        Ok(())
    }

    pub fn greater_equals(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchGreaterEquals(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        Ok(())
    }

    pub fn lesser_equals(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchLesserEquals(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        Ok(())
    }

    pub fn greater(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchGreater(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        Ok(())
    }

    pub fn lesser(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchLesser(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        Ok(())
    }

    pub fn add(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchAddition(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        Ok(())
    }

    pub fn subtract(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchSubtraction(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        Ok(())
    }

    pub fn multiply(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchMultiplication(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        Ok(())
    }

    pub fn divide(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchDivision(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        Ok(())
    }

    pub fn remainder(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchRemainder(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        Ok(())
    }

    pub fn cast(&mut self, is_signed: bool, bitlength: usize) -> Result<(), Error> {
        self.is_signed = is_signed;
        self.bitlength = bitlength;
        Ok(())
    }

    pub fn negate(&self) -> Result<(), Error> {
        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::FieldNegation);
        }

        Ok(())
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.r#type())
    }
}
