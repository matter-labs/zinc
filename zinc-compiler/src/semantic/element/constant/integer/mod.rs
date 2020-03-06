//!
//! The semantic analyzer constant integer element.
//!

mod tests;

pub mod error;

use std::cmp;
use std::convert::TryFrom;
use std::fmt;

use num_bigint::BigInt;
use num_traits::Num;
use num_traits::One;
use num_traits::Signed;
use num_traits::ToPrimitive;
use num_traits::Zero;

use zinc_bytecode::scalar::IntegerType;
use zinc_bytecode::scalar::ScalarType;
use zinc_bytecode::Instruction;
use zinc_utils::euclidean;

use crate::lexical;
use crate::semantic::element::constant::Range;
use crate::semantic::element::constant::RangeInclusive;
use crate::semantic::element::r#type::enumeration::Enumeration;
use crate::semantic::element::r#type::Type;
use crate::syntax::IntegerLiteral;

use self::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Integer {
    pub value: BigInt,
    pub is_signed: bool,
    pub bitlength: usize,
    pub enumeration: Option<Enumeration>,
}

impl Integer {
    pub fn new(value: BigInt, is_signed: bool, bitlength: usize) -> Self {
        Self {
            value,
            is_signed,
            bitlength,
            enumeration: None,
        }
    }

    pub fn set_enumeration(&mut self, enumeration: Enumeration) {
        self.enumeration = Some(enumeration);
    }

    pub fn new_from_usize(value: usize, bitlength: usize) -> Self {
        Self {
            value: BigInt::from(value),
            is_signed: false,
            bitlength,
            enumeration: None,
        }
    }

    pub fn new_zero(is_signed: bool, bitlength: usize) -> Self {
        Self {
            value: BigInt::zero(),
            is_signed,
            bitlength,
            enumeration: None,
        }
    }

    pub fn new_one(is_signed: bool, bitlength: usize) -> Self {
        Self {
            value: BigInt::one(),
            is_signed,
            bitlength,
            enumeration: None,
        }
    }

    pub fn new_min(is_signed: bool, bitlength: usize) -> Self {
        let value = match (is_signed, bitlength) {
            (false, _bitlength) => BigInt::zero(),
            (true, bitlength) => -(BigInt::one() << (bitlength - 1)),
        };

        Self {
            value,
            is_signed,
            bitlength,
            enumeration: None,
        }
    }

    pub fn new_max(is_signed: bool, bitlength: usize) -> Self {
        let value = match (is_signed, bitlength) {
            (false, bitlength) => (BigInt::one() << bitlength) - BigInt::one(),
            (true, bitlength) => (BigInt::one() << (bitlength - 1)) - BigInt::one(),
        };

        Self {
            value,
            is_signed,
            bitlength,
            enumeration: None,
        }
    }

