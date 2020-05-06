//!
//! The semantic analyzer integer value element.
//!

mod tests;

pub mod error;

use std::fmt;

use crate::generator::expression::operator::Operator as GeneratorExpressionOperator;
use crate::lexical::token::location::Location;
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
    pub location: Option<Location>,
    pub is_signed: bool,
    pub bitlength: usize,
    pub enumeration: Option<Enumeration>,
}

impl Integer {
    pub fn new(location: Option<Location>, is_signed: bool, bitlength: usize) -> Self {
        Self {
            location,
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
            None => Type::scalar(self.location, self.is_signed, self.bitlength),
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

    pub fn equals(self, other: Self) -> Result<GeneratorExpressionOperator, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchEquals {
                location: self.location.unwrap(),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let operator = GeneratorExpressionOperator::Equals;

        Ok(operator)
    }

    pub fn not_equals(self, other: Self) -> Result<GeneratorExpressionOperator, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchNotEquals {
                location: self.location.unwrap(),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let operator = GeneratorExpressionOperator::NotEquals;

        Ok(operator)
    }

    pub fn greater_equals(self, other: Self) -> Result<GeneratorExpressionOperator, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchGreaterEquals {
                location: self.location.unwrap(),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let operator = GeneratorExpressionOperator::GreaterEquals;

        Ok(operator)
    }

    pub fn lesser_equals(self, other: Self) -> Result<GeneratorExpressionOperator, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchLesserEquals {
                location: self.location.unwrap(),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let operator = GeneratorExpressionOperator::LesserEquals;

        Ok(operator)
    }

    pub fn greater(self, other: Self) -> Result<GeneratorExpressionOperator, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchGreater {
                location: self.location.unwrap(),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let operator = GeneratorExpressionOperator::Greater;

        Ok(operator)
    }

    pub fn lesser(self, other: Self) -> Result<GeneratorExpressionOperator, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchLesser {
                location: self.location.unwrap(),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let operator = GeneratorExpressionOperator::Lesser;

        Ok(operator)
    }

    pub fn bitwise_or(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchBitwiseOr {
                location: self.location.unwrap(),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        if self.is_signed {
            return Err(Error::ForbiddenSignedBitwise {
                location: self.location.unwrap(),
            });
        }

        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldBitwise {
                location: self.location.unwrap(),
            });
        }

        let operator = GeneratorExpressionOperator::BitwiseOr;

        Ok((self, operator))
    }

    pub fn bitwise_xor(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchBitwiseXor {
                location: self.location.unwrap(),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        if self.is_signed {
            return Err(Error::ForbiddenSignedBitwise {
                location: self.location.unwrap(),
            });
        }

        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldBitwise {
                location: self.location.unwrap(),
            });
        }

        let operator = GeneratorExpressionOperator::BitwiseXor;

        Ok((self, operator))
    }

    pub fn bitwise_and(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchBitwiseAnd {
                location: self.location.unwrap(),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        if self.is_signed {
            return Err(Error::ForbiddenSignedBitwise {
                location: self.location.unwrap(),
            });
        }

        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldBitwise {
                location: self.location.unwrap(),
            });
        }

        let operator = GeneratorExpressionOperator::BitwiseAnd;

        Ok((self, operator))
    }

    pub fn bitwise_shift_left(
        self,
        other: Self,
    ) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if self.is_signed {
            return Err(Error::ForbiddenSignedBitwise {
                location: self.location.unwrap(),
            });
        }

        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldBitwise {
                location: self.location.unwrap(),
            });
        }

        if other.is_signed {
            return Err(
                Error::OperatorBitwiseShiftLeftSecondOperatorExpectedUnsigned {
                    location: other.location.unwrap(),
                    found: other.to_string(),
                },
            );
        }

        let operator = GeneratorExpressionOperator::BitwiseShiftLeft;

        Ok((self, operator))
    }

    pub fn bitwise_shift_right(
        self,
        other: Self,
    ) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if self.is_signed {
            return Err(Error::ForbiddenSignedBitwise {
                location: self.location.unwrap(),
            });
        }

        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldBitwise {
                location: self.location.unwrap(),
            });
        }

        if other.is_signed {
            return Err(
                Error::OperatorBitwiseShiftRightSecondOperatorExpectedUnsigned {
                    location: other.location.unwrap(),
                    found: other.to_string(),
                },
            );
        }

        let operator = GeneratorExpressionOperator::BitwiseShiftRight;

        Ok((self, operator))
    }

    pub fn add(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchAddition {
                location: self.location.unwrap(),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let operator = GeneratorExpressionOperator::Addition;

        Ok((self, operator))
    }

    pub fn subtract(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchSubtraction {
                location: self.location.unwrap(),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let operator = GeneratorExpressionOperator::Subtraction;

        Ok((self, operator))
    }

    pub fn multiply(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchMultiplication {
                location: self.location.unwrap(),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let operator = GeneratorExpressionOperator::Multiplication;

        Ok((self, operator))
    }

    pub fn divide(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchDivision {
                location: self.location.unwrap(),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldDivision {
                location: self.location.unwrap(),
            });
        }

        let operator = GeneratorExpressionOperator::Division;

        Ok((self, operator))
    }

    pub fn remainder(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchRemainder {
                location: self.location.unwrap(),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldRemainder {
                location: self.location.unwrap(),
            });
        }

        let operator = GeneratorExpressionOperator::Remainder;

        Ok((self, operator))
    }

    pub fn cast(
        self,
        is_signed: bool,
        bitlength: usize,
    ) -> Result<(Self, Option<GeneratorExpressionOperator>), Error> {
        let operator = if self.is_signed != is_signed || self.bitlength != bitlength {
            GeneratorExpressionOperator::casting(&Type::scalar(self.location, is_signed, bitlength))
        } else {
            None
        };

        let result = Self {
            location: self.location,
            is_signed,
            bitlength,
            enumeration: None,
        };

        Ok((result, operator))
    }

    pub fn bitwise_not(self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if self.is_signed {
            return Err(Error::ForbiddenSignedBitwise {
                location: self.location.unwrap(),
            });
        }

        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldBitwise {
                location: self.location.unwrap(),
            });
        }

        let operator = GeneratorExpressionOperator::BitwiseNot;

        Ok((self, operator))
    }

    pub fn negate(mut self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldNegation {
                location: self.location.unwrap(),
            });
        }

        self.is_signed = true;

        let operator = GeneratorExpressionOperator::Negation;

        Ok((self, operator))
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<integer> of type '{}'", self.r#type())
    }
}
