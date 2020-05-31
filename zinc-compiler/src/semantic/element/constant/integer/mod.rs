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
use num_traits::Signed;
use num_traits::ToPrimitive;

use zinc_utils::math;

use crate::generator::expression::operator::Operator as GeneratorExpressionOperator;
use crate::lexical::token::lexeme::literal::integer::Integer as LexicalIntegerLiteral;
use crate::lexical::token::location::Location;
use crate::semantic::element::constant::boolean::Boolean as BooleanConstant;
use crate::semantic::element::constant::range::Range;
use crate::semantic::element::constant::range_inclusive::RangeInclusive;
use crate::semantic::element::r#type::enumeration::Enumeration;
use crate::semantic::element::r#type::Type;
use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;

use self::error::Error;

///
/// Integer constants consist of the value, sign, and bitlength.
/// If a constant belongs to an enumeration, the enumeration type is stored in `enumeration`.
/// Enumeration uniquely defines the constant type, even if the sign and bitlength are the same.
///
#[derive(Debug, Clone)]
pub struct Integer {
    pub location: Location,
    pub value: BigInt,
    pub is_signed: bool,
    pub bitlength: usize,
    pub enumeration: Option<Enumeration>,
}

impl Integer {
    pub fn new(location: Location, value: BigInt, is_signed: bool, bitlength: usize) -> Self {
        Self {
            location,
            value,
            is_signed,
            bitlength,
            enumeration: None,
        }
    }

    pub fn set_enumeration(&mut self, enumeration: Enumeration) {
        self.enumeration = Some(enumeration);
    }

    pub fn to_bigint(&self) -> BigInt {
        self.value.to_owned()
    }

    pub fn r#type(&self) -> Type {
        match self.enumeration {
            Some(ref enumeration) => Type::Enumeration(enumeration.to_owned()),
            None => Type::scalar(Some(self.location), self.is_signed, self.bitlength),
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

    pub fn range_inclusive(self, other: Self) -> Result<RangeInclusive, Error> {
        let is_signed = self.is_signed || other.is_signed;
        let bitlength = cmp::max(
            cmp::max(self.bitlength, other.bitlength),
            Self::minimal_bitlength_bigints(
                &[&self.value, &other.value],
                is_signed,
                self.location,
            )?,
        );

        Ok(RangeInclusive::new(
            self.location,
            self.value,
            other.value,
            is_signed,
            bitlength,
        ))
    }

    pub fn range(self, other: Self) -> Result<Range, Error> {
        let is_signed = self.is_signed || other.is_signed;
        let bitlength = cmp::max(
            cmp::max(self.bitlength, other.bitlength),
            Self::minimal_bitlength_bigints(
                &[&self.value, &other.value],
                is_signed,
                self.location,
            )?,
        );

        Ok(Range::new(
            self.location,
            self.value,
            other.value,
            self.is_signed || other.is_signed,
            bitlength,
        ))
    }

    pub fn equals(
        self,
        other: Self,
    ) -> Result<(BooleanConstant, GeneratorExpressionOperator), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchEquals {
                location: self.location,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let result = BooleanConstant::new(self.location, self.value == other.value);

        let operator = GeneratorExpressionOperator::Equals;

        Ok((result, operator))
    }

    pub fn not_equals(
        self,
        other: Self,
    ) -> Result<(BooleanConstant, GeneratorExpressionOperator), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchNotEquals {
                location: self.location,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let result = BooleanConstant::new(self.location, self.value != other.value);

        let operator = GeneratorExpressionOperator::NotEquals;

        Ok((result, operator))
    }

    pub fn greater_equals(
        self,
        other: Self,
    ) -> Result<(BooleanConstant, GeneratorExpressionOperator), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchGreaterEquals {
                location: self.location,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let result = BooleanConstant::new(self.location, self.value >= other.value);

        let operator = GeneratorExpressionOperator::GreaterEquals;

        Ok((result, operator))
    }

    pub fn lesser_equals(
        self,
        other: Self,
    ) -> Result<(BooleanConstant, GeneratorExpressionOperator), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchLesserEquals {
                location: self.location,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let result = BooleanConstant::new(self.location, self.value <= other.value);

        let operator = GeneratorExpressionOperator::LesserEquals;

        Ok((result, operator))
    }

    pub fn greater(
        self,
        other: Self,
    ) -> Result<(BooleanConstant, GeneratorExpressionOperator), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchGreater {
                location: self.location,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let result = BooleanConstant::new(self.location, self.value > other.value);

        let operator = GeneratorExpressionOperator::Greater;

        Ok((result, operator))
    }

    pub fn lesser(
        self,
        other: Self,
    ) -> Result<(BooleanConstant, GeneratorExpressionOperator), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchLesser {
                location: self.location,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let result = BooleanConstant::new(self.location, self.value < other.value);

        let operator = GeneratorExpressionOperator::Lesser;

        Ok((result, operator))
    }

    pub fn bitwise_or(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchBitwiseOr {
                location: self.location,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        if self.is_signed {
            return Err(Error::ForbiddenSignedBitwise {
                location: self.location,
            });
        }

        if self.bitlength == zinc_const::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldBitwise {
                location: self.location,
            });
        }

        let result = Self {
            location: self.location,
            value: self.value | &other.value,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: self.enumeration,
        };

        let operator = GeneratorExpressionOperator::BitwiseOr;

        Ok((result, operator))
    }

