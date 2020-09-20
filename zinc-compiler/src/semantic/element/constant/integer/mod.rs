//!
//! The semantic analyzer constant integer element.
//!

mod tests;

pub mod error;

use std::cmp;
use std::convert::TryFrom;
use std::fmt;
use std::ops::Add;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitXor;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Rem;
use std::ops::Shl;
use std::ops::Shr;
use std::ops::Sub;

use num_bigint::BigInt;
use num_traits::Num;
use num_traits::Signed;
use num_traits::ToPrimitive;

use zinc_utils::InferenceError;

use crate::generator::expression::operator::Operator as GeneratorExpressionOperator;
use crate::lexical::token::lexeme::literal::integer::Integer as LexicalIntegerLiteral;
use crate::lexical::token::location::Location;
use crate::semantic::element::constant::boolean::Boolean as BooleanConstant;
use crate::semantic::element::constant::range::Range;
use crate::semantic::element::constant::range_inclusive::RangeInclusive;
use crate::semantic::element::r#type::enumeration::Enumeration;
use crate::semantic::element::r#type::i_typed::ITyped;
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
    /// The location where the value appears in the code.
    pub location: Location,
    /// The inner constant value.
    pub value: BigInt,
    /// The integer type sign.
    pub is_signed: bool,
    /// The integer type bitlength.
    pub bitlength: usize,
    /// If the constant is an enumeration variant.
    pub enumeration: Option<Enumeration>,
    /// If the constant was created from an integer literal.
    pub is_literal: bool,
}

