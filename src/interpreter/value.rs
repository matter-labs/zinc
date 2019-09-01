//!
//! The interpreter value.
//!

use std::fmt;
use std::str;

use failure::Fail;
use num_bigint::BigInt;
use num_traits::Num;
use num_traits::One;
use num_traits::Zero;
use serde_derive::Serialize;

use crate::interpreter::OperatorError;
use crate::lexical::BooleanLiteral;
use crate::lexical::IntegerLiteral;
use crate::syntax::ExpressionOperator;
use crate::syntax::TypeVariant;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Value {
    #[serde(skip_serializing)]
    pub value: BigInt,
    pub type_variant: TypeVariant,
}

impl Value {
    pub fn new(value: BigInt, type_variant: TypeVariant) -> Self {
        Self {
            value,
            type_variant,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(self, other: Value) -> Result<Value, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Addition;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = self.value + other.value;
        let type_variant = self.type_variant;
        Ok(Value::new(value, type_variant))
    }

    pub fn subtract(self, other: Value) -> Result<Value, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Subtraction;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = self.value - other.value;
        let type_variant = self.type_variant;
        Ok(Value::new(value, type_variant))
    }

    pub fn multiply(self, other: Value) -> Result<Value, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Multiplication;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = self.value * other.value;
        let type_variant = self.type_variant;
        Ok(Value::new(value, type_variant))
    }

    pub fn divide(self, other: Value) -> Result<Value, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Division;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = self.value / other.value;
        let type_variant = self.type_variant;
        Ok(Value::new(value, type_variant))
    }

    pub fn modulo(self, other: Value) -> Result<Value, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Remainder;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = self.value % other.value;
        let type_variant = self.type_variant;
        Ok(Value::new(value, type_variant))
    }

    pub fn negate(self) -> Result<Value, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Negation;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }

        let value = -self.value;
        let type_variant = if let TypeVariant::Uint { bitlength } = self.type_variant {
            TypeVariant::Int { bitlength }
        } else {
            self.type_variant
        };
        Ok(Value::new(value, type_variant))
    }

    pub fn or(self, other: Value) -> Result<Value, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Or;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = if self.value.is_one() || other.value.is_one() {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Value::new(value, TypeVariant::Bool))
    }

    pub fn xor(self, other: Value) -> Result<Value, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Xor;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(OperatorError::operand_type_mismatch(
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
        Ok(Value::new(value, TypeVariant::Bool))
    }

    pub fn and(self, other: Value) -> Result<Value, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::And;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = if self.value.is_one() && other.value.is_one() {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Value::new(value, TypeVariant::Bool))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> Result<Value, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Not;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }

        let value = if self.value.is_zero() {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Value::new(value, TypeVariant::Bool))
    }

    pub fn equal(self, other: Value) -> Result<Value, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Equal;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = if self.value == other.value {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Value::new(value, TypeVariant::Bool))
    }

    pub fn not_equal(self, other: Value) -> Result<Value, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::NotEqual;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = if self.value != other.value {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Value::new(value, TypeVariant::Bool))
    }

    pub fn greater_equal(self, other: Value) -> Result<Value, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::GreaterEqual;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = if self.value >= other.value {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Value::new(value, TypeVariant::Bool))
    }

    pub fn lesser_equal(self, other: Value) -> Result<Value, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::LesserEqual;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = if self.value <= other.value {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Value::new(value, TypeVariant::Bool))
    }

    pub fn greater(self, other: Value) -> Result<Value, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Greater;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = if self.value > other.value {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Value::new(value, TypeVariant::Bool))
    }

    pub fn lesser(self, other: Value) -> Result<Value, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Lesser;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !other.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if self.type_variant != other.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                other.type_variant,
                self.type_variant,
            ));
        }

        let value = if self.value < other.value {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Value::new(value, TypeVariant::Bool))
    }

    pub fn cast(self, type_variant: TypeVariant) -> Result<Value, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Casting;

        if !self.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }

        match (self.type_variant, type_variant) {
            (TypeVariant::Uint { bitlength: b1 }, TypeVariant::Uint { bitlength: b2 }) => {
                if b1 > b2 {
                    return Err(OperatorError::casting_to_lesser_bitlength(
                        self.type_variant,
                        type_variant,
                    ));
                }
            }
            (TypeVariant::Int { bitlength: b1 }, TypeVariant::Int { bitlength: b2 }) => {
                if b1 > b2 {
                    return Err(OperatorError::casting_to_lesser_bitlength(
                        self.type_variant,
                        type_variant,
                    ));
                }
            }
            (TypeVariant::Uint { bitlength: b1 }, TypeVariant::Int { bitlength: b2 }) => {
                if b1 >= b2 {
                    return Err(OperatorError::casting_to_lesser_bitlength(
                        self.type_variant,
                        type_variant,
                    ));
                }
            }
            (TypeVariant::Int { bitlength: b1 }, TypeVariant::Uint { bitlength: b2 }) => {
                if b1 >= b2 {
                    return Err(OperatorError::casting_to_lesser_bitlength(
                        self.type_variant,
                        type_variant,
                    ));
                }
            }
            _ => {}
        }

        Ok(Value::new(self.value, type_variant))
    }
}

impl From<BooleanLiteral> for Value {
    fn from(boolean: BooleanLiteral) -> Self {
        match boolean {
            BooleanLiteral::False => Self::new(BigInt::zero(), TypeVariant::Bool),
            BooleanLiteral::True => Self::new(BigInt::one(), TypeVariant::Bool),
        }
    }
}

impl From<IntegerLiteral> for Value {
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.value, self.type_variant)
    }
}
