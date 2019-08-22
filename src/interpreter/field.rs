//!
//! The interpreter field.
//!

use std::fmt;

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
use crate::syntax::Type;

#[derive(Debug, Serialize)]
pub struct Field {
    #[serde(skip_serializing)]
    pub value: BigInt,
    pub value_type: Type,
}

impl Field {
    pub fn new(value: BigInt, value_type: Type) -> Self {
        Self { value, value_type }
    }

    pub fn addition(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Addition;

        if !self.value_type.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.value_type.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.value_type != other.value_type {
            return Err(Error::operand_type_mismatch(
                other.value_type,
                self.value_type,
            ));
        }

        let value = self.value + other.value;
        let value_type = self.value_type;
        Ok(Field::new(value, value_type))
    }

    pub fn subtraction(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Subtraction;

        if !self.value_type.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.value_type.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.value_type != other.value_type {
            return Err(Error::operand_type_mismatch(
                other.value_type,
                self.value_type,
            ));
        }

        let value = self.value - other.value;
        let value_type = self.value_type;
        Ok(Field::new(value, value_type))
    }

    pub fn multiplication(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Multiplication;

        if !self.value_type.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.value_type.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.value_type != other.value_type {
            return Err(Error::operand_type_mismatch(
                other.value_type,
                self.value_type,
            ));
        }

        let value = self.value * other.value;
        let value_type = self.value_type;
        Ok(Field::new(value, value_type))
    }

    pub fn division(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Division;

        if !self.value_type.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.value_type.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.value_type != other.value_type {
            return Err(Error::operand_type_mismatch(
                other.value_type,
                self.value_type,
            ));
        }

        let value = self.value / other.value;
        let value_type = self.value_type;
        Ok(Field::new(value, value_type))
    }

    pub fn remainder(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Remainder;

        if !self.value_type.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.value_type.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.value_type != other.value_type {
            return Err(Error::operand_type_mismatch(
                other.value_type,
                self.value_type,
            ));
        }

        let value = self.value % other.value;
        let value_type = self.value_type;
        Ok(Field::new(value, value_type))
    }

    pub fn negation(self) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Negation;

        if !self.value_type.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }

        let value = -self.value;
        let value_type = self.value_type;
        Ok(Field::new(value, value_type))
    }

    pub fn or(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Or;

        if !self.value_type.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.value_type.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.value_type != other.value_type {
            return Err(Error::operand_type_mismatch(
                other.value_type,
                self.value_type,
            ));
        }

        let value = if self.value.is_one() || other.value.is_one() {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Field::new(value, Type::Bool))
    }

    pub fn xor(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Xor;

        if !self.value_type.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.value_type.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.value_type != other.value_type {
            return Err(Error::operand_type_mismatch(
                other.value_type,
                self.value_type,
            ));
        }

        let value = if (self.value.is_zero() && other.value.is_one())
            || (self.value.is_one() && other.value.is_zero())
        {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Field::new(value, Type::Bool))
    }

    pub fn and(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::And;

        if !self.value_type.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.value_type.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.value_type != other.value_type {
            return Err(Error::operand_type_mismatch(
                other.value_type,
                self.value_type,
            ));
        }

        let value = if self.value.is_one() && other.value.is_one() {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Field::new(value, Type::Bool))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Not;

        if !self.value_type.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }

        let value = if self.value.is_zero() {
            BigInt::one()
        } else if self.value.is_one() {
            BigInt::zero()
        } else {
            panic!("Invalid boolean value");
        };
        Ok(Field::new(value, Type::Bool))
    }

    pub fn equal(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Equal;

        if !self.value_type.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.value_type.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.value_type != other.value_type {
            return Err(Error::operand_type_mismatch(
                other.value_type,
                self.value_type,
            ));
        }

        let value = if self.value == other.value {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Field::new(value, Type::Bool))
    }

    pub fn not_equal(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::NotEqual;

        if !self.value_type.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.value_type.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.value_type != other.value_type {
            return Err(Error::operand_type_mismatch(
                other.value_type,
                self.value_type,
            ));
        }

        let value = if self.value != other.value {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Field::new(value, Type::Bool))
    }

    pub fn greater_equal(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::GreaterEqual;

        if !self.value_type.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.value_type.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.value_type != other.value_type {
            return Err(Error::operand_type_mismatch(
                other.value_type,
                self.value_type,
            ));
        }

        let value = if self.value >= other.value {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Field::new(value, Type::Bool))
    }

    pub fn lesser_equal(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::LesserEqual;

        if !self.value_type.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.value_type.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.value_type != other.value_type {
            return Err(Error::operand_type_mismatch(
                other.value_type,
                self.value_type,
            ));
        }

        let value = if self.value <= other.value {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Field::new(value, Type::Bool))
    }

    pub fn greater(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Greater;

        if !self.value_type.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.value_type.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.value_type != other.value_type {
            return Err(Error::operand_type_mismatch(
                other.value_type,
                self.value_type,
            ));
        }

        let value = if self.value > other.value {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Field::new(value, Type::Bool))
    }

    pub fn lesser(self, other: Field) -> Result<Field, Error> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Lesser;

        if !self.value_type.can_be_first_operand(OPERATOR) {
            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
        }
        if !other.value_type.can_be_second_operand(OPERATOR) {
            return Err(Error::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.value_type != other.value_type {
            return Err(Error::operand_type_mismatch(
                other.value_type,
                self.value_type,
            ));
        }

        let value = if self.value < other.value {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Field::new(value, Type::Bool))
    }

    //    pub fn casting(self, r#type: Type) -> Result<Field, Error> {
    //        const OPERATOR: ExpressionOperator = ExpressionOperator::Casting;
    //
    //        if !self.value_type.can_be_first_operand(OPERATOR) {
    //            return Err(Error::first_operand_operator_not_available(OPERATOR, self));
    //        }
    //
    //        Ok(Field::new(self.value, r#type))
    //    }
}