impl Integer {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        location: Location,
        value: BigInt,
        is_signed: bool,
        bitlength: usize,
        is_literal: bool,
    ) -> Self {
        Self {
            location,
            value,
            is_signed,
            bitlength,
            enumeration: None,
            is_literal,
        }
    }

    ///
    /// Set the enumeration type for the constant, if the constant is an enumeration variant.
    ///
    pub fn set_enumeration(&mut self, enumeration: Enumeration) {
        self.enumeration = Some(enumeration);
    }

    ///
    /// Returns the inner `BigInt` value.
    ///
    pub fn to_bigint(&self) -> BigInt {
        self.value.to_owned()
    }

    ///
    /// Executes the `..=` range inclusive operator.
    ///
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

    ///
    /// Executes the `..` range operator.
    ///
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

    ///
    /// Executes the `==` equals comparison operator.
    ///
    pub fn equals(
        mut self,
        mut other: Self,
    ) -> Result<(BooleanConstant, GeneratorExpressionOperator), Error> {
        let inference_result = zinc_utils::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchEquals {
                location: self.location,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let result = BooleanConstant::new(self.location, self.value == other.value);

        let operator = GeneratorExpressionOperator::equals_inferred(
            inference_result.first.map(|r#type| {
                Type::scalar(Some(self.location), r#type.is_signed, r#type.bitlength)
            }),
            inference_result.second.map(|r#type| {
                Type::scalar(Some(other.location), r#type.is_signed, r#type.bitlength)
            }),
        );

        Ok((result, operator))
    }

    ///
    /// Executes the `!=` not-equals comparison operator.
    ///
    pub fn not_equals(
        mut self,
        mut other: Self,
    ) -> Result<(BooleanConstant, GeneratorExpressionOperator), Error> {
        let inference_result = zinc_utils::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchNotEquals {
                location: self.location,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let result = BooleanConstant::new(self.location, self.value != other.value);

        let operator = GeneratorExpressionOperator::not_equals_inferred(
            inference_result.first.map(|r#type| {
                Type::scalar(Some(self.location), r#type.is_signed, r#type.bitlength)
            }),
            inference_result.second.map(|r#type| {
                Type::scalar(Some(other.location), r#type.is_signed, r#type.bitlength)
            }),
        );

        Ok((result, operator))
    }

    ///
    /// Executes the `>=` greater-equals comparison operator.
    ///
    pub fn greater_equals(
        mut self,
        mut other: Self,
    ) -> Result<(BooleanConstant, GeneratorExpressionOperator), Error> {
        let inference_result = zinc_utils::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchGreaterEquals {
                location: self.location,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let result = BooleanConstant::new(self.location, self.value >= other.value);

        let operator = GeneratorExpressionOperator::greater_equals_inferred(
            inference_result.first.map(|r#type| {
                Type::scalar(Some(self.location), r#type.is_signed, r#type.bitlength)
            }),
            inference_result.second.map(|r#type| {
                Type::scalar(Some(other.location), r#type.is_signed, r#type.bitlength)
            }),
        );

        Ok((result, operator))
    }

    ///
    /// Executes the `<=` lesser-equals comparison operator.
    ///
    pub fn lesser_equals(
        mut self,
        mut other: Self,
    ) -> Result<(BooleanConstant, GeneratorExpressionOperator), Error> {
        let inference_result = zinc_utils::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchLesserEquals {
                location: self.location,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let result = BooleanConstant::new(self.location, self.value <= other.value);

        let operator = GeneratorExpressionOperator::lesser_equals_inferred(
            inference_result.first.map(|r#type| {
                Type::scalar(Some(self.location), r#type.is_signed, r#type.bitlength)
            }),
            inference_result.second.map(|r#type| {
                Type::scalar(Some(other.location), r#type.is_signed, r#type.bitlength)
            }),
        );

        Ok((result, operator))
    }

    ///
    /// Executes the `>` greater comparison operator.
    ///
    pub fn greater(
        mut self,
        mut other: Self,
    ) -> Result<(BooleanConstant, GeneratorExpressionOperator), Error> {
        let inference_result = zinc_utils::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchGreater {
                location: self.location,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let result = BooleanConstant::new(self.location, self.value > other.value);

        let operator = GeneratorExpressionOperator::greater_inferred(
            inference_result.first.map(|r#type| {
                Type::scalar(Some(self.location), r#type.is_signed, r#type.bitlength)
            }),
            inference_result.second.map(|r#type| {
                Type::scalar(Some(other.location), r#type.is_signed, r#type.bitlength)
            }),
        );

        Ok((result, operator))
    }

    ///
    /// Executes the `<` lesser comparison operator.
    ///
    pub fn lesser(
        mut self,
        mut other: Self,
    ) -> Result<(BooleanConstant, GeneratorExpressionOperator), Error> {
        let inference_result = zinc_utils::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchLesser {
                location: self.location,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let result = BooleanConstant::new(self.location, self.value < other.value);

        let operator = GeneratorExpressionOperator::lesser_inferred(
            inference_result.first.map(|r#type| {
                Type::scalar(Some(self.location), r#type.is_signed, r#type.bitlength)
            }),
            inference_result.second.map(|r#type| {
                Type::scalar(Some(other.location), r#type.is_signed, r#type.bitlength)
            }),
        );

        Ok((result, operator))
    }
}

impl BitOr for Integer {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn bitor(mut self, mut other: Self) -> Self::Output {
        let location_1 = self.location;
        let location_2 = other.location;

        let inference_result = zinc_utils::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

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

        if self.bitlength == zinc_const::bitlength::FIELD {
            return Err(Error::ForbiddenFieldBitwise {
                location: self.location,
            });
        }

        let result = Self {
            location: self.location,
            value: self.value | &other.value,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: None,
            is_literal: self.is_literal && other.is_literal,
        };

        let operator = GeneratorExpressionOperator::bitwise_or_inferred(
            inference_result
                .first
                .map(|r#type| Type::scalar(Some(location_1), r#type.is_signed, r#type.bitlength)),
            inference_result
                .second
                .map(|r#type| Type::scalar(Some(location_2), r#type.is_signed, r#type.bitlength)),
        );

        Ok((result, operator))
    }
}

impl BitXor for Integer {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn bitxor(mut self, mut other: Self) -> Self::Output {
        let location_1 = self.location;
        let location_2 = other.location;

        let inference_result = zinc_utils::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

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

        if self.bitlength == zinc_const::bitlength::FIELD {
            return Err(Error::ForbiddenFieldBitwise {
                location: self.location,
            });
        }

        let result = Self {
            location: self.location,
            value: self.value ^ &other.value,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: None,
            is_literal: self.is_literal && other.is_literal,
        };

        let operator = GeneratorExpressionOperator::bitwise_xor_inferred(
            inference_result
                .first
                .map(|r#type| Type::scalar(Some(location_1), r#type.is_signed, r#type.bitlength)),
            inference_result
                .second
                .map(|r#type| Type::scalar(Some(location_2), r#type.is_signed, r#type.bitlength)),
        );

        Ok((result, operator))
    }
}

impl BitAnd for Integer {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn bitand(mut self, mut other: Self) -> Self::Output {
        let location_1 = self.location;
        let location_2 = other.location;

        let inference_result = zinc_utils::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

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

        if self.bitlength == zinc_const::bitlength::FIELD {
            return Err(Error::ForbiddenFieldBitwise {
                location: self.location,
            });
        }

        let result = Self {
            location: self.location,
            value: self.value & &other.value,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: None,
            is_literal: self.is_literal && other.is_literal,
        };

        let operator = GeneratorExpressionOperator::bitwise_and_inferred(
            inference_result
                .first
                .map(|r#type| Type::scalar(Some(location_1), r#type.is_signed, r#type.bitlength)),
            inference_result
                .second
                .map(|r#type| Type::scalar(Some(location_2), r#type.is_signed, r#type.bitlength)),
        );

        Ok((result, operator))
    }
}

impl Shl<Self> for Integer {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn shl(self, other: Self) -> Self::Output {
        if self.is_signed {
            return Err(Error::ForbiddenSignedBitwise {
                location: self.location,
            });
        }

        if self.bitlength == zinc_const::bitlength::FIELD {
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
                inner: InferenceError::Overflow {
                    value: other.value,
                    is_signed: other.is_signed,
                    bitlength: other.bitlength,
                },
            })?;

        let result = Self {
            location: self.location,
            value: self.value << other,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: None,
            is_literal: self.is_literal,
        };

        let operator = GeneratorExpressionOperator::BitwiseShiftLeft;

        Ok((result, operator))
    }
}

impl Shr<Self> for Integer {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn shr(self, other: Self) -> Self::Output {
        if self.is_signed {
            return Err(Error::ForbiddenSignedBitwise {
                location: self.location,
            });
        }

        if self.bitlength == zinc_const::bitlength::FIELD {
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
                inner: InferenceError::Overflow {
                    value: other.value,
                    is_signed: other.is_signed,
                    bitlength: other.bitlength,
                },
            })?;

        let result = Self {
            location: self.location,
            value: self.value >> other,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: None,
            is_literal: self.is_literal,
        };

        let operator = GeneratorExpressionOperator::BitwiseShiftRight;

        Ok((result, operator))
    }
}

impl Add for Integer {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn add(mut self, mut other: Self) -> Self::Output {
        let location_1 = self.location;
        let location_2 = other.location;

        let inference_result = zinc_utils::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchAddition {
                location: location_1,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let result = self.value + other.value;
        if result.is_negative() && !self.is_signed {
            return Err(Error::OverflowAddition {
                location: location_1,
                value: result,
                r#type: Type::scalar(Some(self.location), self.is_signed, self.bitlength)
                    .to_string(),
            });
        }

        let bitlength =
            zinc_utils::infer_minimal_bitlength(&result, self.is_signed).map_err(|error| {
                Error::IntegerTooLarge {
                    location: location_1,
                    inner: error,
                }
            })?;
        if bitlength > self.bitlength {
            return Err(Error::OverflowAddition {
                location: location_1,
                value: result,
                r#type: Type::scalar(Some(self.location), self.is_signed, self.bitlength)
                    .to_string(),
            });
        }

        #[allow(clippy::suspicious_arithmetic_impl)]
        let is_literal = self.is_literal && other.is_literal;
        let result = Self {
            location: location_1,
            value: result,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: None,
            is_literal,
        };

        let operator = GeneratorExpressionOperator::addition_inferred(
            inference_result
                .first
                .map(|r#type| Type::scalar(Some(location_1), r#type.is_signed, r#type.bitlength)),
            inference_result
                .second
                .map(|r#type| Type::scalar(Some(location_2), r#type.is_signed, r#type.bitlength)),
        );

        Ok((result, operator))
    }
}

impl Sub for Integer {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn sub(mut self, mut other: Self) -> Self::Output {
        let location_1 = self.location;
        let location_2 = other.location;

        let inference_result = zinc_utils::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchSubtraction {
                location: location_1,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let result = self.value - other.value;
        if result.is_negative() && !self.is_signed {
            return Err(Error::OverflowSubtraction {
                location: location_1,
                value: result,
                r#type: Type::scalar(Some(self.location), self.is_signed, self.bitlength)
                    .to_string(),
            });
        }

        let bitlength =
            zinc_utils::infer_minimal_bitlength(&result, self.is_signed).map_err(|error| {
                Error::IntegerTooLarge {
                    location: location_1,
                    inner: error,
                }
            })?;
        if bitlength > self.bitlength {
            return Err(Error::OverflowSubtraction {
                location: location_1,
                value: result,
                r#type: Type::scalar(Some(self.location), self.is_signed, self.bitlength)
                    .to_string(),
            });
        }

        #[allow(clippy::suspicious_arithmetic_impl)]
        let is_literal = self.is_literal && other.is_literal;
        let result = Self {
            location: location_1,
            value: result,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: None,
            is_literal,
        };

        let operator = GeneratorExpressionOperator::subtraction_inferred(
            inference_result
                .first
                .map(|r#type| Type::scalar(Some(location_1), r#type.is_signed, r#type.bitlength)),
            inference_result
                .second
                .map(|r#type| Type::scalar(Some(location_2), r#type.is_signed, r#type.bitlength)),
        );

        Ok((result, operator))
    }
}

impl Mul for Integer {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn mul(mut self, mut other: Self) -> Self::Output {
        let location_1 = self.location;
        let location_2 = other.location;

        let inference_result = zinc_utils::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchMultiplication {
                location: location_1,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let result = self.value * other.value;
        if result.is_negative() && !self.is_signed {
            return Err(Error::OverflowMultiplication {
                location: location_1,
                value: result,
                r#type: Type::scalar(Some(self.location), self.is_signed, self.bitlength)
                    .to_string(),
            });
        }

        let bitlength =
            zinc_utils::infer_minimal_bitlength(&result, self.is_signed).map_err(|error| {
                Error::IntegerTooLarge {
                    location: location_1,
                    inner: error,
                }
            })?;
        if bitlength > self.bitlength {
            return Err(Error::OverflowMultiplication {
                location: location_1,
                value: result,
                r#type: Type::scalar(Some(self.location), self.is_signed, self.bitlength)
                    .to_string(),
            });
        }

        #[allow(clippy::suspicious_arithmetic_impl)]
        let is_literal = self.is_literal && other.is_literal;
        let result = Self {
            location: location_1,
            value: result,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: None,
            is_literal,
        };

        let operator = GeneratorExpressionOperator::multiplication_inferred(
            inference_result
                .first
                .map(|r#type| Type::scalar(Some(location_1), r#type.is_signed, r#type.bitlength)),
            inference_result
                .second
                .map(|r#type| Type::scalar(Some(location_2), r#type.is_signed, r#type.bitlength)),
        );

        Ok((result, operator))
    }
}

impl Div for Integer {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn div(mut self, mut other: Self) -> Self::Output {
        let location_1 = self.location;
        let location_2 = other.location;

        let inference_result = zinc_utils::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchDivision {
                location: self.location,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        if self.bitlength == zinc_const::bitlength::FIELD {
            return Err(Error::ForbiddenFieldDivision {
                location: self.location,
            });
        }

        let (result, _remainder) = zinc_utils::euclidean_div_rem(&self.value, &other.value).ok_or(
            Error::ZeroDivision {
                location: other.location,
            },
        )?;
        if result.is_negative() && !self.is_signed {
            return Err(Error::OverflowDivision {
                location: self.location,
                value: result,
                r#type: Type::scalar(Some(self.location), self.is_signed, self.bitlength)
                    .to_string(),
            });
        }

        let bitlength =
            zinc_utils::infer_minimal_bitlength(&result, self.is_signed).map_err(|error| {
                Error::IntegerTooLarge {
                    location: self.location,
                    inner: error,
                }
            })?;
        if bitlength > self.bitlength {
            return Err(Error::OverflowDivision {
                location: self.location,
                value: result,
                r#type: Type::scalar(Some(self.location), self.is_signed, self.bitlength)
                    .to_string(),
            });
        }

        #[allow(clippy::suspicious_arithmetic_impl)]
        let is_literal = self.is_literal && other.is_literal;
        let result = Self {
            location: self.location,
            value: result,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: None,
            is_literal,
        };

        let operator = GeneratorExpressionOperator::division_inferred(
            inference_result
                .first
                .map(|r#type| Type::scalar(Some(location_1), r#type.is_signed, r#type.bitlength)),
            inference_result
                .second
                .map(|r#type| Type::scalar(Some(location_2), r#type.is_signed, r#type.bitlength)),
        );

        Ok((result, operator))
    }
}

impl Rem for Integer {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn rem(mut self, mut other: Self) -> Self::Output {
        let location_1 = self.location;
        let location_2 = other.location;

        let inference_result = zinc_utils::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::TypesMismatchRemainder {
                location: self.location,
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        if self.bitlength == zinc_const::bitlength::FIELD {
            return Err(Error::ForbiddenFieldRemainder {
                location: self.location,
            });
        }

        let (_quotient, result) = zinc_utils::euclidean_div_rem(&self.value, &other.value).ok_or(
            Error::ZeroRemainder {
                location: other.location,
            },
        )?;
        if result.is_negative() && !self.is_signed {
            return Err(Error::OverflowRemainder {
                location: self.location,
                value: result,
                r#type: Type::scalar(Some(self.location), self.is_signed, self.bitlength)
                    .to_string(),
            });
        }

        let bitlength =
            zinc_utils::infer_minimal_bitlength(&result, self.is_signed).map_err(|error| {
                Error::IntegerTooLarge {
                    location: self.location,
                    inner: error,
                }
            })?;
        if bitlength > self.bitlength {
            return Err(Error::OverflowRemainder {
                location: self.location,
                value: result,
                r#type: Type::scalar(Some(self.location), self.is_signed, self.bitlength)
                    .to_string(),
            });
        }

        #[allow(clippy::suspicious_arithmetic_impl)]
        let is_literal = self.is_literal && other.is_literal;
        let result = Self {
            location: self.location,
            value: result,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: None,
            is_literal,
        };

        let operator = GeneratorExpressionOperator::remainder_inferred(
            inference_result
                .first
                .map(|r#type| Type::scalar(Some(location_1), r#type.is_signed, r#type.bitlength)),
            inference_result
                .second
                .map(|r#type| Type::scalar(Some(location_2), r#type.is_signed, r#type.bitlength)),
        );

        Ok((result, operator))
    }
}

impl Integer {
    ///
    /// Executes the `as` casting operator.
    ///
    pub fn cast(
        self,
        is_signed: bool,
        bitlength: usize,
    ) -> Result<(Self, Option<GeneratorExpressionOperator>), Error> {
        if self.value.is_negative() && !is_signed {
            return Err(Error::OverflowCasting {
                location: self.location,
                value: self.value,
                r#type: Type::scalar(Some(self.location), is_signed, bitlength).to_string(),
            });
        }

        let inferred_bitlength = zinc_utils::infer_minimal_bitlength(&self.value, is_signed)
            .map_err(|error| Error::IntegerTooLarge {
                location: self.location,
                inner: error,
            })?;
        if inferred_bitlength > bitlength {
            return Err(Error::OverflowCasting {
                location: self.location,
                value: self.value,
                r#type: Type::scalar(Some(self.location), is_signed, bitlength).to_string(),
            });
        }

        let operator = if self.is_signed != is_signed || self.bitlength != bitlength {
            GeneratorExpressionOperator::try_casting(&Type::scalar(
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
            is_literal: false,
        };

        Ok((result, operator))
    }

    ///
    /// Executes the `~` bitwise NOT operator.
    ///
    pub fn bitwise_not(self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if self.is_signed {
            return Err(Error::ForbiddenSignedBitwise {
                location: self.location,
            });
        }

        if self.bitlength == zinc_const::bitlength::FIELD {
            return Err(Error::ForbiddenFieldBitwise {
                location: self.location,
            });
        }

        let result = Self {
            location: self.location,
            value: !self.value,
            is_signed: self.is_signed,
            bitlength: self.bitlength,
            enumeration: None,
            is_literal: self.is_literal,
        };

        let operator = GeneratorExpressionOperator::BitwiseNot;

        Ok((result, operator))
    }
}

impl Neg for Integer {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn neg(self) -> Self::Output {
        let location = self.location;

        if self.bitlength == zinc_const::bitlength::FIELD {
            return Err(Error::ForbiddenFieldNegation {
                location: self.location,
            });
        }

        let is_signed = true;

        let result = -self.value;
        let bitlength =
            zinc_utils::infer_minimal_bitlength(&result, is_signed).map_err(|error| {
                Error::IntegerTooLarge {
                    location,
                    inner: error,
                }
            })?;
        if bitlength > self.bitlength {
            return Err(Error::OverflowNegation {
                location,
                value: result,
                r#type: Type::scalar(Some(self.location), is_signed, self.bitlength).to_string(),
            });
        }

        let result = Self {
            location,
            value: result,
            is_signed,
            bitlength: self.bitlength,
            enumeration: None,
            is_literal: self.is_literal,
        };

        let operator = GeneratorExpressionOperator::Negation;

        Ok((result, operator))
    }
}

impl Integer {
    ///
    /// Tries to convert the constant to a `usize` value.
    ///
    /// Returns an error, if the constant is too big or negative.
    ///
    pub fn to_usize(&self) -> Result<usize, Error> {
        self.value.to_usize().ok_or_else(|| Error::IntegerTooLarge {
            location: self.location,
            inner: InferenceError::Overflow {
                value: self.value.to_owned(),
                is_signed: false,
                bitlength: zinc_const::bitlength::INDEX,
            },
        })
    }

    ///
    /// Calculates the minimal bitlength required to represent each element of `literals`.
    ///
    pub fn minimal_bitlength_literals(literals: &[&IntegerLiteral]) -> Result<usize, Error> {
        let mut result = zinc_const::bitlength::BYTE;

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
        let mut result = zinc_const::bitlength::BYTE;

        for value in values.iter() {
            let bitlength =
                zinc_utils::infer_minimal_bitlength(value, is_signed).map_err(|error| {
                    Error::IntegerTooLarge {
                        location,
                        inner: error,
                    }
                })?;
            if bitlength > result {
                result = bitlength;
            }
        }

        Ok(result)
    }
}

impl ITyped for Integer {
    fn r#type(&self) -> Type {
        match self.enumeration {
            Some(ref enumeration) => Type::Enumeration(enumeration.to_owned()),
            None => Type::scalar(Some(self.location), self.is_signed, self.bitlength),
        }
    }

    fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.is_signed == other.is_signed
            && self.bitlength == other.bitlength
            && match (self.enumeration.as_ref(), other.enumeration.as_ref()) {
                (Some(enumeration_1), Some(enumeration_2)) => enumeration_1 == enumeration_2,
                (None, None) => true,
                _ => false,
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
        let (string, base) = match literal.inner {
            LexicalIntegerLiteral::Binary { ref inner } => (inner, zinc_const::base::BINARY),
            LexicalIntegerLiteral::Octal { ref inner } => (inner, zinc_const::base::OCTAL),
            LexicalIntegerLiteral::Decimal { ref inner } => (inner, zinc_const::base::DECIMAL),
            LexicalIntegerLiteral::Hexadecimal { ref inner } => {
                (inner, zinc_const::base::HEXADECIMAL)
            }
        };

        let value = BigInt::from_str_radix(&string, base)
            .expect(zinc_const::panic::VALIDATED_DURING_LEXICAL_ANALYSIS);
        let bitlength = zinc_utils::infer_minimal_bitlength(&value, false).map_err(|error| {
            Error::IntegerTooLarge {
                location: literal.location,
                inner: error,
            }
        })?;

        Ok(Self::new(literal.location, value, false, bitlength, true))
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}' of type '{}'", self.value, self.r#type())
    }
}
