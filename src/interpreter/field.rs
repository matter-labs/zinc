//!
//! The interpreter field.
//!

use std::fmt;
use std::str;

use failure::Fail;
use num_bigint::BigInt;
use num_traits::Num;
use num_traits::One;
use num_traits::Zero;
use serde_derive::Serialize;

use crate::lexical::BooleanLiteral;
use crate::lexical::IntegerLiteral;
use crate::lexical::Literal;
use crate::syntax::ExpressionOperator;
use crate::syntax::TypeVariant;

#[derive(Debug, Serialize, Clone)]
pub struct Field {
    #[serde(skip_serializing)]
    pub value: BigInt,
    pub type_variant: TypeVariant,
}

impl Field {
    pub fn new(value: BigInt, type_variant: TypeVariant) -> Self {
        Self {
            value,
            type_variant,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Addition;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(Error::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = self.value + other.value;
        let type_variant = self.type_variant;
        Ok(Field::new(value, type_variant))
    }

    pub fn subtract(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Subtraction;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(Error::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = self.value - other.value;
        let type_variant = self.type_variant;
        Ok(Field::new(value, type_variant))
    }

    pub fn multiply(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Multiplication;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(Error::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = self.value * other.value;
        let type_variant = self.type_variant;
        Ok(Field::new(value, type_variant))
    }

    pub fn divide(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Division;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(Error::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = self.value / other.value;
        let type_variant = self.type_variant;
        Ok(Field::new(value, type_variant))
    }

    pub fn modulo(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Remainder;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(Error::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = self.value % other.value;
        let type_variant = self.type_variant;
        Ok(Field::new(value, type_variant))
    }

    pub fn negate(self) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Negation;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }

        let value = -self.value;
        let type_variant = if let TypeVariant::Uint { bitlength } = self.type_variant {
            TypeVariant::Int { bitlength }
        } else {
            self.type_variant
        };
        Ok(Field::new(value, type_variant))
    }

    pub fn or(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Or;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(Error::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = if self.value.is_one() || other.value.is_one() {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Field::new(value, TypeVariant::Bool))
    }

    pub fn xor(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Xor;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(Error::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = if (self.value.is_zero() && other.value.is_one())
            || (self.value.is_one() && other.value.is_zero())
        {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Field::new(value, TypeVariant::Bool))
    }

    pub fn and(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::And;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(Error::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = if self.value.is_one() && other.value.is_one() {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Field::new(value, TypeVariant::Bool))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Not;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }

        let value = if self.value.is_zero() {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Field::new(value, TypeVariant::Bool))
    }

    pub fn equal(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Equal;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(Error::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = if self.value == other.value {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Field::new(value, TypeVariant::Bool))
    }

    pub fn not_equal(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::NotEqual;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(Error::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = if self.value != other.value {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Field::new(value, TypeVariant::Bool))
    }

    pub fn greater_equal(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::GreaterEqual;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(Error::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = if self.value >= other.value {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Field::new(value, TypeVariant::Bool))
    }

    pub fn lesser_equal(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::LesserEqual;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(Error::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = if self.value <= other.value {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Field::new(value, TypeVariant::Bool))
    }

    pub fn greater(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Greater;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(Error::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = if self.value > other.value {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Field::new(value, TypeVariant::Bool))
    }

    pub fn lesser(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Lesser;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(Error::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = if self.value < other.value {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Field::new(value, TypeVariant::Bool))
    }

    pub fn cast(self, type_variant: TypeVariant) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Casting;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }

        Ok(Field::new(self.value, type_variant))
    }
}

impl From<Literal> for Field {
    fn from(literal: Literal) -> Self {
        match literal {
            Literal::Boolean(boolean) => Self::from(boolean),
            Literal::Integer(integer) => Self::from(integer),
            Literal::String(string) => panic!("Field from string '{}' casting bug", string),
        }
    }
}

impl From<BooleanLiteral> for Field {
    fn from(boolean: BooleanLiteral) -> Self {
        match boolean {
            BooleanLiteral::False => Self::new(BigInt::zero(), TypeVariant::Bool),
            BooleanLiteral::True => Self::new(BigInt::one(), TypeVariant::Bool),
        }
    }
}

impl From<IntegerLiteral> for Field {
    fn from(integer: IntegerLiteral) -> Self {
        match integer {
            IntegerLiteral::Decimal { value } => {
                let value = BigInt::from_str_radix(unsafe { str::from_utf8_unchecked(&value) }, 10)
                    .expect("Decimal integer literal parsing bug");
                let mut bitlength = 2;
                let mut exponent = BigInt::from(4);
                while value >= exponent {
                    exponent *= 2;
                    bitlength += 1;
                }

                let type_variant = if 2 <= bitlength && bitlength <= 253 {
                    TypeVariant::Uint { bitlength }
                } else if bitlength == 254 {
                    TypeVariant::Field
                } else {
                    unreachable!();
                };

                Self::new(value, type_variant)
            }
            IntegerLiteral::Hexadecimal { value } => {
                //                let first_not_zero = value.find(|c: char| c != '0').unwrap_or(0);
                //                let bitlength = match value.chars().nth(first_not_zero).expect("Unreachable") {
                //                    '1'..='3' => value.len() * 4 - 3,
                //                    '4'..='7' => value.len() * 4 - 2,
                //                    '8'..='9' | 'a'..='b' => value.len() * 4 - 1,
                //                    'c'..='f' => value.len() * 4,
                //                    _ => unreachable!(),
                //                };
                let bitlength = value.len() * 4;

                let value = BigInt::from_str_radix(unsafe { str::from_utf8_unchecked(&value) }, 16)
                    .expect("Hexadecimal integer literal parsing bug");
                let type_variant = if 2 <= bitlength && bitlength <= 253 {
                    TypeVariant::Uint { bitlength }
                } else if bitlength == 254 {
                    TypeVariant::Field
                } else {
                    unreachable!();
                };

                Self::new(value, type_variant)
            }
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.value, self.type_variant)
    }
}

#[derive(Debug, Fail, Serialize)]
pub enum Error {
    #[fail(
        display = "operator {} is not available for the first operand: [ {} ]",
        operator, field
    )]
    FirstOperandOperatorNotAvailable {
        operator: ExpressionOperator,
        field: Field,
    },
    #[fail(
        display = "operator {} is not available for the second operand: [ {} ]",
        operator, field
    )]
    SecondOperandOperatorNotAvaiable {
        operator: ExpressionOperator,
        field: Field,
    },
    #[fail(display = "operand type mismatch: got {}, expected {}", got, expected)]
    OperandTypesMismatch {
        got: TypeVariant,
        expected: TypeVariant,
    },
}

impl Error {
    pub fn first_operand_operator_not_available(
        operator: ExpressionOperator,
        field: Field,
    ) -> Self {
        Self::FirstOperandOperatorNotAvailable { operator, field }
    }

    pub fn second_operand_operator_not_available(
        operator: ExpressionOperator,
        field: Field,
    ) -> Self {
        Self::SecondOperandOperatorNotAvaiable { operator, field }
    }

    pub fn operand_type_mismatch(got: TypeVariant, expected: TypeVariant) -> Self {
        Self::OperandTypesMismatch { got, expected }
    }
}