    pub fn bitwise_xor(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchBitwiseXor {
                location: self.location,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        if self.is_signed {
            return Err(Error::ForbiddenSignedBitwise {
                location: self.location,
            });
        }

        if self.bitlength == zinc_const::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldBitwise {
                location: self.location,
            });
        }

        let result = Self {
            location: self.location,
            value: self.value ^ &other.value,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: self.enumeration,
        };

        let operator = GeneratorExpressionOperator::BitwiseXor;

        Ok((result, operator))
    }

    pub fn bitwise_and(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchBitwiseAnd {
                location: self.location,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        if self.is_signed {
            return Err(Error::ForbiddenSignedBitwise {
                location: self.location,
            });
        }

        if self.bitlength == zinc_const::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldBitwise {
                location: self.location,
            });
        }

        let result = Self {
            location: self.location,
            value: self.value & &other.value,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: self.enumeration,
        };

        let operator = GeneratorExpressionOperator::BitwiseAnd;

        Ok((result, operator))
    }

    pub fn bitwise_shift_left(
        self,
        other: Self,
    ) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if self.is_signed {
            return Err(Error::ForbiddenSignedBitwise {
                location: self.location,
            });
        }

        if self.bitlength == zinc_const::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldBitwise {
                location: self.location,
            });
        }

        if other.is_signed {
            return Err(
                Error::OperatorBitwiseShiftLeftSecondOperatorExpectedUnsigned {
                    location: other.location,
                    found: other.to_string(),
                },
            );
        }

        let other = other
            .value
            .to_usize()
            .ok_or_else(|| Error::IntegerTooLarge {
                location: other.location,
                value: other.value,
                bitlength: self.bitlength,
            })?;

        let result = Self {
            location: self.location,
            value: self.value << other,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: self.enumeration,
        };

        let operator = GeneratorExpressionOperator::BitwiseShiftLeft;

        Ok((result, operator))
    }

    pub fn bitwise_shift_right(
        self,
        other: Self,
    ) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if self.is_signed {
            return Err(Error::ForbiddenSignedBitwise {
                location: self.location,
            });
        }

        if self.bitlength == zinc_const::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldBitwise {
                location: self.location,
            });
        }

        if other.is_signed {
            return Err(
                Error::OperatorBitwiseShiftRightSecondOperatorExpectedUnsigned {
                    location: other.location,
                    found: other.to_string(),
                },
            );
        }

        let other = other
            .value
            .to_usize()
            .ok_or_else(|| Error::IntegerTooLarge {
                location: other.location,
                value: other.value,
                bitlength: self.bitlength,
            })?;

        let result = Self {
            location: self.location,
            value: self.value >> other,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: self.enumeration,
        };

        let operator = GeneratorExpressionOperator::BitwiseShiftRight;

        Ok((result, operator))
    }

    pub fn add(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchAddition {
                location: self.location,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let result = self.value + other.value;
        if result.is_negative() && !self.is_signed {
            return Err(Error::OverflowAddition {
                location: self.location,
                value: result,
                r#type: Type::integer(Some(self.location), self.is_signed, self.bitlength)
                    .to_string(),
            });
        }

        if Self::minimal_bitlength(&result, self.is_signed, self.location)? > self.bitlength {
            return Err(Error::OverflowAddition {
                location: self.location,
                value: result,
                r#type: Type::integer(Some(self.location), self.is_signed, self.bitlength)
                    .to_string(),
            });
        }

        let result = Self {
            location: self.location,
            value: result,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: self.enumeration,
        };

        let operator = GeneratorExpressionOperator::Addition;

        Ok((result, operator))
    }

    pub fn subtract(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchSubtraction {
                location: self.location,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let result = self.value - other.value;
        if result.is_negative() && !self.is_signed {
            return Err(Error::OverflowSubtraction {
                location: self.location,
                value: result,
                r#type: Type::integer(Some(self.location), self.is_signed, self.bitlength)
                    .to_string(),
            });
        }

        if Self::minimal_bitlength(&result, self.is_signed, self.location)? > self.bitlength {
            return Err(Error::OverflowSubtraction {
                location: self.location,
                value: result,
                r#type: Type::integer(Some(self.location), self.is_signed, self.bitlength)
                    .to_string(),
            });
        }

        let result = Self {
            location: self.location,
            value: result,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: self.enumeration,
        };

        let operator = GeneratorExpressionOperator::Subtraction;

        Ok((result, operator))
    }

    pub fn multiply(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchMultiplication {
                location: self.location,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let result = self.value * other.value;
        if result.is_negative() && !self.is_signed {
            return Err(Error::OverflowMultiplication {
                location: self.location,
                value: result,
                r#type: Type::integer(Some(self.location), self.is_signed, self.bitlength)
                    .to_string(),
            });
        }

        if Self::minimal_bitlength(&result, self.is_signed, self.location)? > self.bitlength {
            return Err(Error::OverflowMultiplication {
                location: self.location,
                value: result,
                r#type: Type::integer(Some(self.location), self.is_signed, self.bitlength)
                    .to_string(),
            });
        }

        let result = Self {
            location: self.location,
            value: result,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: self.enumeration,
        };

        let operator = GeneratorExpressionOperator::Multiplication;

        Ok((result, operator))
    }

    pub fn divide(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchDivision {
                location: self.location,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        if self.bitlength == zinc_const::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldDivision {
                location: self.location,
            });
        }

        let (result, _remainder) =
            math::euclidean::div_rem(&self.value, &other.value).ok_or(Error::ZeroDivision {
                location: other.location,
            })?;
        if result.is_negative() && !self.is_signed {
            return Err(Error::OverflowDivision {
                location: self.location,
                value: result,
                r#type: Type::integer(Some(self.location), self.is_signed, self.bitlength)
                    .to_string(),
            });
        }

        if Self::minimal_bitlength(&result, self.is_signed, self.location)? > self.bitlength {
            return Err(Error::OverflowDivision {
                location: self.location,
                value: result,
                r#type: Type::integer(Some(self.location), self.is_signed, self.bitlength)
                    .to_string(),
            });
        }

        let result = Self {
            location: self.location,
            value: result,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: self.enumeration,
        };

        let operator = GeneratorExpressionOperator::Division;

        Ok((result, operator))
    }

    pub fn remainder(self, other: Self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchRemainder {
                location: self.location,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        if self.bitlength == zinc_const::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldRemainder {
                location: self.location,
            });
        }

        let (_quotient, result) =
            math::euclidean::div_rem(&self.value, &other.value).ok_or(Error::ZeroRemainder {
                location: other.location,
            })?;
        if result.is_negative() && !self.is_signed {
            return Err(Error::OverflowRemainder {
                location: self.location,
                value: result,
                r#type: Type::integer(Some(self.location), self.is_signed, self.bitlength)
                    .to_string(),
            });
        }

        if Self::minimal_bitlength(&result, self.is_signed, self.location)? > self.bitlength {
            return Err(Error::OverflowRemainder {
                location: self.location,
                value: result,
                r#type: Type::integer(Some(self.location), self.is_signed, self.bitlength)
                    .to_string(),
            });
        }

        let result = Self {
            location: self.location,
            value: result,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: self.enumeration,
        };

        let operator = GeneratorExpressionOperator::Remainder;

        Ok((result, operator))
    }

    pub fn cast(
        self,
        is_signed: bool,
        bitlength: usize,
    ) -> Result<(Self, Option<GeneratorExpressionOperator>), Error> {
        if self.value.is_negative() && !is_signed {
            return Err(Error::OverflowCasting {
                location: self.location,
                value: self.value,
                r#type: Type::integer(Some(self.location), is_signed, bitlength).to_string(),
            });
        }

        if Self::minimal_bitlength(&self.value, is_signed, self.location)? > bitlength {
            return Err(Error::OverflowCasting {
                location: self.location,
                value: self.value,
                r#type: Type::integer(Some(self.location), is_signed, bitlength).to_string(),
            });
        }

        let operator = if self.is_signed != is_signed || self.bitlength != bitlength {
            GeneratorExpressionOperator::casting(&Type::scalar(
                Some(self.location),
                is_signed,
                bitlength,
            ))
        } else {
            None
        };

        let result = Self {
            location: self.location,
            value: self.value,
            is_signed,
            bitlength,
            enumeration: None,
        };

        Ok((result, operator))
    }

    pub fn bitwise_not(self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if self.is_signed {
            return Err(Error::ForbiddenSignedBitwise {
                location: self.location,
            });
        }

        if self.bitlength == zinc_const::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldBitwise {
                location: self.location,
            });
        }

        let result = Self {
            location: self.location,
            value: !self.value,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: self.enumeration,
        };

        let operator = GeneratorExpressionOperator::BitwiseNot;

        Ok((result, operator))
    }

    pub fn negate(self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if self.bitlength == zinc_const::BITLENGTH_FIELD {
            return Err(Error::ForbiddenFieldNegation {
                location: self.location,
            });
        }

        let is_signed = true;

        let result = -self.value;
        if Self::minimal_bitlength(&result, is_signed, self.location)? > self.bitlength {
            return Err(Error::OverflowNegation {
                location: self.location,
                value: result,
                r#type: Type::integer(Some(self.location), is_signed, self.bitlength).to_string(),
            });
        }

        let result = Self {
            location: self.location,
            value: result,
            is_signed,
            bitlength: self.bitlength,
            enumeration: self.enumeration,
        };

        let operator = GeneratorExpressionOperator::Negation;

        Ok((result, operator))
    }

    pub fn to_usize(&self) -> Result<usize, Error> {
        self.value.to_usize().ok_or_else(|| Error::IntegerTooLarge {
            location: self.location,
            value: self.value.to_owned(),
            bitlength: zinc_const::BITLENGTH_INDEX,
        })
    }

    ///
    /// Calculates the minimal bitlength required to represent each element of `literals`.
    ///
    pub fn minimal_bitlength_literals(literals: &[&IntegerLiteral]) -> Result<usize, Error> {
        let mut result = zinc_const::BITLENGTH_BYTE;

        for literal in literals.iter() {
            let bitlength = Self::try_from(*literal)?.bitlength;
            if bitlength > result {
                result = bitlength;
            }
        }

        Ok(result)
    }

    ///
    /// Calculates the minimal bitlength required to represent each element of `values`
    /// with sign specified as `is_signed`.
    ///
    pub fn minimal_bitlength_bigints(
        values: &[&BigInt],
        is_signed: bool,
        location: Location,
    ) -> Result<usize, Error> {
        let mut result = zinc_const::BITLENGTH_BYTE;

        for value in values.iter() {
            let bitlength = Self::minimal_bitlength(value, is_signed, location)?;
            if bitlength > result {
                result = bitlength;
            }
        }

        Ok(result)
    }

    ///
    /// Infers the minimal bitlength enough to represent the `value` with sign specified
    /// as `is_signed`.
    ///
    pub fn minimal_bitlength(
        value: &BigInt,
        is_signed: bool,
        location: Location,
    ) -> Result<usize, Error> {
        let mut bitlength = zinc_const::BITLENGTH_BYTE;
        let mut exponent = BigInt::from(1 << zinc_const::BITLENGTH_BYTE);

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
            if bitlength == zinc_const::BITLENGTH_INTEGER_MAX {
                exponent <<= zinc_const::BITLENGTH_FIELD - zinc_const::BITLENGTH_INTEGER_MAX;
                bitlength += zinc_const::BITLENGTH_FIELD - zinc_const::BITLENGTH_INTEGER_MAX;
            } else if bitlength == zinc_const::BITLENGTH_FIELD {
                return Err(Error::IntegerTooLarge {
                    location,
                    value: value.to_owned(),
                    bitlength: zinc_const::BITLENGTH_FIELD,
                });
            } else {
                exponent <<= zinc_const::BITLENGTH_BYTE;
                bitlength += zinc_const::BITLENGTH_BYTE;
            }
        }

        if value.is_negative() && !is_signed {
            panic!(crate::panic::VALIDATED_DURING_LEXICAL_ANALYSIS);
        }

        Ok(bitlength)
    }
}

