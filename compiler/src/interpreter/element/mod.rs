//!
//! The interpreter element.
//!

mod error;
mod place;
mod value;

pub use self::error::Error;
pub use self::place::Place;
pub use self::value::Error as ValueError;
pub use self::value::Value;

use std::fmt;

use bellman::ConstraintSystem;
use franklin_crypto::circuit::boolean::Boolean;
use pairing::bn256::Bn256;

use crate::syntax::OperatorExpressionOperator;
use crate::syntax::Type;

#[derive(Debug, Clone, PartialEq)]
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
    pub fn assign<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        system: &mut S,
    ) -> Result<Place, Error> {
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

    pub fn or<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        system: &mut S,
    ) -> Result<Self, Error> {
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

        jab::or(system, &Boolean::from(value_1), &Boolean::from(value_2))
            .map(|value| Self::Value(Value::Boolean(value.get_variable().cloned().unwrap())))
            .map_err(|error| Error::Synthesis(error.to_string()))
    }

    pub fn xor<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        system: &mut S,
    ) -> Result<Self, Error> {
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

        jab::xor(system, &Boolean::from(value_1), &Boolean::from(value_2))
            .map(|value| Self::Value(Value::Boolean(value.get_variable().cloned().unwrap())))
            .map_err(|error| Error::Synthesis(error.to_string()))
    }

    pub fn and<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        system: &mut S,
    ) -> Result<Self, Error> {
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

        jab::and(system, &Boolean::from(value_1), &Boolean::from(value_2))
            .map(|value| Self::Value(Value::Boolean(value.get_variable().cloned().unwrap())))
            .map_err(|error| Error::Synthesis(error.to_string()))
    }

    pub fn equal<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        system: &mut S,
    ) -> Result<Self, Error> {
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

        unimplemented!();
        //        jab::equals(system, &value_1, &value_2, 0)
        //            .map(|value| Self::Value(Value::Boolean(*value.get_variable().unwrap())))
        //            .map_err(|error| Error::Synthesis(error.to_string()))
    }

    pub fn not_equal<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        system: &mut S,
    ) -> Result<Self, Error> {
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

        unimplemented!();
        //        jab::not_equals(system, &value_1, &value_2, 0)
        //            .map(|value| Self::Value(Value::Boolean(*value.get_variable().unwrap())))
        //            .map_err(|error| Error::Synthesis(error.to_string()))
    }

    pub fn greater_equal<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        system: &mut S,
    ) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::GreaterEqual;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        jab::greater_equals(system, &value_1, &value_2, 0)
            .map(|value| Self::Value(Value::Boolean(value.get_variable().cloned().unwrap())))
            .map_err(|error| Error::Synthesis(error.to_string()))
    }

    pub fn lesser_equal<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        system: &mut S,
    ) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::LesserEqual;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        jab::lesser_equals(system, &value_1, &value_2, 0)
            .map(|value| Self::Value(Value::Boolean(value.get_variable().cloned().unwrap())))
            .map_err(|error| Error::Synthesis(error.to_string()))
    }

    pub fn greater<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        system: &mut S,
    ) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Greater;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        jab::greater(system, &value_1, &value_2, 0)
            .map(|value| Self::Value(Value::Boolean(value.get_variable().cloned().unwrap())))
            .map_err(|error| Error::Synthesis(error.to_string()))
    }

    pub fn lesser<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        system: &mut S,
    ) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Lesser;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        jab::lesser(system, &value_1, &value_2, 0)
            .map(|value| Self::Value(Value::Boolean(value.get_variable().cloned().unwrap())))
            .map_err(|error| Error::Synthesis(error.to_string()))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        system: &mut S,
    ) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Addition;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        jab::addition(system, &value_1, &value_2, 0)
            .map(|(value, _bits)| Self::Value(Value::Integer(value)))
            .map_err(|error| Error::Synthesis(error.to_string()))
    }

    pub fn subtract<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        system: &mut S,
    ) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Subtraction;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        jab::subtraction(system, &value_1, &value_2, 0)
            .map(|(value, _bits)| Self::Value(Value::Integer(value)))
            .map_err(|error| Error::Synthesis(error.to_string()))
    }

    pub fn multiply<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        system: &mut S,
    ) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Multiplication;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        jab::multiplication(system, &value_1, &value_2, 0)
            .map(|(value, _bits)| Self::Value(Value::Integer(value)))
            .map_err(|error| Error::Synthesis(error.to_string()))
    }

    pub fn divide<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        system: &mut S,
    ) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Division;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        unimplemented!();
    }

    pub fn modulo<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        system: &mut S,
    ) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Remainder;

        let value_1 = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        let value_2 = match other {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        unimplemented!();
    }

    pub fn negate<S: ConstraintSystem<Bn256>>(self, system: &mut S) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Negation;

        let value = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        jab::negation(system, &value, 0)
            .map(|(value, _bits)| Self::Value(Value::Integer(value)))
            .map_err(|error| Error::Synthesis(error.to_string()))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn not<S: ConstraintSystem<Bn256>>(self, system: &mut S) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Not;

        let value = match self {
            Self::Place(Place {
                value: Value::Boolean(value),
                ..
            }) => value,
            Self::Value(Value::Boolean(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        jab::not(system, &Boolean::from(value))
            .map(|value| Self::Value(Value::Boolean(value.get_variable().cloned().unwrap())))
            .map_err(|error| Error::Synthesis(error.to_string()))
    }

    pub fn cast<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        system: &mut S,
    ) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Casting;

        let value = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        let r#type = match other {
            Self::Type(r#type) => r#type,
            value => return Err(Error::ExpectedTypeExpression(OPERATOR, value)),
        };

        jab::casting(system, &value, 0)
            .map(|value| Self::Value(Value::Integer(value)))
            .map_err(|error| Error::Synthesis(error.to_string()))
    }
}
