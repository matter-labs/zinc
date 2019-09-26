//!
//! The interpreter element.
//!

mod error;
mod integer;
mod place;
mod value;

pub use self::error::Error;
pub use self::integer::Error as IntegerError;
pub use self::integer::Integer;
pub use self::place::Place;
pub use self::value::Error as ValueError;
pub use self::value::Value;

use std::fmt;

use bellman::ConstraintSystem;
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
    pub fn assign(self, other: Self) -> Result<Place, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Assignment;

        let mut place = match self {
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

        place.assign(value).map_err(Error::Value)?;
        Ok(place)
    }

    pub fn or<CS: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: CS,
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

        jab::or(system.namespace(|| "or"), &value_1, &value_2)
            .map(|value| Self::Value(Value::Boolean(value)))
            .map_err(|error| Error::Synthesis(error.to_string()))
    }

    pub fn xor<CS: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: CS,
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

        jab::xor(system.namespace(|| "xor"), &value_1, &value_2)
            .map(|value| Self::Value(Value::Boolean(value)))
            .map_err(|error| Error::Synthesis(error.to_string()))
    }

    pub fn and<CS: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: CS,
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

        jab::and(system.namespace(|| "and"), &value_1, &value_2)
            .map(|value| Self::Value(Value::Boolean(value)))
            .map_err(|error| Error::Synthesis(error.to_string()))
    }

    pub fn equals<CS: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: CS,
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

        match (value_1, value_2) {
            (Value::Boolean(ref value_1), Value::Boolean(ref value_2)) => {
                jab::equals_boolean(system.namespace(|| "element_equals"), value_1, value_2)
                    .map(|value| Self::Value(Value::Boolean(value)))
                    .map_err(|error| Error::Synthesis(error.to_string()))
            }
            (Value::Integer(ref value_1), Value::Integer(ref value_2)) => jab::equals_number(
                system.namespace(|| "element_equals"),
                &value_1.number,
                &value_2.number,
                value_1.bitlength,
            )
            .map(|value| Self::Value(Value::Boolean(value)))
            .map_err(|error| Error::Synthesis(error.to_string())),
            (value_1, value_2) => Err(Error::ComparingInvalidValues(value_1, value_2)),
        }
    }

    pub fn not_equals<CS: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: CS,
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

        match (value_1, value_2) {
            (Value::Boolean(ref value_1), Value::Boolean(ref value_2)) => {
                jab::not_equals_boolean(system.namespace(|| "element_not_equals"), value_1, value_2)
                    .map(|value| Self::Value(Value::Boolean(value)))
                    .map_err(|error| Error::Synthesis(error.to_string()))
            }
            (Value::Integer(ref value_1), Value::Integer(ref value_2)) => jab::not_equals_number(
                system.namespace(|| "element_not_equals"),
                &value_1.number,
                &value_2.number,
                value_1.bitlength,
            )
            .map(|value| Self::Value(Value::Boolean(value)))
            .map_err(|error| Error::Synthesis(error.to_string())),
            (value_1, value_2) => Err(Error::ComparingInvalidValues(value_1, value_2)),
        }
    }

    pub fn greater_equals<CS: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: CS,
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

        value_1
            .greater_equals(&value_2, system.namespace(|| "element_greater_equal"))
            .map(|value| Self::Value(Value::Boolean(value)))
            .map_err(Error::Integer)
    }

    pub fn lesser_equals<CS: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: CS,
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

        value_1
            .lesser_equals(&value_2, system.namespace(|| "element_lesser_equal"))
            .map(|value| Self::Value(Value::Boolean(value)))
            .map_err(Error::Integer)
    }

    pub fn greater<CS: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: CS,
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

        value_1
            .greater(&value_2, system.namespace(|| "element_greater"))
            .map(|value| Self::Value(Value::Boolean(value)))
            .map_err(Error::Integer)
    }

    pub fn lesser<CS: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: CS,
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

        value_1
            .lesser(&value_2, system.namespace(|| "element_lesser"))
            .map(|value| Self::Value(Value::Boolean(value)))
            .map_err(Error::Integer)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add<CS: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: CS,
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

        value_1
            .add(value_2, system.namespace(|| "element_add"))
            .map(|value| Self::Value(Value::Integer(value)))
            .map_err(Error::Integer)
    }

    pub fn subtract<CS: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: CS,
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

        value_1
            .subtract(value_2, system.namespace(|| "element_subtract"))
            .map(|value| Self::Value(Value::Integer(value)))
            .map_err(Error::Integer)
    }

    pub fn multiply<CS: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: CS,
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

        value_1
            .multiply(value_2, system.namespace(|| "element_multiply"))
            .map(|value| Self::Value(Value::Integer(value)))
            .map_err(Error::Integer)
    }

    pub fn divide<CS: ConstraintSystem<Bn256>>(
        self,
        _other: Self,
        _system: CS,
    ) -> Result<Self, Error> {
        //        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Division;
        //
        //        let value_1 = match self {
        //            Self::Place(Place {
        //                value: Value::Integer(value),
        //                ..
        //            }) => value,
        //            Self::Value(Value::Integer(value)) => value,
        //            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        //        };
        //
        //        let value_2 = match other {
        //            Self::Place(Place {
        //                value: Value::Integer(value),
        //                ..
        //            }) => value,
        //            Self::Value(Value::Integer(value)) => value,
        //            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        //        };

        unimplemented!();
    }

    pub fn modulo<CS: ConstraintSystem<Bn256>>(
        self,
        _other: Self,
        _system: CS,
    ) -> Result<Self, Error> {
        //        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Remainder;
        //
        //        let value_1 = match self {
        //            Self::Place(Place {
        //                value: Value::Integer(value),
        //                ..
        //            }) => value,
        //            Self::Value(Value::Integer(value)) => value,
        //            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        //        };
        //
        //        let value_2 = match other {
        //            Self::Place(Place {
        //                value: Value::Integer(value),
        //                ..
        //            }) => value,
        //            Self::Value(Value::Integer(value)) => value,
        //            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        //        };

        unimplemented!();
    }

    pub fn negate<CS: ConstraintSystem<Bn256>>(self, mut system: CS) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Negation;

        let value = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        value
            .negate(system.namespace(|| "element_negate"))
            .map(|value| Self::Value(Value::Integer(value)))
            .map_err(Error::Integer)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn not<CS: ConstraintSystem<Bn256>>(self, mut system: CS) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Not;

        let value = match self {
            Self::Place(Place {
                value: Value::Boolean(value),
                ..
            }) => value,
            Self::Value(Value::Boolean(value)) => value,
            value => return Err(Error::ExpectedBooleanValue(OPERATOR, value)),
        };

        jab::not(system.namespace(|| "element_not"), &value)
            .map(|value| Self::Value(Value::Boolean(value)))
            .map_err(|error| Error::Synthesis(error.to_string()))
    }

    pub fn cast<CS: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: CS,
    ) -> Result<Self, Error> {
        const OPERATOR: OperatorExpressionOperator = OperatorExpressionOperator::Casting;

        let value = match self {
            Self::Place(Place {
                value: Value::Integer(value),
                ..
            }) => value,
            Self::Value(Value::Integer(value)) => value,
            value => return Err(Error::ExpectedIntegerValue(OPERATOR, value)),
        };

        let type_variant = match other {
            Self::Type(r#type) => r#type.variant,
            value => return Err(Error::ExpectedTypeExpression(OPERATOR, value)),
        };

        value
            .cast(type_variant, system.namespace(|| "element_cast"))
            .map(|value| Self::Value(Value::Integer(value)))
            .map_err(Error::Integer)
    }
}