impl TryFrom<&IntegerLiteral> for Integer {
    type Error = Error;

    ///
    /// Converts `literal` to a `BigInt` and its bitlength.
    /// For now, the minimal bitlength enough to contain the number is inferred.
    ///
    fn try_from(literal: &IntegerLiteral) -> Result<Self, Self::Error> {
        let (string, base) = match literal.inner {
            LexicalIntegerLiteral::Binary { ref inner } => (inner, zinc_const::BASE_BINARY as u32),
            LexicalIntegerLiteral::Octal { ref inner } => (inner, zinc_const::BASE_OCTAL as u32),
            LexicalIntegerLiteral::Decimal { ref inner } => {
                (inner, zinc_const::BASE_DECIMAL as u32)
            }
            LexicalIntegerLiteral::Hexadecimal { ref inner } => {
                (inner, zinc_const::BASE_HEXADECIMAL as u32)
            }
        };

        let value = BigInt::from_str_radix(&string, base)
            .expect(crate::panic::VALIDATED_DURING_LEXICAL_ANALYSIS);
        let bitlength = Self::minimal_bitlength(&value, false, literal.location)?;

        Ok(Self::new(literal.location, value, false, bitlength))
    }
}

impl PartialEq<Self> for Integer {
    fn eq(&self, other: &Self) -> bool {
        let are_enum_types_equal = match (&self.enumeration, &other.enumeration) {
            (Some(enum_1), Some(enum_2)) => enum_1.type_id == enum_2.type_id,
            (None, None) => true,
            (_, _) => false,
        };

        self.value == other.value
            && self.is_signed == other.is_signed
            && self.bitlength == other.bitlength
            && are_enum_types_equal
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'{}' of type '{}'", self.value, self.r#type())
    }
}
