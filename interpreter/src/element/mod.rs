//!
//! The interpreter element.
//!

mod error;
mod place;
mod value;

pub use self::error::Error;
pub use self::place::Descriptor as PlaceDescriptor;
pub use self::place::Error as PlaceError;
pub use self::place::Place;
pub use self::value::Array;
pub use self::value::ArrayError;
pub use self::value::Boolean;
pub use self::value::BooleanError;
pub use self::value::Error as ValueError;
pub use self::value::Integer;
pub use self::value::IntegerError;
pub use self::value::Structure;
pub use self::value::StructureError;
pub use self::value::Tuple;
pub use self::value::Value;

use std::fmt;

use parser::Type;
use r1cs::Bn256;
use r1cs::ConstraintSystem;

#[derive(Clone, PartialEq)]
pub enum Element {
    Place(Place),
    Value(Value),
    Type(Type),
}

impl Element {
    pub fn assign(self, other: Self) -> Result<(Place, Value), Error> {
        let place = match self {
            Self::Place(place) => place,
            value => {
                return Err(Error::ExpectedPlaceExpression("assign", value));
            }
        };

        let value = match other {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("assign", value)),
        };

        Ok((place, value))
    }

    pub fn or<S: ConstraintSystem<Bn256>>(self, other: Self, mut system: S) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("or", value)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("or", value)),
        };

        value_1
            .or(system.namespace(|| "element_or"), value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn xor<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: S,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("xor", value)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("xor", value)),
        };

        value_1
            .xor(system.namespace(|| "element_xor"), value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn and<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: S,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("and", value)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("and", value)),
        };

        value_1
            .and(system.namespace(|| "element_and"), value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn equals<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: S,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("equals", value)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("equals", value)),
        };

        value_1
            .equals(system.namespace(|| "element_equals"), &value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn not_equals<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: S,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("not_equals", value)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("not_equals", value)),
        };

        value_1
            .not_equals(system.namespace(|| "element_not_equals"), &value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn greater_equals<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: S,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("greater_equals", value)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("greater_equals", value)),
        };

        value_1
            .greater_equals(system.namespace(|| "element_greater_equals"), &value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn lesser_equals<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: S,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("lesser_equals", value)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("lesser_equals", value)),
        };

        value_1
            .lesser_equals(system.namespace(|| "element_lesser_equals"), &value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn greater<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: S,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("greater", value)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("greater", value)),
        };

        value_1
            .greater(system.namespace(|| "element_greater"), &value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn lesser<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: S,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("lesser", value)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("lesser", value)),
        };

        value_1
            .lesser(system.namespace(|| "element_lesser"), &value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn add<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: S,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("add", value)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("add", value)),
        };

        value_1
            .add(system.namespace(|| "element_add"), value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn subtract<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: S,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("subtract", value)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("subtract", value)),
        };

        value_1
            .subtract(system.namespace(|| "element_subtract"), value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn multiply<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: S,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("multiply", value)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("multiply", value)),
        };

        value_1
            .multiply(system.namespace(|| "element_multiply"), value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn divide<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: S,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("divide", value)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("divide", value)),
        };

        value_1
            .divide(system.namespace(|| "element_divide"), value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn modulo<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: S,
    ) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("modulo", value)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("modulo", value)),
        };

        value_1
            .modulo(system.namespace(|| "element_modulo"), value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn negate<S: ConstraintSystem<Bn256>>(self, mut system: S) -> Result<Self, Error> {
        let value = match self {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("negate", value)),
        };

        value
            .negate(system.namespace(|| "element_negate"))
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn not<S: ConstraintSystem<Bn256>>(self, mut system: S) -> Result<Self, Error> {
        let value = match self {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("not", value)),
        };

        value
            .not(system.namespace(|| "element_not"))
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn cast<S: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        mut system: S,
    ) -> Result<Self, Error> {
        let value = match self {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("cast", value)),
        };

        let r#type = match other {
            Self::Type(r#type) => r#type,
            value => return Err(Error::ExpectedTypeExpression("cast", value)),
        };

        value
            .cast(system.namespace(|| "element_cast"), r#type)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn index(self, other: Self) -> Result<Self, Error> {
        let mut place = match self {
            Self::Place(place) => place,
            value => return Err(Error::ExpectedPlaceExpression("index", value)),
        };

        let value = match other {
            Self::Value(value) => value,
            value => return Err(Error::ExpectedValueExpression("index", value)),
        };

        place.index(value).map_err(Error::Place)?;
        Ok(Self::Place(place))
    }

    pub fn field(self, other: Self) -> Result<Self, Error> {
        let mut place = match self {
            Self::Place(place) => place,
            value => return Err(Error::ExpectedPlaceExpression("field", value)),
        };

        match other {
            Self::Value(value) => {
                place.access_tuple(value).map_err(Error::Place)?;
                Ok(Self::Place(place))
            }
            Self::Place(field) => {
                place.access_structure(field).map_err(Error::Place)?;
                Ok(Self::Place(place))
            }
            value => Err(Error::ExpectedValueOrPlaceExpression("field", value)),
        }
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Place(place) => write!(f, "{}", place),
            Self::Value(value) => write!(f, "{}", value),
            Self::Type(r#type) => write!(f, "{}", r#type),
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}
