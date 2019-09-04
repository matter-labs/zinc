//!
//! The interpreter stack.
//!

use std::fmt;

use serde_derive::Serialize;

use crate::interpreter::Integer;
use crate::interpreter::OperatorError;
use crate::interpreter::Place;
use crate::interpreter::Value;
use crate::syntax::OperatorExpressionOperator;
use crate::syntax::Type;

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
    pub fn assign(self, other: Self) -> Result<Place, OperatorError> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Assignment;

        let mut place = match self {
            Self::Place(ref place) => place.clone(),
            value => {
                return Err(OperatorError::expected_place_expression(OPERATOR, value));
            }
        };

        if !place.is_mutable {
            return Err(OperatorError::assignment_to_immutable_variable(
                place, other,
            ));
        }

        let value = match other {
            Self::Place(ref place) => place.value.clone(),
            Self::Value(ref value) => value.clone(),
            value => return Err(OperatorError::expected_value_expression(OPERATOR, value)),
        };

        if !place.value.has_the_same_type_as(&value) {
            return Err(OperatorError::operand_type_mismatch(OPERATOR, self, other));
        }

        place.value = value;
        Ok(place)
    }

    pub fn or(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Or;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Boolean(value),
                ..
            }) => value,
            Self::Value(Value::Boolean(value)) => value,
            value => return Err(OperatorError::expected_boolean_value(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Boolean(value),
                ..
            }) => value,
            Self::Value(Value::Boolean(value)) => value,
            value => return Err(OperatorError::expected_boolean_value(OPERATOR, value)),
        };

        let result = value_1 || value_2;
        Ok(Self::Value(Value::Boolean(result)))
    }

    pub fn xor(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Xor;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Boolean(value),
                ..
            }) => value,
            Self::Value(Value::Boolean(value)) => value,
            value => return Err(OperatorError::expected_boolean_value(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Boolean(value),
                ..
            }) => value,
            Self::Value(Value::Boolean(value)) => value,
            value => return Err(OperatorError::expected_boolean_value(OPERATOR, value)),
        };

        let result = (value_1 && !value_2) || (!value_1 && value_2);
        Ok(Self::Value(Value::Boolean(result)))
    }

    pub fn and(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::And;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Boolean(value),
                ..
            }) => value,
            Self::Value(Value::Boolean(value)) => value,
            value => return Err(OperatorError::expected_boolean_value(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Boolean(value),
                ..
            }) => value,
            Self::Value(Value::Boolean(value)) => value,
            value => return Err(OperatorError::expected_boolean_value(OPERATOR, value)),
        };

        let result = value_1 && value_2;
        Ok(Self::Value(Value::Boolean(result)))
    }

    pub fn equal(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Equal;

        let value_1 = match self {
            Self::Place(ref place) => place.value.clone(),
            Self::Value(ref value) => value.clone(),
            value => return Err(OperatorError::expected_value_expression(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(ref place) => place.value.clone(),
            Self::Value(ref value) => value.clone(),
            value => return Err(OperatorError::expected_value_expression(OPERATOR, value)),
        };

        if !value_1.has_the_same_type_as(&value_2) {
            return Err(OperatorError::operand_type_mismatch(OPERATOR, self, other));
        }

        let result = value_1 == value_2;
        Ok(Self::Value(Value::Boolean(result)))
    }

    pub fn not_equal(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::NotEqual;

        let value_1 = match self {
            Self::Place(ref place) => place.value.clone(),
            Self::Value(ref value) => value.clone(),
            value => return Err(OperatorError::expected_value_expression(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(ref place) => place.value.clone(),
            Self::Value(ref value) => value.clone(),
            value => return Err(OperatorError::expected_value_expression(OPERATOR, value)),
        };

        if !value_1.has_the_same_type_as(&value_2) {
            return Err(OperatorError::operand_type_mismatch(OPERATOR, self, other));
        }

        let result = value_1 != value_2;
        Ok(Self::Value(Value::Boolean(result)))
    }

    pub fn greater_equal(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::GreaterEqual;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(ref value),
                ..
            }) => value.clone(),
            Self::Value(Value::Integer(ref value)) => value.clone(),
            value => return Err(OperatorError::expected_integer_value(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(ref value),
                ..
            }) => value.clone(),
            Self::Value(Value::Integer(ref value)) => value.clone(),
            value => return Err(OperatorError::expected_integer_value(OPERATOR, value)),
        };

        if !value_1.has_the_same_type_as(&value_2) {
            return Err(OperatorError::operand_type_mismatch(OPERATOR, self, other));
        }

        let result = value_1.data >= value_2.data;
        Ok(Self::Value(Value::Boolean(result)))
    }

    pub fn lesser_equal(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::LesserEqual;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(ref value),
                ..
            }) => value.clone(),
            Self::Value(Value::Integer(ref value)) => value.clone(),
            value => return Err(OperatorError::expected_integer_value(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(ref value),
                ..
            }) => value.clone(),
            Self::Value(Value::Integer(ref value)) => value.clone(),
            value => return Err(OperatorError::expected_integer_value(OPERATOR, value)),
        };

        if !value_1.has_the_same_type_as(&value_2) {
            return Err(OperatorError::operand_type_mismatch(OPERATOR, self, other));
        }

        let result = value_1.data <= value_2.data;
        Ok(Self::Value(Value::Boolean(result)))
    }

    pub fn greater(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Greater;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(ref value),
                ..
            }) => value.clone(),
            Self::Value(Value::Integer(ref value)) => value.clone(),
            value => return Err(OperatorError::expected_integer_value(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(ref value),
                ..
            }) => value.clone(),
            Self::Value(Value::Integer(ref value)) => value.clone(),
            value => return Err(OperatorError::expected_integer_value(OPERATOR, value)),
        };

        if !value_1.has_the_same_type_as(&value_2) {
            return Err(OperatorError::operand_type_mismatch(OPERATOR, self, other));
        }

        let result = value_1.data > value_2.data;
        Ok(Self::Value(Value::Boolean(result)))
    }

    pub fn lesser(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Lesser;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(ref value),
                ..
            }) => value.clone(),
            Self::Value(Value::Integer(ref value)) => value.clone(),
            value => return Err(OperatorError::expected_integer_value(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(ref value),
                ..
            }) => value.clone(),
            Self::Value(Value::Integer(ref value)) => value.clone(),
            value => return Err(OperatorError::expected_integer_value(OPERATOR, value)),
        };

        if !value_1.has_the_same_type_as(&value_2) {
            return Err(OperatorError::operand_type_mismatch(OPERATOR, self, other));
        }

        let result = value_1.data < value_2.data;
        Ok(Self::Value(Value::Boolean(result)))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Addition;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(ref value),
                ..
            }) => value.clone(),
            Self::Value(Value::Integer(ref value)) => value.clone(),
            value => return Err(OperatorError::expected_integer_value(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(ref value),
                ..
            }) => value.clone(),
            Self::Value(Value::Integer(ref value)) => value.clone(),
            value => return Err(OperatorError::expected_integer_value(OPERATOR, value)),
        };

        if !value_1.has_the_same_type_as(&value_2) {
            return Err(OperatorError::operand_type_mismatch(OPERATOR, self, other));
        }

        let result = value_1.data + value_2.data;
        Ok(Self::Value(Value::Integer(Integer::new(
            result,
            value_1.r#type,
        ))))
    }

    pub fn subtract(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Subtraction;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(ref value),
                ..
            }) => value.clone(),
            Self::Value(Value::Integer(ref value)) => value.clone(),
            value => return Err(OperatorError::expected_integer_value(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(ref value),
                ..
            }) => value.clone(),
            Self::Value(Value::Integer(ref value)) => value.clone(),
            value => return Err(OperatorError::expected_integer_value(OPERATOR, value)),
        };

        if !value_1.has_the_same_type_as(&value_2) {
            return Err(OperatorError::operand_type_mismatch(OPERATOR, self, other));
        }

        let result = value_1.data - value_2.data;
        Ok(Self::Value(Value::Integer(Integer::new(
            result,
            value_1.r#type,
        ))))
    }

    pub fn multiply(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Multiplication;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(ref value),
                ..
            }) => value.clone(),
            Self::Value(Value::Integer(ref value)) => value.clone(),
            value => return Err(OperatorError::expected_integer_value(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(ref value),
                ..
            }) => value.clone(),
            Self::Value(Value::Integer(ref value)) => value.clone(),
            value => return Err(OperatorError::expected_integer_value(OPERATOR, value)),
        };

        if !value_1.has_the_same_type_as(&value_2) {
            return Err(OperatorError::operand_type_mismatch(OPERATOR, self, other));
        }

        let result = value_1.data * value_2.data;
        Ok(Self::Value(Value::Integer(Integer::new(
            result,
            value_1.r#type,
        ))))
    }

    pub fn divide(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Division;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(ref value),
                ..
            }) => value.clone(),
            Self::Value(Value::Integer(ref value)) => value.clone(),
            value => return Err(OperatorError::expected_integer_value(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(ref value),
                ..
            }) => value.clone(),
            Self::Value(Value::Integer(ref value)) => value.clone(),
            value => return Err(OperatorError::expected_integer_value(OPERATOR, value)),
        };

        if !value_1.has_the_same_type_as(&value_2) {
            return Err(OperatorError::operand_type_mismatch(OPERATOR, self, other));
        }

        let result = value_1.data / value_2.data;
        Ok(Self::Value(Value::Integer(Integer::new(
            result,
            value_1.r#type,
        ))))
    }

    pub fn modulo(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Remainder;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(ref value),
                ..
            }) => value.clone(),
            Self::Value(Value::Integer(ref value)) => value.clone(),
            value => return Err(OperatorError::expected_integer_value(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(ref value),
                ..
            }) => value.clone(),
            Self::Value(Value::Integer(ref value)) => value.clone(),
            value => return Err(OperatorError::expected_integer_value(OPERATOR, value)),
        };

        if !value_1.has_the_same_type_as(&value_2) {
            return Err(OperatorError::operand_type_mismatch(OPERATOR, self, other));
        }

        let result = value_1.data % value_2.data;
        Ok(Self::Value(Value::Integer(Integer::new(
            result,
            value_1.r#type,
        ))))
    }

    pub fn negate(self) -> Result<Self, OperatorError> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Negation;

        let mut value = match self {
            Self::Place(Place {
                value: Value::Integer(ref value),
                ..
            }) => value.clone(),
            Self::Value(Value::Integer(ref value)) => value.clone(),
            value => return Err(OperatorError::expected_integer_value(OPERATOR, value)),
        };

        value.data = -value.data;
        Ok(Self::Value(Value::Integer(value)))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> Result<Self, OperatorError> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Not;

        let value = match self {
            Self::Place(Place {
                value: Value::Boolean(value),
                ..
            }) => value,
            Self::Value(Value::Boolean(value)) => value,
            value => return Err(OperatorError::expected_boolean_value(OPERATOR, value)),
        };

        let result = !value;
        Ok(Self::Value(Value::Boolean(result)))
    }

    pub fn cast(self, other: Self) -> Result<Self, OperatorError> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Casting;

        let mut value = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(OperatorError::expected_integer_value(OPERATOR, value)),
        };

        let r#type = match other {
            Self::Type(r#type) => r#type,
            value => return Err(OperatorError::expected_type(OPERATOR, value)),
        };

        value.cast(r#type.variant.into())?;
        Ok(Self::Value(Value::Integer(value)))
    }
}
