//!
//! The semantic analyzer element.
//!

mod error;
mod place;
mod value;

pub use self::error::Error;
pub use self::place::Descriptor as PlaceDescriptor;
pub use self::place::Error as PlaceError;
pub use self::place::Place;
pub use self::value::Boolean;
pub use self::value::Error as ValueError;
pub use self::value::Integer;
pub use self::value::IntegerError;
pub use self::value::Value;

use std::fmt;

use crate::semantic::Scope;
use crate::syntax::TypeVariant;

#[derive(Clone, PartialEq)]
pub enum Element {
    /// Memory descriptor a.k.a. lvalue
    Place(Place),
    /// Single operand representing a logical piece of data a.k.a. rvalue
    Value(Value),
    /// Type is used only as the second operand of the casting operator
    Type(TypeVariant),
    /// Value or argument list is used only as the second operator of the function call operator
    ValueList(Vec<Value>),
}

impl Element {
    pub fn type_variant(&self, scope: &Scope) -> Result<TypeVariant, Error> {
        Ok(match self {
            Self::Place(place) => Self::resolve(&place, scope)?.type_variant(),
            Self::Value(value) => value.type_variant(),
            Self::Type(type_variant) => type_variant.to_owned(),
            Self::ValueList(values) => {
                TypeVariant::new_tuple(values.iter().map(|value| value.type_variant()).collect())
            }
        })
    }

    pub fn assign(self, other: Self, scope: &Scope) -> Result<Place, Error> {
        let (value_1, place) = match self {
            Self::Place(place) => (Self::resolve(&place, scope)?, place),
            element => return Err(Error::ExpectedPlaceExpression("assign", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("assign", element)),
        };

        value_1
            .assign(&value_2)
            .map(|_| place)
            .map_err(Error::Value)
    }

    pub fn or(self, other: Self, scope: &Scope) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("or", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("or", element)),
        };

        value_1.or(&value_2).map(Self::Value).map_err(Error::Value)
    }

    pub fn xor(self, other: Self, scope: &Scope) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("xor", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("xor", element)),
        };

        value_1.xor(&value_2).map(Self::Value).map_err(Error::Value)
    }

    pub fn and(self, other: Self, scope: &Scope) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("and", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("and", element)),
        };

        value_1.and(&value_2).map(Self::Value).map_err(Error::Value)
    }

    pub fn equals(self, other: Self, scope: &Scope) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("equals", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("equals", element)),
        };

        value_1
            .equals(&value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn not_equals(self, other: Self, scope: &Scope) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("not_equals", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("not_equals", element)),
        };

        value_1
            .not_equals(&value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn greater_equals(self, other: Self, scope: &Scope) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => {
                return Err(Error::ExpectedResolvableExpression(
                    "greater_equals",
                    element,
                ))
            }
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => {
                return Err(Error::ExpectedResolvableExpression(
                    "greater_equals",
                    element,
                ))
            }
        };

        value_1
            .greater_equals(&value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn lesser_equals(self, other: Self, scope: &Scope) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => {
                return Err(Error::ExpectedResolvableExpression(
                    "lesser_equals",
                    element,
                ))
            }
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => {
                return Err(Error::ExpectedResolvableExpression(
                    "lesser_equals",
                    element,
                ))
            }
        };

        value_1
            .lesser_equals(&value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn greater(self, other: Self, scope: &Scope) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("greater", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("greater", element)),
        };

        value_1
            .greater(&value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn lesser(self, other: Self, scope: &Scope) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("lesser", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("lesser", element)),
        };

        value_1
            .lesser(&value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn add(self, other: Self, scope: &Scope) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("add", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("add", element)),
        };

        value_1.add(&value_2).map(Self::Value).map_err(Error::Value)
    }

    pub fn subtract(self, other: Self, scope: &Scope) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("subtract", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("subtract", element)),
        };

        value_1
            .subtract(&value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn multiply(self, other: Self, scope: &Scope) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("multiply", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("multiply", element)),
        };

        value_1
            .multiply(&value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn divide(self, other: Self, scope: &Scope) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("divide", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("divide", element)),
        };

        value_1
            .divide(&value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn remainder(self, other: Self, scope: &Scope) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("modulo", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("modulo", element)),
        };

        value_1
            .remainder(&value_2)
            .map(Self::Value)
            .map_err(Error::Value)
    }

    pub fn cast(self, other: Self, scope: &Scope) -> Result<(bool, usize), Error> {
        let value = match self {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("cast", element)),
        };

        let r#type = match other {
            Self::Type(r#type) => r#type,
            element => return Err(Error::ExpectedTypeExpression("cast", element)),
        };

        value.cast(&r#type).map_err(Error::Value)
    }

    pub fn negate(self, scope: &Scope) -> Result<Self, Error> {
        let value = match self {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("negate", element)),
        };

        value.negate().map(Self::Value).map_err(Error::Value)
    }

    pub fn not(self, scope: &Scope) -> Result<Self, Error> {
        let value = match self {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            element => return Err(Error::ExpectedResolvableExpression("not", element)),
        };

        value.not().map(Self::Value).map_err(Error::Value)
    }

    pub fn index(self, other: Self) -> Result<Self, Error> {
        let mut place = match self {
            Self::Place(place) => place,
            element => return Err(Error::ExpectedPlaceExpression("index", element)),
        };

        let value = match other {
            Self::Value(value) => value,
            element => return Err(Error::ExpectedValueExpression("index", element)),
        };

        place.index(value).map_err(Error::Place)?;
        Ok(Self::Place(place))
    }

    pub fn field(self, other: Self) -> Result<Self, Error> {
        let mut place = match self {
            Self::Place(place) => place,
            element => return Err(Error::ExpectedPlaceExpression("field", element)),
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
            element => Err(Error::ExpectedResolvableExpression("field", element)),
        }
    }

    pub fn resolve(place: &Place, scope: &Scope) -> Result<Value, Error> {
        scope
            .get_declaration(place)
            .map(|declaration| Value::new(declaration.type_variant))
            .map_err(Error::Resolving)
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Place(place) => write!(f, "{}", place),
            Self::Value(value) => write!(f, "{}", value),
            Self::Type(r#type) => write!(f, "{}", r#type),
            Self::ValueList(values) => write!(
                f,
                "{}",
                values
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
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
