//!
//! The semantic analyzer constant integer element.
//!

mod error;

pub use self::error::Error;

use std::convert::TryFrom;
use std::fmt;

use num_bigint::BigInt;
use num_traits::Num;
use num_traits::One;
use num_traits::ToPrimitive;
use num_traits::Zero;

use zinc_bytecode::Instruction;

use crate::lexical;
use crate::semantic::Type;
use crate::syntax::IntegerLiteral;

#[derive(Debug, Clone, PartialEq)]
pub struct Integer {
    pub value: BigInt,
    pub is_signed: bool,
    pub bitlength: usize,
}

impl Integer {
    pub fn new(value: BigInt, is_signed: bool, bitlength: usize) -> Self {
        Self {
            value,
            is_signed,
            bitlength,
        }
    }

    pub fn new_one(bitlength: usize) -> Self {
        Self {
            value: BigInt::one(),
            is_signed: false,
            bitlength,
        }
    }

    pub fn to_bigint(&self) -> BigInt {
        self.value.to_owned()
    }

    pub fn r#type(&self) -> Type {
        Type::new_numeric(self.is_signed, self.bitlength)
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.is_signed == other.is_signed && self.bitlength == other.bitlength
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
        Ok(Self {
            value: result,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
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
        Ok(Self {
            value: result,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
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
        Ok(Self {
            value: result,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
        })
    }

    pub fn divide(&self, other: &Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchDivision(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        if other.value.is_zero() {
            return Err(Error::DivisionZero);
        }

        let result = self.value.to_owned() / other.value.to_owned();
        Ok(Self {
            value: result,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
        })
    }

    pub fn remainder(&self, other: &Self) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchRemainder(
                self.r#type().to_string(),
                other.r#type().to_string(),
            ));
        }

        if other.value.is_zero() {
            return Err(Error::RemainderZero);
        }

        let result = self.value.to_owned() % other.value.to_owned();
        Ok(Self {
            value: result,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
        })
    }

    pub fn cast(&mut self, is_signed: bool, bitlength: usize) {
        self.is_signed = is_signed;
        self.bitlength = bitlength;
    }

    pub fn negate(&self) -> Result<Self, Error> {
        if self.bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::NegationBitlengthTooBig(self.bitlength));
        }

        let result = -self.value.to_owned();
        Ok(Self {
            value: result,
            is_signed: true,
            bitlength: self.bitlength,
        })
    }

    pub fn to_usize(&self) -> Result<usize, Error> {
        self.value.to_usize().ok_or_else(|| {
            Error::LiteralTooLargeForIndex(self.value.to_string(), crate::BITLENGTH_BYTE)
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

    pub fn minimal_bitlength_bigints(values: &[&BigInt]) -> Result<usize, Error> {
        let mut max = 0;
        for value in values.iter() {
            let bitlength = Self::minimal_bitlength(value)?;
            if bitlength > max {
                max = bitlength;
            }
        }
        Ok(max)
    }

    fn minimal_bitlength(value: &BigInt) -> Result<usize, Error> {
        let mut bitlength = crate::BITLENGTH_BYTE;
        let mut exponent = BigInt::from(1 << crate::BITLENGTH_BYTE);
        while value.ge(&exponent) {
            if bitlength == crate::BITLENGTH_MAX_INT {
                exponent <<= crate::BITLENGTH_FIELD - crate::BITLENGTH_MAX_INT;
                bitlength += crate::BITLENGTH_FIELD - crate::BITLENGTH_MAX_INT;
            } else if bitlength == crate::BITLENGTH_FIELD {
                return Err(Error::IntegerTooLargeForField(
                    value.to_string(),
                    crate::BITLENGTH_FIELD,
                ));
            } else {
                exponent <<= crate::BITLENGTH_BYTE;
                bitlength += crate::BITLENGTH_BYTE;
            }
        }
        Ok(bitlength)
    }

    pub fn to_instruction(&self) -> Instruction {
        Instruction::PushConst(zinc_bytecode::PushConst::new(
            self.value.to_owned(),
            self.is_signed,
            self.bitlength,
        ))
    }
}

impl From<(usize, usize)> for Integer {
    fn from((value, bitlength): (usize, usize)) -> Self {
        Self {
            value: BigInt::from(value),
            is_signed: false,
            bitlength,
        }
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
        let bitlength = Self::minimal_bitlength(&value)?;

        Ok(Self::new(value, false, bitlength))
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.value, self.r#type())
    }
}
