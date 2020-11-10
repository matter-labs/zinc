//!
//! The semantic analyzer integer value element.
//!

#[cfg(test)]
mod tests;

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

use zinc_lexical::Location;

use crate::generator::expression::operator::Operator as GeneratorExpressionOperator;
use crate::semantic::element::r#type::enumeration::Enumeration;
use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;

///
/// Integer values consist of the value, sign, and bitlength.
/// If a value belongs to an enumeration, the enumeration type is stored in `enumeration`.
/// Enumeration uniquely defines the value type, even if the sign and bitlength are the same.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Integer {
    /// The location where the value appears in the code.
    pub location: Option<Location>,
    /// The integer type sign.
    pub is_signed: bool,
    /// The integer type bitlength.
    pub bitlength: usize,
    /// If the value was created from an enumeration variant.
    pub enumeration: Option<Enumeration>,
    /// If the value was created from an integer literal.
    pub is_literal: bool,
}

impl Integer {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        location: Option<Location>,
        is_signed: bool,
        bitlength: usize,
        is_literal: bool,
    ) -> Self {
        Self {
            location,
            is_signed,
            bitlength,
            enumeration: None,
            is_literal,
        }
    }

    ///
    /// Set the enumeration type for the value, if the value is an enumeration variant.
    ///
    pub fn set_enumeration(&mut self, enumeration: Enumeration) {
        self.enumeration = Some(enumeration);
    }

    ///
    /// Executes the `==` equals comparison operator.
    ///
    pub fn equals(mut self, mut other: Self) -> Result<GeneratorExpressionOperator, Error> {
        let inference_result = zinc_math::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorEqualsTypesMismatch {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let operator = GeneratorExpressionOperator::equals_inferred(
            inference_result
                .first
                .map(|r#type| Type::scalar(self.location, r#type.is_signed, r#type.bitlength)),
            inference_result
                .second
                .map(|r#type| Type::scalar(other.location, r#type.is_signed, r#type.bitlength)),
        );

        self.is_literal = false;

        Ok(operator)
    }

    ///
    /// Executes the `!=` not-equals comparison operator.
    ///
    pub fn not_equals(mut self, mut other: Self) -> Result<GeneratorExpressionOperator, Error> {
        let inference_result = zinc_math::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorNotEqualsTypesMismatch {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let operator = GeneratorExpressionOperator::not_equals_inferred(
            inference_result
                .first
                .map(|r#type| Type::scalar(self.location, r#type.is_signed, r#type.bitlength)),
            inference_result
                .second
                .map(|r#type| Type::scalar(other.location, r#type.is_signed, r#type.bitlength)),
        );

        self.is_literal = false;

        Ok(operator)
    }

    ///
    /// Executes the `>=` greater-equals comparison operator.
    ///
    pub fn greater_equals(mut self, mut other: Self) -> Result<GeneratorExpressionOperator, Error> {
        let inference_result = zinc_math::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorGreaterEqualsTypesMismatch {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let operator = GeneratorExpressionOperator::greater_equals_inferred(
            inference_result
                .first
                .map(|r#type| Type::scalar(self.location, r#type.is_signed, r#type.bitlength)),
            inference_result
                .second
                .map(|r#type| Type::scalar(other.location, r#type.is_signed, r#type.bitlength)),
        );

        self.is_literal = false;

        Ok(operator)
    }

    ///
    /// Executes the `<=` lesser-equals comparison operator.
    ///
    pub fn lesser_equals(mut self, mut other: Self) -> Result<GeneratorExpressionOperator, Error> {
        let inference_result = zinc_math::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorLesserEqualsTypesMismatch {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let operator = GeneratorExpressionOperator::lesser_equals_inferred(
            inference_result
                .first
                .map(|r#type| Type::scalar(self.location, r#type.is_signed, r#type.bitlength)),
            inference_result
                .second
                .map(|r#type| Type::scalar(other.location, r#type.is_signed, r#type.bitlength)),
        );

        self.is_literal = false;

        Ok(operator)
    }

    ///
    /// Executes the `>` greater comparison operator.
    ///
    pub fn greater(mut self, mut other: Self) -> Result<GeneratorExpressionOperator, Error> {
        let inference_result = zinc_math::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorGreaterTypesMismatch {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let operator = GeneratorExpressionOperator::greater_inferred(
            inference_result
                .first
                .map(|r#type| Type::scalar(self.location, r#type.is_signed, r#type.bitlength)),
            inference_result
                .second
                .map(|r#type| Type::scalar(other.location, r#type.is_signed, r#type.bitlength)),
        );

        self.is_literal = false;

        Ok(operator)
    }

    ///
    /// Executes the `<` lesser comparison operator.
    ///
    pub fn lesser(mut self, mut other: Self) -> Result<GeneratorExpressionOperator, Error> {
        let inference_result = zinc_math::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorLesserTypesMismatch {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let operator = GeneratorExpressionOperator::lesser_inferred(
            inference_result
                .first
                .map(|r#type| Type::scalar(self.location, r#type.is_signed, r#type.bitlength)),
            inference_result
                .second
                .map(|r#type| Type::scalar(other.location, r#type.is_signed, r#type.bitlength)),
        );

        self.is_literal = false;

        Ok(operator)
    }
}

impl BitOr for Integer {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn bitor(mut self, mut other: Self) -> Self::Output {
        let inference_result = zinc_math::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorBitwiseOrTypesMismatch {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        if self.is_signed {
            return Err(Error::OperatorBitwiseSignedOperandForbidden {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
            });
        }

        if self.bitlength == zinc_const::bitlength::FIELD {
            return Err(Error::OperatorBitwiseFieldOperandForbidden {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
            });
        }

        let operator = GeneratorExpressionOperator::bitwise_or_inferred(
            inference_result
                .first
                .map(|r#type| Type::scalar(self.location, r#type.is_signed, r#type.bitlength)),
            inference_result
                .second
                .map(|r#type| Type::scalar(other.location, r#type.is_signed, r#type.bitlength)),
        );

        self.enumeration = None;
        self.is_literal = false;

        Ok((self, operator))
    }
}

impl BitXor for Integer {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn bitxor(mut self, mut other: Self) -> Self::Output {
        let inference_result = zinc_math::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorBitwiseXorTypesMismatch {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        if self.is_signed {
            return Err(Error::OperatorBitwiseSignedOperandForbidden {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
            });
        }

        if self.bitlength == zinc_const::bitlength::FIELD {
            return Err(Error::OperatorBitwiseFieldOperandForbidden {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
            });
        }

        let operator = GeneratorExpressionOperator::bitwise_xor_inferred(
            inference_result
                .first
                .map(|r#type| Type::scalar(self.location, r#type.is_signed, r#type.bitlength)),
            inference_result
                .second
                .map(|r#type| Type::scalar(other.location, r#type.is_signed, r#type.bitlength)),
        );

        self.enumeration = None;
        self.is_literal = false;

        Ok((self, operator))
    }
}

impl BitAnd for Integer {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn bitand(mut self, mut other: Self) -> Self::Output {
        let inference_result = zinc_math::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorBitwiseAndTypesMismatch {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        if self.is_signed {
            return Err(Error::OperatorBitwiseSignedOperandForbidden {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
            });
        }

        if self.bitlength == zinc_const::bitlength::FIELD {
            return Err(Error::OperatorBitwiseFieldOperandForbidden {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
            });
        }

        let operator = GeneratorExpressionOperator::bitwise_and_inferred(
            inference_result
                .first
                .map(|r#type| Type::scalar(self.location, r#type.is_signed, r#type.bitlength)),
            inference_result
                .second
                .map(|r#type| Type::scalar(other.location, r#type.is_signed, r#type.bitlength)),
        );

        self.enumeration = None;
        self.is_literal = false;

        Ok((self, operator))
    }
}

impl Shl<Self> for Integer {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn shl(mut self, other: Self) -> Self::Output {
        if self.is_signed {
            return Err(Error::OperatorBitwiseSignedOperandForbidden {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
            });
        }

        if self.bitlength == zinc_const::bitlength::FIELD {
            return Err(Error::OperatorBitwiseFieldOperandForbidden {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
            });
        }

        if other.is_signed {
            return Err(
                Error::OperatorBitwiseShiftLeftSecondOperatorExpectedUnsigned {
                    location: other
                        .location
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: other.to_string(),
                },
            );
        }

        let operator = GeneratorExpressionOperator::BitwiseShiftLeft;

        self.enumeration = None;
        self.is_literal = false;

        Ok((self, operator))
    }
}

impl Shr<Self> for Integer {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn shr(mut self, other: Self) -> Self::Output {
        if self.is_signed {
            return Err(Error::OperatorBitwiseSignedOperandForbidden {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
            });
        }

        if self.bitlength == zinc_const::bitlength::FIELD {
            return Err(Error::OperatorBitwiseFieldOperandForbidden {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
            });
        }

        if other.is_signed {
            return Err(
                Error::OperatorBitwiseShiftRightSecondOperatorExpectedUnsigned {
                    location: other
                        .location
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    found: other.to_string(),
                },
            );
        }

        let operator = GeneratorExpressionOperator::BitwiseShiftRight;

        self.enumeration = None;
        self.is_literal = false;

        Ok((self, operator))
    }
}

impl Add for Integer {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn add(mut self, mut other: Self) -> Self::Output {
        let inference_result = zinc_math::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorAdditionTypesMismatch {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let operator = GeneratorExpressionOperator::addition_inferred(
            inference_result
                .first
                .map(|r#type| Type::scalar(self.location, r#type.is_signed, r#type.bitlength)),
            inference_result
                .second
                .map(|r#type| Type::scalar(other.location, r#type.is_signed, r#type.bitlength)),
        );

        self.enumeration = None;
        self.is_literal = false;

        Ok((self, operator))
    }
}

impl Sub for Integer {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn sub(mut self, mut other: Self) -> Self::Output {
        let inference_result = zinc_math::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorSubtractionTypesMismatch {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let operator = GeneratorExpressionOperator::subtraction_inferred(
            inference_result
                .first
                .map(|r#type| Type::scalar(self.location, r#type.is_signed, r#type.bitlength)),
            inference_result
                .second
                .map(|r#type| Type::scalar(other.location, r#type.is_signed, r#type.bitlength)),
        );

        self.enumeration = None;
        self.is_literal = false;

        Ok((self, operator))
    }
}

impl Mul for Integer {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn mul(mut self, mut other: Self) -> Self::Output {
        let inference_result = zinc_math::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorMultiplicationTypesMismatch {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        let operator = GeneratorExpressionOperator::multiplication_inferred(
            inference_result
                .first
                .map(|r#type| Type::scalar(self.location, r#type.is_signed, r#type.bitlength)),
            inference_result
                .second
                .map(|r#type| Type::scalar(other.location, r#type.is_signed, r#type.bitlength)),
        );

        self.enumeration = None;
        self.is_literal = false;

        Ok((self, operator))
    }
}

impl Div for Integer {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn div(mut self, mut other: Self) -> Self::Output {
        let inference_result = zinc_math::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorDivisionTypesMismatch {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        if self.bitlength == zinc_const::bitlength::FIELD {
            return Err(Error::OperatorDivisionFieldOperandForbidden {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
            });
        }

        let operator = GeneratorExpressionOperator::division_inferred(
            inference_result
                .first
                .map(|r#type| Type::scalar(self.location, r#type.is_signed, r#type.bitlength)),
            inference_result
                .second
                .map(|r#type| Type::scalar(other.location, r#type.is_signed, r#type.bitlength)),
        );

        self.enumeration = None;
        self.is_literal = false;

        Ok((self, operator))
    }
}

impl Rem for Integer {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn rem(mut self, mut other: Self) -> Self::Output {
        let inference_result = zinc_math::infer_literal_types(
            self.is_literal,
            &mut self.is_signed,
            &mut self.bitlength,
            other.is_literal,
            &mut other.is_signed,
            &mut other.bitlength,
        );

        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperatorRemainderTypesMismatch {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                first: self.r#type().to_string(),
                second: other.r#type().to_string(),
            });
        }

        if self.bitlength == zinc_const::bitlength::FIELD {
            return Err(Error::OperatorRemainderFieldOperandForbidden {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
            });
        }

        let operator = GeneratorExpressionOperator::remainder_inferred(
            inference_result
                .first
                .map(|r#type| Type::scalar(self.location, r#type.is_signed, r#type.bitlength)),
            inference_result
                .second
                .map(|r#type| Type::scalar(other.location, r#type.is_signed, r#type.bitlength)),
        );

        self.enumeration = None;
        self.is_literal = false;

        Ok((self, operator))
    }
}

impl Integer {
    ///
    /// Executes the `as` casting operator.
    ///
    pub fn cast(
        mut self,
        is_signed: bool,
        bitlength: usize,
    ) -> Result<(Self, Option<GeneratorExpressionOperator>), Error> {
        let operator = if self.is_signed != is_signed || self.bitlength != bitlength {
            GeneratorExpressionOperator::try_casting(&Type::scalar(
                self.location,
                is_signed,
                bitlength,
            ))
        } else {
            None
        };

        self.is_signed = is_signed;
        self.bitlength = bitlength;
        self.enumeration = None;
        self.is_literal = false;

        Ok((self, operator))
    }

    ///
    /// Executes the `~` bitwise NOT operator.
    ///
    pub fn bitwise_not(mut self) -> Result<(Self, GeneratorExpressionOperator), Error> {
        if self.is_signed {
            return Err(Error::OperatorBitwiseSignedOperandForbidden {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
            });
        }

        if self.bitlength == zinc_const::bitlength::FIELD {
            return Err(Error::OperatorBitwiseFieldOperandForbidden {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
            });
        }

        let operator = GeneratorExpressionOperator::BitwiseNot;

        self.enumeration = None;
        self.is_literal = false;

        Ok((self, operator))
    }
}

impl Neg for Integer {
    type Output = Result<(Self, GeneratorExpressionOperator), Error>;

    fn neg(mut self) -> Self::Output {
        if self.bitlength == zinc_const::bitlength::FIELD {
            return Err(Error::OperatorNegationFieldOperandForbidden {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
            });
        }

        let operator = GeneratorExpressionOperator::Negation;

        self.is_signed = true;
        self.enumeration = None;
        self.is_literal = false;

        Ok((self, operator))
    }
}

impl ITyped for Integer {
    fn r#type(&self) -> Type {
        match self.enumeration {
            Some(ref enumeration) => Type::Enumeration(enumeration.to_owned()),
            None => Type::scalar(self.location, self.is_signed, self.bitlength),
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

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<runtime> of type '{}'", self.r#type())
    }
}
