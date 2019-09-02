//!
//! The interpreter stack.
//!

use std::fmt;

use num_bigint::BigInt;
use num_traits::One;
use num_traits::Zero;
use serde_derive::Serialize;

use crate::interpreter::OperatorError;
use crate::interpreter::Place;
use crate::interpreter::Value;
use crate::syntax::ExpressionOperator;
use crate::syntax::Type;
use crate::syntax::TypeVariant;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Element {
    Place(Place),
    Value(Value),
    Type(Type),
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Place(ref place) => write!(f, "{}", place),
            Self::Value(ref value) => write!(f, "{}", value),
            Self::Type(r#type) => write!(f, "{}", r#type),
        }
    }
}

impl Element {
    pub fn assign(self, other: Self) -> Result<(), OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Assignment;

        let value_1 = match self.clone() {
            Self::Place(place) => place.value,
            Self::Value(..) => {
                return Err(OperatorError::assignment_to_value_expression(self, other))
            }
            Self::Type(..) => {
                return Err(OperatorError::assignment_to_type_expression(self, other))
            }
        };
        let value_2 = match other.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => return Err(OperatorError::assignment_type_expression(self, other)),
        };
        if !value_1.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !value_2.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if value_1.type_variant != value_2.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                OPERATOR,
                value_2.type_variant,
                value_1.type_variant,
            ));
        }

        Ok(())
    }

    pub fn or(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Or;

        let value_1 = match self.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, self,
                ))
            }
        };
        let value_2 = match other.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, other,
                ))
            }
        };
        if !value_1.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !value_2.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if value_1.type_variant != value_2.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                OPERATOR,
                value_2.type_variant,
                value_1.type_variant,
            ));
        }

        let result = if value_1.field.is_one() || value_2.field.is_one() {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Self::Value(Value::new(result, TypeVariant::Bool)))
    }

    pub fn xor(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Xor;

        let value_1 = match self.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, self,
                ));
            }
        };
        let value_2 = match other.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, other,
                ));
            }
        };
        if !value_1.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !value_2.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if value_1.type_variant != value_2.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                OPERATOR,
                value_2.type_variant,
                value_1.type_variant,
            ));
        }

        let result = if (value_1.field + value_2.field).is_one() {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Self::Value(Value::new(result, TypeVariant::Bool)))
    }

    pub fn and(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::And;

        let value_1 = match self.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, self,
                ));
            }
        };
        let value_2 = match other.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, other,
                ));
            }
        };
        if !value_1.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !value_2.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if value_1.type_variant != value_2.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                OPERATOR,
                value_2.type_variant,
                value_1.type_variant,
            ));
        }

        let result = if value_1.field.is_one() && value_2.field.is_one() {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Self::Value(Value::new(result, TypeVariant::Bool)))
    }

    pub fn equal(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Equal;

        let value_1 = match self.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, self,
                ));
            }
        };
        let value_2 = match other.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, other,
                ));
            }
        };
        if !value_1.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !value_2.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if value_1.type_variant != value_2.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                OPERATOR,
                value_2.type_variant,
                value_1.type_variant,
            ));
        }

        let result = if value_1.field == value_2.field {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Self::Value(Value::new(result, TypeVariant::Bool)))
    }

    pub fn not_equal(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::NotEqual;

        let value_1 = match self.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, self,
                ));
            }
        };
        let value_2 = match other.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, other,
                ));
            }
        };
        if !value_1.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !value_2.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if value_1.type_variant != value_2.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                OPERATOR,
                value_2.type_variant,
                value_1.type_variant,
            ));
        }

        let result = if value_1.field != value_2.field {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Self::Value(Value::new(result, TypeVariant::Bool)))
    }

    pub fn greater_equal(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::GreaterEqual;

        let value_1 = match self.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, self,
                ));
            }
        };
        let value_2 = match other.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, other,
                ));
            }
        };
        if !value_1.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !value_2.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if value_1.type_variant != value_2.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                OPERATOR,
                value_2.type_variant,
                value_1.type_variant,
            ));
        }

        let result = if value_1.field >= value_2.field {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Self::Value(Value::new(result, TypeVariant::Bool)))
    }

    pub fn lesser_equal(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::LesserEqual;

        let value_1 = match self.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, self,
                ));
            }
        };
        let value_2 = match other.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, other,
                ));
            }
        };
        if !value_1.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !value_2.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if value_1.type_variant != value_2.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                OPERATOR,
                value_2.type_variant,
                value_1.type_variant,
            ));
        }

        let result = if value_1.field <= value_2.field {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Self::Value(Value::new(result, TypeVariant::Bool)))
    }

    pub fn greater(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Greater;

        let value_1 = match self.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, self,
                ));
            }
        };
        let value_2 = match other.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, other,
                ));
            }
        };
        if !value_1.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !value_2.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if value_1.type_variant != value_2.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                OPERATOR,
                value_2.type_variant,
                value_1.type_variant,
            ));
        }

        let result = if value_1.field > value_2.field {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Self::Value(Value::new(result, TypeVariant::Bool)))
    }

    pub fn lesser(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Lesser;

        let value_1 = match self.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, self,
                ));
            }
        };
        let value_2 = match other.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, other,
                ));
            }
        };
        if !value_1.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !value_2.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if value_1.type_variant != value_2.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                OPERATOR,
                value_2.type_variant,
                value_1.type_variant,
            ));
        }

        let result = if value_1.field < value_2.field {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Self::Value(Value::new(result, TypeVariant::Bool)))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Addition;

        let value_1 = match self.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, self,
                ));
            }
        };
        let value_2 = match other.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, other,
                ));
            }
        };
        if !value_1.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !value_2.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if value_1.type_variant != value_2.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                OPERATOR,
                value_2.type_variant,
                value_1.type_variant,
            ));
        }

        let result = value_1.field + value_2.field;
        Ok(Self::Value(Value::new(result, value_1.type_variant)))
    }

    pub fn subtract(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Subtraction;

        let value_1 = match self.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, self,
                ));
            }
        };
        let value_2 = match other.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, other,
                ));
            }
        };
        if !value_1.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !value_2.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if value_1.type_variant != value_2.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                OPERATOR,
                value_2.type_variant,
                value_1.type_variant,
            ));
        }

        let result = value_1.field - value_2.field;
        Ok(Self::Value(Value::new(result, value_1.type_variant)))
    }

    pub fn multiply(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Multiplication;

        let value_1 = match self.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, self,
                ));
            }
        };
        let value_2 = match other.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, other,
                ));
            }
        };
        if !value_1.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !value_2.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if value_1.type_variant != value_2.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                OPERATOR,
                value_2.type_variant,
                value_1.type_variant,
            ));
        }

        let result = value_1.field * value_2.field;
        Ok(Self::Value(Value::new(result, value_1.type_variant)))
    }

    pub fn divide(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Division;

        let value_1 = match self.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, self,
                ));
            }
        };
        let value_2 = match other.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, other,
                ));
            }
        };
        if !value_1.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !value_2.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if value_1.type_variant != value_2.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                OPERATOR,
                value_2.type_variant,
                value_1.type_variant,
            ));
        }

        let result = value_1.field / value_2.field;
        Ok(Self::Value(Value::new(result, value_1.type_variant)))
    }

    pub fn modulo(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Remainder;

        let value_1 = match self.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, self,
                ));
            }
        };
        let value_2 = match other.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, other,
                ));
            }
        };
        if !value_1.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }
        if !value_2.type_variant.can_be_second_operand(OPERATOR) {
            return Err(OperatorError::second_operand_operator_not_available(
                OPERATOR, other,
            ));
        }
        if value_1.type_variant != value_2.type_variant {
            return Err(OperatorError::operand_type_mismatch(
                OPERATOR,
                value_2.type_variant,
                value_1.type_variant,
            ));
        }

        let result = value_1.field % value_2.field;
        Ok(Self::Value(Value::new(result, value_1.type_variant)))
    }

    pub fn negate(self) -> Result<Self, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Negation;

        let value_1 = match self.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, self,
                ));
            }
        };
        if !value_1.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }

        let result = -value_1.field;
        let type_variant = if let TypeVariant::Uint { bitlength } = value_1.type_variant {
            TypeVariant::Int { bitlength }
        } else {
            value_1.type_variant
        };
        Ok(Self::Value(Value::new(result, type_variant)))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> Result<Self, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Not;

        let value_1 = match self.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, self,
                ));
            }
        };
        if !value_1.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, self,
            ));
        }

        let result = if value_1.field.is_zero() {
            BigInt::one()
        } else {
            BigInt::zero()
        };
        Ok(Self::Value(Value::new(result, TypeVariant::Bool)))
    }

    pub fn cast(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: ExpressionOperator = ExpressionOperator::Casting;

        let value_1 = match self.clone() {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            Self::Type(..) => {
                return Err(OperatorError::type_expression_not_as_the_casting_operand(
                    OPERATOR, self,
                ));
            }
        };

        let other_type_variant = match other.clone() {
            Self::Place(..) => {
                return Err(OperatorError::casting_to_place_expression(self, other));
            }
            Self::Value(..) => {
                return Err(OperatorError::casting_to_value_expression(self, other));
            }
            Self::Type(r#type) => r#type.variant,
        };

        if !value_1.type_variant.can_be_first_operand(OPERATOR) {
            return Err(OperatorError::first_operand_operator_not_available(
                OPERATOR, other,
            ));
        }

        Ok(Self::Value(value_1.cast(other_type_variant)?))
    }
}
