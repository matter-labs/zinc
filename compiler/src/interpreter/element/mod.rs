//!
//! The interpreter element.
//!

mod error;
mod place;
mod value;

pub use self::error::Error;
pub use self::place::Place;
pub use self::value::Boolean;
pub use self::value::Error as ValueError;
pub use self::value::Integer;
pub use self::value::Value;

use std::fmt;

use serde_derive::Serialize;

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
            Self::Place(place) => write!(f, "{}", place),
            Self::Value(value) => write!(f, "{}", value),
            Self::Type(r#type) => write!(f, "{}", r#type),
        }
    }
}

impl Element {
    pub fn assign(self, other: Self) -> Result<Place, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Assignment;

        let place = match self {
            Self::Place(place) => place,
            value => {
                return Err(Error::ExpectedPlaceExpression(OPERATOR, value));
            }
        };

        let value = match other {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression(OPERATOR, value)),
        };

        Ok(place.assign(value).map_err(Error::Value)?)
    }

    pub fn or(self, other: Self) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Or;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Boolean(value),
                ..
            }) => value,
            Self::Value(Value::Boolean(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Boolean(value),
                ..
            }) => value,
            Self::Value(Value::Boolean(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        Ok(Self::Value(Value::Boolean(value_1.or(&value_2))))
    }

    pub fn xor(self, other: Self) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Xor;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Boolean(value),
                ..
            }) => value,
            Self::Value(Value::Boolean(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Boolean(value),
                ..
            }) => value,
            Self::Value(Value::Boolean(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        Ok(Self::Value(Value::Boolean(value_1.xor(&value_2))))
    }

    pub fn and(self, other: Self) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::And;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Boolean(value),
                ..
            }) => value,
            Self::Value(Value::Boolean(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Boolean(value),
                ..
            }) => value,
            Self::Value(Value::Boolean(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        Ok(Self::Value(Value::Boolean(value_1.and(&value_2))))
    }

    pub fn equal(self, other: Self) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Equal;

        let value_1 = match self {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression(OPERATOR, value)),
        };

        Ok(Self::Value(Value::Boolean(
            value_1.equal(&value_2).map_err(Error::Value)?,
        )))
    }

    pub fn not_equal(self, other: Self) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::NotEqual;

        let value_1 = match self {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(place) => place.value,
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression(OPERATOR, value)),
        };

        Ok(Self::Value(Value::Boolean(
            value_1.not_equal(&value_2).map_err(Error::Value)?,
        )))
    }

    pub fn greater_equal(self, other: Self) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::GreaterEqual;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedIntegerValue(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedIntegerValue(OPERATOR, value)),
        };

        Ok(Self::Value(Value::Boolean(
            value_1.greater_equal(&value_2).map_err(Error::Value)?,
        )))
    }

    pub fn lesser_equal(self, other: Self) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::LesserEqual;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedIntegerValue(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedIntegerValue(OPERATOR, value)),
        };

        Ok(Self::Value(Value::Boolean(
            value_1.lesser_equal(&value_2).map_err(Error::Value)?,
        )))
    }

    pub fn greater(self, other: Self) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Greater;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedIntegerValue(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedIntegerValue(OPERATOR, value)),
        };

        Ok(Self::Value(Value::Boolean(
            value_1.greater(&value_2).map_err(Error::Value)?,
        )))
    }

    pub fn lesser(self, other: Self) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Lesser;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedIntegerValue(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedIntegerValue(OPERATOR, value)),
        };

        Ok(Self::Value(Value::Boolean(
            value_1.lesser(&value_2).map_err(Error::Value)?,
        )))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(self, other: Self) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Addition;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedIntegerValue(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedIntegerValue(OPERATOR, value)),
        };

        Ok(Self::Value(Value::Integer(
            value_1.add(value_2).map_err(Error::Value)?,
        )))
    }

    pub fn subtract(self, other: Self) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Subtraction;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedIntegerValue(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedIntegerValue(OPERATOR, value)),
        };

        Ok(Self::Value(Value::Integer(
            value_1.subtract(value_2).map_err(Error::Value)?,
        )))
    }

    pub fn multiply(self, other: Self) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Multiplication;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedIntegerValue(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedIntegerValue(OPERATOR, value)),
        };

        Ok(Self::Value(Value::Integer(
            value_1.multiply(value_2).map_err(Error::Value)?,
        )))
    }

    pub fn divide(self, other: Self) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Division;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedIntegerValue(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedIntegerValue(OPERATOR, value)),
        };

        Ok(Self::Value(Value::Integer(
            value_1.divide(value_2).map_err(Error::Value)?,
        )))
    }

    pub fn modulo(self, other: Self) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Remainder;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedIntegerValue(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedIntegerValue(OPERATOR, value)),
        };

        Ok(Self::Value(Value::Integer(
            value_1.modulo(value_2).map_err(Error::Value)?,
        )))
    }

    pub fn negate(self) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Negation;

        let value = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedIntegerValue(OPERATOR, value)),
        };

        Ok(Self::Value(Value::Integer(value.negate())))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Not;

        let value = match self {
            Self::Place(Place {
                value: Value::Boolean(value),
                ..
            }) => value,
            Self::Value(Value::Boolean(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        let result = value.not();
        Ok(Self::Value(Value::Boolean(result)))
    }

    pub fn cast(self, other: Self) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Casting;

        let value = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedIntegerValue(OPERATOR, value)),
        };

        let r#type = match other {
            Self::Type(r#type) => r#type,
            value => return Err(Error::ExpectedTypeExpression(OPERATOR, value)),
        };

        Ok(Self::Value(Value::Integer(
            value.cast(r#type.variant).map_err(Error::Value)?,
        )))
    }
}