    pub fn to_bigint(&self) -> BigInt {
        self.value.to_owned()
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

    pub fn range_inclusive(&self, other: &Self) -> Result<RangeInclusive, Error> {
        let is_signed = self.is_signed || other.is_signed;
        let bitlength = cmp::max(
            cmp::max(self.bitlength, other.bitlength),
            Self::minimal_bitlength_bigints(&[&self.value, &other.value], is_signed)?,
        );
        Ok(RangeInclusive::new(
            self.value.to_owned(),
            other.value.to_owned(),
            is_signed,
            bitlength,
        ))
    }

    pub fn range(&self, other: &Self) -> Result<Range, Error> {
        let is_signed = self.is_signed || other.is_signed;
        let bitlength = cmp::max(
            cmp::max(self.bitlength, other.bitlength),
            Self::minimal_bitlength_bigints(&[&self.value, &other.value], is_signed)?,
        );
        Ok(Range::new(
            self.value.to_owned(),
            other.value.to_owned(),
            self.is_signed || other.is_signed,
            bitlength,
        ))
    }

    pub fn equals(&self, other: &Self) -> Result<bool, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchEquals(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        let result = self.value == other.value;
        Ok(result)
    }

    pub fn not_equals(&self, other: &Self) -> Result<bool, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchNotEquals(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        let result = self.value != other.value;
        Ok(result)
    }

    pub fn greater_equals(&self, other: &Self) -> Result<bool, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchGreaterEquals(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        let result = self.value >= other.value;
        Ok(result)
    }

    pub fn lesser_equals(&self, other: &Self) -> Result<bool, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchLesserEquals(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        let result = self.value <= other.value;
        Ok(result)
    }

    pub fn greater(&self, other: &Self) -> Result<bool, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchGreater(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        let result = self.value > other.value;
        Ok(result)
    }

    pub fn lesser(&self, other: &Self) -> Result<bool, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchLesser(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        let result = self.value < other.value;
        Ok(result)
    }

    pub fn add(&self, other: &Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchAddition(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        let result = self.value.to_owned() + other.value.to_owned();
        if result.is_negative() && !self.is_signed {
            return Err(Error::OverflowAddition {
                value: result,
                r#type: Type::integer(self.is_signed, self.bitlength).to_string(),
            });
        }

        if Self::minimal_bitlength(&result, self.is_signed)? > self.bitlength {
            return Err(Error::OverflowAddition {
                value: result,
                r#type: Type::integer(self.is_signed, self.bitlength).to_string(),
            });
        }

        Ok(Self {
            value: result,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: self.enumeration.to_owned(),
        })
    }

    pub fn subtract(&self, other: &Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchSubtraction(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        let result = self.value.to_owned() - other.value.to_owned();
        if result.is_negative() && !self.is_signed {
            return Err(Error::OverflowSubtraction {
                value: result,
                r#type: Type::integer(self.is_signed, self.bitlength).to_string(),
            });
        }

        if Self::minimal_bitlength(&result, self.is_signed)? > self.bitlength {
            return Err(Error::OverflowSubtraction {
                value: result,
                r#type: Type::integer(self.is_signed, self.bitlength).to_string(),
            });
        }

        Ok(Self {
            value: result,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: self.enumeration.to_owned(),
        })
    }

    pub fn multiply(&self, other: &Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchMultiplication(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        let result = self.value.to_owned() * other.value.to_owned();
        if result.is_negative() && !self.is_signed {
            return Err(Error::OverflowMultiplication {
                value: result,
                r#type: Type::integer(self.is_signed, self.bitlength).to_string(),
            });
        }

        if Self::minimal_bitlength(&result, self.is_signed)? > self.bitlength {
            return Err(Error::OverflowMultiplication {
                value: result,
                r#type: Type::integer(self.is_signed, self.bitlength).to_string(),
            });
        }

        Ok(Self {
            value: result,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: self.enumeration.to_owned(),
        })
    }

    pub fn divide(&self, other: &Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchDivision(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        let (result, _remainder) =
            euclidean::div_rem(&self.value, &other.value).ok_or(Error::ZeroDivision)?;
        if result.is_negative() && !self.is_signed {
            return Err(Error::OverflowDivision {
                value: result,
                r#type: Type::integer(self.is_signed, self.bitlength).to_string(),
            });
        }

        if Self::minimal_bitlength(&result, self.is_signed)? > self.bitlength {
            return Err(Error::OverflowDivision {
                value: result,
                r#type: Type::integer(self.is_signed, self.bitlength).to_string(),
            });
        }

        Ok(Self {
            value: result,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: self.enumeration.to_owned(),
        })
    }

    pub fn remainder(&self, other: &Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchRemainder(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldRemainder);
        }

        let (_quotient, result) =
            euclidean::div_rem(&self.value, &other.value).ok_or(Error::ZeroRemainder)?;
        if result.is_negative() && !self.is_signed {
            return Err(Error::OverflowRemainder {
                value: result,
                r#type: Type::integer(self.is_signed, self.bitlength).to_string(),
            });
        }

        if Self::minimal_bitlength(&result, self.is_signed)? > self.bitlength {
            return Err(Error::OverflowRemainder {
                value: result,
                r#type: Type::integer(self.is_signed, self.bitlength).to_string(),
            });
        }

        Ok(Self {
            value: result,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: self.enumeration.to_owned(),
        })
    }

    pub fn cast(&mut self, is_signed: bool, bitlength: usize) -> Result<(), Error> {
        if self.value.is_negative() && !is_signed {
            return Err(Error::OverflowCasting {
                value: self.value.to_owned(),
                r#type: Type::integer(is_signed, bitlength).to_string(),
            });
        }

        if Self::minimal_bitlength(&self.value, is_signed)? > bitlength {
            return Err(Error::OverflowCasting {
                value: self.value.to_owned(),
                r#type: Type::integer(is_signed, bitlength).to_string(),
            });
        }

        self.is_signed = is_signed;
        self.bitlength = bitlength;
        self.enumeration = None;
        Ok(())
    }

    pub fn negate(&self) -> Result<Self, Error> {
        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldNegation);
        }

        let is_signed = true;

        let result = -self.value.to_owned();
        if Self::minimal_bitlength(&result, is_signed)? > self.bitlength {
            return Err(Error::OverflowNegation {
                value: result,
                r#type: Type::integer(is_signed, self.bitlength).to_string(),
            });
        }

        Ok(Self {
            value: result,
            is_signed,
            bitlength: self.bitlength,
            enumeration: self.enumeration.to_owned(),
        })
    }

    pub fn to_usize(&self) -> Result<usize, Error> {
        self.value.to_usize().ok_or_else(|| Error::IntegerTooLarge {
            value: self.value.to_owned(),
            bitlength: crate::BITLENGTH_INDEX,
        })
    }

    pub fn minimal_bitlength_literals(literals: &[&IntegerLiteral]) -> Result<usize, Error> {
        let mut max = 0;
        for literal in literals.iter() {
            let bitlength = Self::try_from(*literal)?.bitlength;
            if bitlength > max {
                max = bitlength;
            }
        }
        Ok(max)
    }

    pub fn minimal_bitlength_bigints(values: &[&BigInt], is_signed: bool) -> Result<usize, Error> {
        let mut max = 0;
        for value in values.iter() {
            let bitlength = Self::minimal_bitlength(value, is_signed)?;
            if bitlength > max {
                max = bitlength;
            }
        }
        Ok(max)
    }

    ///
    /// Infers the minimal bitlength enough to represent the value.
    ///
    pub fn minimal_bitlength(value: &BigInt, is_signed: bool) -> Result<usize, Error> {
        let mut bitlength = crate::BITLENGTH_BYTE;
        let mut exponent = BigInt::from(1 << crate::BITLENGTH_BYTE);

        while if is_signed {
            if value.is_negative() {
                let bound = -(exponent.clone() / BigInt::from(2));
                value < &bound
            } else {
                let bound = exponent.clone() / BigInt::from(2);
                value >= &bound
            }
        } else {
            value >= &exponent
        } {
            if bitlength == crate::BITLENGTH_MAX_INT {
                exponent <<= crate::BITLENGTH_FIELD - crate::BITLENGTH_MAX_INT;
                bitlength += crate::BITLENGTH_FIELD - crate::BITLENGTH_MAX_INT;
            } else if bitlength == crate::BITLENGTH_FIELD {
                return Err(Error::IntegerTooLarge {
                    value: value.to_owned(),
                    bitlength: crate::BITLENGTH_FIELD,
                });
            } else {
                exponent <<= crate::BITLENGTH_BYTE;
                bitlength += crate::BITLENGTH_BYTE;
            }
        }

        if value.is_negative() && !is_signed {
            return Err(Error::UnsignedNegative {
                value: value.to_owned(),
                r#type: Type::integer(is_signed, bitlength).to_string(),
            });
        }

        Ok(bitlength)
    }

    pub fn to_instruction(&self) -> Instruction {
        let scalar_type = match (self.is_signed, self.bitlength) {
            (false, crate::BITLENGTH_FIELD) => ScalarType::Field,
            (signed, length) => IntegerType { signed, length }.into(),
        };
        Instruction::PushConst(zinc_bytecode::PushConst::new(
            self.value.to_owned(),
            scalar_type,
        ))
    }
}

impl TryFrom<&IntegerLiteral> for Integer {
    type Error = Error;

    ///
    /// Converts `literal` to a `BigInt` and its bitlength.
    /// For now, the minimal bitlength enough to contain the number is inferred.
    ///
    fn try_from(literal: &IntegerLiteral) -> Result<Self, Self::Error> {
        let (string, base) = match literal.data {
            lexical::IntegerLiteral::Decimal { ref value } => (value, crate::BASE_DECIMAL as u32),
            lexical::IntegerLiteral::Hexadecimal { ref value } => {
                (value, crate::BASE_HEXADECIMAL as u32)
            }
        };

        let value = BigInt::from_str_radix(&string, base)
            .expect(crate::semantic::PANIC_VALIDATED_DURING_LEXICAL_ANALYSIS);
        let bitlength = Self::minimal_bitlength(&value, false)?;

        Ok(Self::new(value, false, bitlength))
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "constant integer '{}' of type '{}'",
            self.value,
            self.r#type()
        )
    }
}
