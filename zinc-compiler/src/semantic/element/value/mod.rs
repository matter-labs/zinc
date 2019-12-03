//!
//! The semantic analyzer value element.
//!

mod array;
mod error;
mod integer;
mod structure;
mod tuple;

pub use self::array::Array;
pub use self::array::Error as ArrayError;
pub use self::error::Error;
pub use self::integer::Error as IntegerError;
pub use self::integer::Integer;
pub use self::structure::Error as StructureError;
pub use self::structure::Structure;
pub use self::tuple::Tuple;

use std::fmt;

use crate::semantic::Constant;
use crate::semantic::Type;

#[derive(Clone, PartialEq)]
pub enum Value {
    Unit,
    Boolean,
    Integer(Integer),
    Array(Array),
    Tuple(Tuple),
    Structure(Structure),
}

impl Value {
    pub fn new(r#type: Type) -> Self {
        match r#type {
            Type::Unit => Self::Unit,
            Type::Boolean => Self::Boolean,
            Type::IntegerUnsigned { bitlength } => Self::Integer(Integer::new(false, bitlength)),
            Type::IntegerSigned { bitlength } => Self::Integer(Integer::new(true, bitlength)),
            Type::Field => Self::Integer(Integer::new(false, crate::BITLENGTH_FIELD)),
            Type::Array { r#type, size } => Self::Array(Array::new(*r#type, size)),
            Type::Tuple { types } => Self::Tuple(Tuple::new(types)),
            Type::Structure { index, fields } => Self::Structure(Structure::new(index, fields)),
            r#type => panic!(
                "{}{}",
                crate::semantic::PANIC_VALUE_CANNOT_BE_CREATED_FROM,
                r#type
            ),
        }
    }

    pub fn r#type(&self) -> Type {
        match self {
            Self::Unit => Type::Unit,
            Self::Boolean => Type::Boolean,
            Self::Integer(integer) => integer.r#type(),
            Self::Array(array) => array.r#type(),
            Self::Tuple(tuple) => tuple.r#type(),
            Self::Structure(structure) => structure.r#type(),
        }
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Unit, Self::Unit) => true,
            (Self::Boolean, Self::Boolean) => true,
            (Self::Integer(value_1), Self::Integer(value_2)) => {
                value_1.has_the_same_type_as(value_2)
            }
            (Self::Array(value_1), Self::Array(value_2)) => value_1.has_the_same_type_as(value_2),
            (Self::Tuple(value_1), Self::Tuple(value_2)) => value_1.has_the_same_type_as(value_2),
            (Self::Structure(value_1), Self::Structure(value_2)) => {
                value_1.has_the_same_type_as(value_2)
            }
            _ => false,
        }
    }

    pub fn assign(&self, other: &Self) -> Result<(), Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::AssignmentTypeMismatch(other.r#type(), self.r#type()));
        }

        Ok(())
    }

    pub fn or(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Boolean => match other {
                Self::Boolean => Ok(Self::Boolean),
                value => Err(Error::ExpectedBoolean("or", value.r#type())),
            },
            value => Err(Error::ExpectedBoolean("or", value.r#type())),
        }
    }

    pub fn xor(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Boolean => match other {
                Self::Boolean => Ok(Self::Boolean),
                value => Err(Error::ExpectedBoolean("xor", value.r#type())),
            },
            value => Err(Error::ExpectedBoolean("xor", value.r#type())),
        }
    }

    pub fn and(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Boolean => match other {
                Self::Boolean => Ok(Self::Boolean),
                value => Err(Error::ExpectedBoolean("and", value.r#type())),
            },
            value => Err(Error::ExpectedBoolean("and", value.r#type())),
        }
    }

    pub fn equals(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Self::Unit, Self::Unit) => Ok(Self::Boolean),
            (Self::Unit, value_2) => Err(Error::ExpectedUnit("equals", value_2.r#type())),
            (Self::Boolean, Self::Boolean) => Ok(Self::Boolean),
            (Self::Boolean, value_2) => Err(Error::ExpectedBoolean("equals", value_2.r#type())),
            (Self::Integer { .. }, Self::Integer { .. }) => Ok(Self::Boolean),
            (Self::Integer { .. }, value_2) => {
                Err(Error::ExpectedInteger("equals", value_2.r#type()))
            }
            (value_1, value_2) => Err(Error::ExpectedPrimitiveTypes(
                "equals",
                value_1.r#type(),
                value_2.r#type(),
            )),
        }
    }

    pub fn not_equals(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Self::Unit, Self::Unit) => Ok(Self::Boolean),
            (Self::Unit, value_2) => Err(Error::ExpectedUnit("not_equals", value_2.r#type())),
            (Self::Boolean, Self::Boolean) => Ok(Self::Boolean),
            (Self::Boolean, value_2) => Err(Error::ExpectedBoolean("not_equals", value_2.r#type())),
            (Self::Integer { .. }, Self::Integer { .. }) => Ok(Self::Boolean),
            (Self::Integer { .. }, value_2) => {
                Err(Error::ExpectedInteger("not_equals", value_2.r#type()))
            }
            (value_1, value_2) => Err(Error::ExpectedPrimitiveTypes(
                "not_equals",
                value_1.r#type(),
                value_2.r#type(),
            )),
        }
    }

    pub fn greater_equals(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .greater_equals(integer_2)
                    .map(|_| Self::Boolean)
                    .map_err(Error::Integer),
                value => Err(Error::ExpectedInteger("greater_equals", value.r#type())),
            },
            value => Err(Error::ExpectedInteger("greater_equals", value.r#type())),
        }
    }

    pub fn lesser_equals(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .lesser_equals(integer_2)
                    .map(|_| Self::Boolean)
                    .map_err(Error::Integer),
                value => Err(Error::ExpectedInteger("lesser_equals", value.r#type())),
            },
            value => Err(Error::ExpectedInteger("lesser_equals", value.r#type())),
        }
    }

    pub fn greater(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .greater(integer_2)
                    .map(|_| Self::Boolean)
                    .map_err(Error::Integer),
                value => Err(Error::ExpectedInteger("greater", value.r#type())),
            },
            value => Err(Error::ExpectedInteger("greater", value.r#type())),
        }
    }

    pub fn lesser(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .lesser(integer_2)
                    .map(|_| Self::Boolean)
                    .map_err(Error::Integer),
                value => Err(Error::ExpectedInteger("lesser", value.r#type())),
            },
            value => Err(Error::ExpectedInteger("lesser", value.r#type())),
        }
    }

    pub fn add(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .add(integer_2)
                    .map(|_| Self::Integer(integer_1.to_owned()))
                    .map_err(Error::Integer),
                value => Err(Error::ExpectedInteger("add", value.r#type())),
            },
            value => Err(Error::ExpectedInteger("add", value.r#type())),
        }
    }

    pub fn subtract(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .subtract(integer_2)
                    .map(|_| Self::Integer(integer_1.to_owned()))
                    .map_err(Error::Integer),
                value => Err(Error::ExpectedInteger("subtract", value.r#type())),
            },
            value => Err(Error::ExpectedInteger("subtract", value.r#type())),
        }
    }

    pub fn multiply(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .multiply(integer_2)
                    .map(|_| Self::Integer(integer_1.to_owned()))
                    .map_err(Error::Integer),
                value => Err(Error::ExpectedInteger("multiply", value.r#type())),
            },
            value => Err(Error::ExpectedInteger("multiply", value.r#type())),
        }
    }

    pub fn divide(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .divide(integer_2)
                    .map(|_| Self::Integer(integer_1.to_owned()))
                    .map_err(Error::Integer),
                value => Err(Error::ExpectedInteger("divide", value.r#type())),
            },
            value => Err(Error::ExpectedInteger("divide", value.r#type())),
        }
    }

    pub fn remainder(&self, other: &Self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer_1) => match other {
                Self::Integer(integer_2) => integer_1
                    .modulo(integer_2)
                    .map(|_| Self::Integer(integer_1.to_owned()))
                    .map_err(Error::Integer),
                value => Err(Error::ExpectedInteger("modulo", value.r#type())),
            },
            value => Err(Error::ExpectedInteger("modulo", value.r#type())),
        }
    }

    pub fn cast(&self, r#type: &Type) -> Result<(bool, usize), Error> {
        match self {
            Self::Integer(integer) => integer.cast(r#type).map_err(Error::Integer),
            value => Err(Error::ExpectedInteger("cast", value.r#type())),
        }
    }

    pub fn negate(&self) -> Result<Self, Error> {
        match self {
            Self::Integer(integer) => integer
                .negate()
                .map(|_| Self::Integer(integer.to_owned()))
                .map_err(Error::Integer),
            value => Err(Error::ExpectedInteger("negate", value.r#type())),
        }
    }

    pub fn not(&self) -> Result<Self, Error> {
        match self {
            Self::Boolean => Ok(Self::Boolean),
            value => Err(Error::ExpectedBoolean("not", value.r#type())),
        }
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "()"),
            Self::Boolean => write!(f, "bool"),
            Self::Integer(integer) => write!(f, "{}", integer),
            Self::Array(array) => write!(f, "{}", array),
            Self::Tuple(tuple) => write!(f, "{}", tuple),
            Self::Structure(structure) => write!(f, "{}", structure),
        }
    }
}

impl From<Constant> for Value {
    fn from(constant: Constant) -> Self {
        Self::new(constant.r#type())
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}
