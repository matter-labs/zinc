//!
//! The semantic analyzer element.
//!

mod constant;
mod error;
mod place;
mod r#type;
mod value;

pub use self::constant::Constant;
pub use self::constant::Error as ConstantError;
pub use self::error::Error;
pub use self::place::Descriptor as PlaceDescriptor;
pub use self::place::Error as PlaceError;
pub use self::place::Place;
pub use self::r#type::Type;
pub use self::value::Array;
pub use self::value::ArrayError;
pub use self::value::Error as ValueError;
pub use self::value::Integer;
pub use self::value::IntegerError;
pub use self::value::Structure;
pub use self::value::StructureError;
pub use self::value::Tuple;
pub use self::value::Value;

use std::fmt;

use crate::semantic::Scope;

#[derive(Clone, PartialEq)]
pub enum Element {
    /// Memory descriptor a.k.a. `lvalue`
    Place(Place),
    /// Single operand representing a logical piece of data a.k.a. `rvalue`
    Value(Value),
    /// Type is used only as the second operand of the casting operator
    Type(Type),
    /// `rvalue` argument list is used only as the second operator of the function call operator
    ValueList(Vec<Value>),
    /// Constant value, which can be inserted into bytecode and then turned into an `rvalue`
    Constant(Constant),
}

impl Element {
    pub fn r#type(&self, scope: &Scope) -> Result<Type, Error> {
        Ok(match self {
            Self::Place(place) => Self::resolve(&place, scope)?.r#type(),
            Self::Value(value) => value.r#type(),
            Self::Type(r#type) => r#type.to_owned(),
            Self::ValueList(values) => {
                Type::new_tuple(values.iter().map(|value| value.r#type()).collect())
            }
            Self::Constant(constant) => constant.r#type(),
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
            Self::Constant(constant) => Value::from(constant),
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
            Self::Constant(constant) => Value::from(constant),
            element => return Err(Error::ExpectedResolvableExpression("or", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            Self::Constant(constant) => Value::from(constant),
            element => return Err(Error::ExpectedResolvableExpression("or", element)),
        };

        value_1.or(&value_2).map(Self::Value).map_err(Error::Value)
    }

    pub fn xor(self, other: Self, scope: &Scope) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            Self::Constant(constant) => Value::from(constant),
            element => return Err(Error::ExpectedResolvableExpression("xor", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            Self::Constant(constant) => Value::from(constant),
            element => return Err(Error::ExpectedResolvableExpression("xor", element)),
        };

        value_1.xor(&value_2).map(Self::Value).map_err(Error::Value)
    }

    pub fn and(self, other: Self, scope: &Scope) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            Self::Constant(constant) => Value::from(constant),
            element => return Err(Error::ExpectedResolvableExpression("and", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            Self::Constant(constant) => Value::from(constant),
            element => return Err(Error::ExpectedResolvableExpression("and", element)),
        };

        value_1.and(&value_2).map(Self::Value).map_err(Error::Value)
    }

    pub fn equals(self, other: Self, scope: &Scope) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            Self::Constant(constant) => Value::from(constant),
            element => return Err(Error::ExpectedResolvableExpression("equals", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            Self::Constant(constant) => Value::from(constant),
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
            Self::Constant(constant) => Value::from(constant),
            element => return Err(Error::ExpectedResolvableExpression("not_equals", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            Self::Constant(constant) => Value::from(constant),
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
            Self::Constant(constant) => Value::from(constant),
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
            Self::Constant(constant) => Value::from(constant),
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
            Self::Constant(constant) => Value::from(constant),
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
            Self::Constant(constant) => Value::from(constant),
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
            Self::Constant(constant) => Value::from(constant),
            element => return Err(Error::ExpectedResolvableExpression("greater", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            Self::Constant(constant) => Value::from(constant),
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
            Self::Constant(constant) => Value::from(constant),
            element => return Err(Error::ExpectedResolvableExpression("lesser", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            Self::Constant(constant) => Value::from(constant),
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
            Self::Constant(constant) => Value::from(constant),
            element => return Err(Error::ExpectedResolvableExpression("add", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            Self::Constant(constant) => Value::from(constant),
            element => return Err(Error::ExpectedResolvableExpression("add", element)),
        };

        value_1.add(&value_2).map(Self::Value).map_err(Error::Value)
    }

    pub fn subtract(self, other: Self, scope: &Scope) -> Result<Self, Error> {
        let value_1 = match self {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            Self::Constant(constant) => Value::from(constant),
            element => return Err(Error::ExpectedResolvableExpression("subtract", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            Self::Constant(constant) => Value::from(constant),
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
            Self::Constant(constant) => Value::from(constant),
            element => return Err(Error::ExpectedResolvableExpression("multiply", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            Self::Constant(constant) => Value::from(constant),
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
            Self::Constant(constant) => Value::from(constant),
            element => return Err(Error::ExpectedResolvableExpression("divide", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            Self::Constant(constant) => Value::from(constant),
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
            Self::Constant(constant) => Value::from(constant),
            element => return Err(Error::ExpectedResolvableExpression("modulo", element)),
        };

        let value_2 = match other {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            Self::Constant(constant) => Value::from(constant),
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
            Self::Constant(constant) => Value::from(constant),
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
            Self::Constant(constant) => Value::from(constant),
            element => return Err(Error::ExpectedResolvableExpression("negate", element)),
        };

        value.negate().map(Self::Value).map_err(Error::Value)
    }

    pub fn not(self, scope: &Scope) -> Result<Self, Error> {
        let value = match self {
            Self::Value(value) => value,
            Self::Place(place) => Self::resolve(&place, scope)?,
            Self::Constant(constant) => Value::from(constant),
            element => return Err(Error::ExpectedResolvableExpression("not", element)),
        };

        value.not().map(Self::Value).map_err(Error::Value)
    }

    pub fn index(self, other: Self) -> Result<Self, Error> {
        let mut place = match self {
            Self::Place(place) => place,
            element => return Err(Error::ExpectedPlaceExpression("index", element)),
        };

        let constant = match other {
            Self::Constant(constant) => constant,
            element => return Err(Error::ExpectedConstantExpression("index", element)),
        };

        place.index(constant).map_err(Error::Place)?;
        Ok(Self::Place(place))
    }

    pub fn field(self, other: Self) -> Result<Self, Error> {
        let mut place = match self {
            Self::Place(place) => place,
            element => return Err(Error::ExpectedPlaceExpression("field", element)),
        };

        match other {
            Self::Place(field) => {
                place.access_structure(field.identifier);
                Ok(Self::Place(place))
            }
            Self::Constant(constant) => {
                place.access_tuple(constant).map_err(Error::Place)?;
                Ok(Self::Place(place))
            }
            element => Err(Error::ExpectedConstantExpression("field", element)),
        }
    }

    pub fn resolve(place: &Place, scope: &Scope) -> Result<Value, Error> {
        scope
            .get_declaration(&place.identifier)
            .map(|declaration| Value::new(declaration.r#type))
            .map_err(Error::Resolving)
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Place(place) => write!(f, "{}", place),
            Self::Value(value) => write!(f, "{}", value),
            Self::Type(r#type) => write!(f, "{}", r#type),
            Self::Constant(constant) => write!(f, "{}", constant),
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