impl From<Literal> for Field {
    fn from(literal: Literal) -> Self {
        match literal {
            Literal::Boolean(boolean) => Self::from(boolean),
            Literal::Integer(integer) => Self::from(integer),
        }
    }
}

impl From<BooleanLiteral> for Field {
    fn from(boolean: BooleanLiteral) -> Self {
        match boolean {
            BooleanLiteral::False => Self::new(BigInt::zero(), Type::Bool),
            BooleanLiteral::True => Self::new(BigInt::one(), Type::Bool),
        }
    }
}

impl From<IntegerLiteral> for Field {
    fn from(integer: IntegerLiteral) -> Self {
        match integer {
            IntegerLiteral::Decimal { value } => {
                let value = BigInt::from_str_radix(value.as_str(), 10)
                    .expect("Decimal integer literal parsing bug");
                let mut bitlength = 2;
                let mut exponent = BigInt::from(4);
                while value >= exponent {
                    exponent *= 2;
                    bitlength += 1;
                }

                let value_type = if 2 <= bitlength && bitlength <= 253 {
                    Type::Uint { bitlength }
                } else if bitlength == 254 {
                    Type::Field
                } else {
                    unreachable!();
                };

                Self::new(value, value_type)
            }
            IntegerLiteral::Hexadecimal { value } => {
                let first_not_zero = value.find(|c: char| c != '0').unwrap_or(0);
                let bitlength = match value.chars().nth(first_not_zero).expect("Unreachable") {
                    '1'..='3' => value.len() * 4 - 3,
                    '4'..='7' => value.len() * 4 - 2,
                    '8'..='9' | 'a'..='b' => value.len() * 4 - 1,
                    'c'..='f' => value.len() * 4,
                    _ => unreachable!(),
                };

                let value = BigInt::from_str_radix(value.as_str(), 16)
                    .expect("Hexadecimal integer literal parsing bug");
                let value_type = if 2 <= bitlength && bitlength <= 253 {
                    Type::Uint { bitlength }
                } else if bitlength == 254 {
                    Type::Field
                } else {
                    unreachable!();
                };

                Self::new(value, value_type)
            }
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.value, self.value_type)
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
    OperandTypesMismatch { got: Type, expected: Type },
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

    pub fn operand_type_mismatch(got: Type, expected: Type) -> Self {
        Self::OperandTypesMismatch { got, expected }
    }
}
