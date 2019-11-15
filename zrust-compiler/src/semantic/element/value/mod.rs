//!
//! The semantic analyzer element value.
//!

mod boolean;
mod error;
mod integer;

pub use self::boolean::Boolean;
pub use self::error::Error;
pub use self::integer::Error as IntegerError;
pub use self::integer::Integer;

use std::convert::TryFrom;
use std::fmt;

use zrust_bytecode::Push;

use crate::lexical::BooleanLiteral;
use crate::lexical::IntegerLiteral;
use crate::syntax::TypeVariant;

#[derive(Clone, PartialEq)]
pub enum Value {
    Boolean(Boolean),
    Integer(Integer),
}

impl Value {
    pub fn new(type_variant: TypeVariant) -> Self {
        match type_variant {
            TypeVariant::Boolean { .. } => Self::Boolean(Boolean::default()),
            TypeVariant::IntegerUnsigned { bitlength } => {
                Self::Integer(Integer::new(false, bitlength))
            }
            TypeVariant::IntegerSigned { bitlength } => {
                Self::Integer(Integer::new(true, bitlength))
            }
            TypeVariant::Field => Self::Integer(Integer::new(false, crate::BITLENGTH_FIELD)),
            _ => panic!("Always checked by some branches above"),
        }
    }

    pub fn type_variant(&self) -> TypeVariant {
        match self {
            Self::Boolean(boolean) => boolean.type_variant(),
            Self::Integer(integer) => integer.type_variant(),
        }
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Boolean { .. }, Self::Boolean { .. }) => true,
            (Self::Integer(value_1), Self::Integer(value_2)) => {
                value_1.has_the_same_type_as(value_2)
            }
            _ => false,
        }
    }

    pub fn or(self, other: Self) -> Result<Self, Error> {
        let boolean_1 = match self {
            Self::Boolean(boolean) => boolean,
            value => return Err(Error::ExpectedBoolean("or", value.type_variant())),
        };

        let boolean_2 = match other {
            Self::Boolean(boolean) => boolean,
            value => return Err(Error::ExpectedBoolean("or", value.type_variant())),
        };

        Ok(Self::Boolean(boolean_1.or(boolean_2)))
    }

    pub fn xor(self, other: Self) -> Result<Self, Error> {
        let boolean_1 = match self {
            Self::Boolean(boolean) => boolean,
            value => return Err(Error::ExpectedBoolean("xor", value.type_variant())),
        };

        let boolean_2 = match other {
            Self::Boolean(boolean) => boolean,
            value => return Err(Error::ExpectedBoolean("xor", value.type_variant())),
        };

        Ok(Self::Boolean(boolean_1.xor(boolean_2)))
    }

    pub fn and(self, other: Self) -> Result<Self, Error> {
        let boolean_1 = match self {
            Self::Boolean(boolean) => boolean,
            value => return Err(Error::ExpectedBoolean("and", value.type_variant())),
        };

        let boolean_2 = match other {
            Self::Boolean(boolean) => boolean,
            value => return Err(Error::ExpectedBoolean("and", value.type_variant())),
        };

        Ok(Self::Boolean(boolean_1.and(boolean_2)))
    }

    pub fn equals(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Self::Boolean { .. }, Self::Boolean { .. }) => Ok(Self::Boolean(Boolean::default())),
            (Self::Boolean { .. }, value_2) => {
                Err(Error::ExpectedBoolean("equals", value_2.type_variant()))
            }
            (Self::Integer { .. }, Self::Integer { .. }) => Ok(Self::Boolean(Boolean::default())),
            (Self::Integer { .. }, value_2) => {
                Err(Error::ExpectedInteger("equals", value_2.type_variant()))
            }
        }
    }

    pub fn not_equals(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Self::Boolean { .. }, Self::Boolean { .. }) => Ok(Self::Boolean(Boolean::default())),
            (Self::Boolean { .. }, value_2) => {
                Err(Error::ExpectedBoolean("not_equals", value_2.type_variant()))
            }
            (Self::Integer { .. }, Self::Integer { .. }) => Ok(Self::Boolean(Boolean::default())),
            (Self::Integer { .. }, value_2) => {
                Err(Error::ExpectedInteger("not_equals", value_2.type_variant()))
            }
        }
    }

    pub fn greater_equals(self, other: Self) -> Result<Self, Error> {
        let integer_1 = match self {
            Self::Integer(integer) => integer,
            value => {
                return Err(Error::ExpectedInteger(
                    "greater_equals",
                    value.type_variant(),
                ))
            }
        };

        let integer_2 = match other {
            Self::Integer(integer) => integer,
            value => {
                return Err(Error::ExpectedInteger(
                    "greater_equals",
                    value.type_variant(),
                ))
            }
        };

        integer_1
            .greater_equals(integer_2)
            .map(Self::Boolean)
            .map_err(Error::Integer)
    }

    pub fn lesser_equals(self, other: Self) -> Result<Self, Error> {
        let integer_1 = match self {
            Self::Integer(integer) => integer,
            value => {
                return Err(Error::ExpectedInteger(
                    "lesser_equals",
                    value.type_variant(),
                ))
            }
        };

        let integer_2 = match other {
            Self::Integer(integer) => integer,
            value => {
                return Err(Error::ExpectedInteger(
                    "lesser_equals",
                    value.type_variant(),
                ))
            }
        };

        integer_1
            .lesser_equals(integer_2)
            .map(Self::Boolean)
            .map_err(Error::Integer)
    }

    pub fn greater(self, other: Self) -> Result<Self, Error> {
        let integer_1 = match self {
            Self::Integer(integer) => integer,
            value => return Err(Error::ExpectedInteger("greater", value.type_variant())),
        };

        let integer_2 = match other {
            Self::Integer(integer) => integer,
            value => return Err(Error::ExpectedInteger("greater", value.type_variant())),
        };

        integer_1
            .greater(integer_2)
            .map(Self::Boolean)
            .map_err(Error::Integer)
    }

    pub fn lesser(self, other: Self) -> Result<Self, Error> {
        let integer_1 = match self {
            Self::Integer(integer) => integer,
            value => return Err(Error::ExpectedInteger("lesser", value.type_variant())),
        };

        let integer_2 = match other {
            Self::Integer(integer) => integer,
            value => return Err(Error::ExpectedInteger("lesser", value.type_variant())),
        };

        integer_1
            .lesser(integer_2)
            .map(Self::Boolean)
            .map_err(Error::Integer)
    }

    pub fn add(self, other: Self) -> Result<Self, Error> {
        let integer_1 = match self {
            Self::Integer(integer) => integer,
            value => return Err(Error::ExpectedInteger("add", value.type_variant())),
        };

        let integer_2 = match other {
            Self::Integer(integer) => integer,
            value => return Err(Error::ExpectedInteger("add", value.type_variant())),
        };

        integer_1
            .add(integer_2)
            .map(Self::Integer)
            .map_err(Error::Integer)
    }

    pub fn subtract(self, other: Self) -> Result<Self, Error> {
        let integer_1 = match self {
            Self::Integer(integer) => integer,
            value => return Err(Error::ExpectedInteger("subtract", value.type_variant())),
        };

        let integer_2 = match other {
            Self::Integer(integer) => integer,
            value => return Err(Error::ExpectedInteger("subtract", value.type_variant())),
        };

        integer_1
            .subtract(integer_2)
            .map(Self::Integer)
            .map_err(Error::Integer)
    }

    pub fn multiply(self, other: Self) -> Result<Self, Error> {
        let integer_1 = match self {
            Self::Integer(integer) => integer,
            value => return Err(Error::ExpectedInteger("multiply", value.type_variant())),
        };

        let integer_2 = match other {
            Self::Integer(integer) => integer,
            value => return Err(Error::ExpectedInteger("multiply", value.type_variant())),
        };

        integer_1
            .multiply(integer_2)
            .map(Self::Integer)
            .map_err(Error::Integer)
    }

    pub fn divide(self, other: Self) -> Result<Self, Error> {
        let integer_1 = match self {
            Self::Integer(integer) => integer,
            value => return Err(Error::ExpectedInteger("divide", value.type_variant())),
        };

        let integer_2 = match other {
            Self::Integer(integer) => integer,
            value => return Err(Error::ExpectedInteger("divide", value.type_variant())),
        };

        integer_1
            .divide(integer_2)
            .map(Self::Integer)
            .map_err(Error::Integer)
    }

    pub fn modulo(self, other: Self) -> Result<Self, Error> {
        let integer_1 = match self {
            Self::Integer(integer) => integer,
            value => return Err(Error::ExpectedInteger("modulo", value.type_variant())),
        };

        let integer_2 = match other {
            Self::Integer(integer) => integer,
            value => return Err(Error::ExpectedInteger("modulo", value.type_variant())),
        };

        integer_1
            .modulo(integer_2)
            .map(Self::Integer)
            .map_err(Error::Integer)
    }

    pub fn cast(self, type_variant: TypeVariant) -> Result<(Self, usize), Error> {
        let integer = match self {
            Self::Integer(value) => value,
            value => return Err(Error::ExpectedInteger("cast", value.type_variant())),
        };

        integer
            .cast(type_variant)
            .map(|(integer, bitlength)| (Self::Integer(integer), bitlength))
            .map_err(Error::Integer)
    }

    pub fn negate(self) -> Result<Self, Error> {
        let integer = match self {
            Self::Integer(integer) => integer,
            value => return Err(Error::ExpectedInteger("negate", value.type_variant())),
        };

        integer.negate().map(Self::Integer).map_err(Error::Integer)
    }

    pub fn not(self) -> Result<Self, Error> {
        match self {
            Self::Boolean(boolean) => Ok(Self::Boolean(boolean.not())),
            value => Err(Error::ExpectedBoolean("not", value.type_variant())),
        }
    }

    pub fn to_push(&self) -> Push {
        match self {
            Self::Boolean(value) => value.to_push(),
            Self::Integer(value) => value.to_push(),
        }
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(boolean) => write!(f, "{:?}", boolean),
            Self::Integer(integer) => write!(f, "{}", integer),
        }
    }
}

impl From<BooleanLiteral> for Value {
    fn from(value: BooleanLiteral) -> Self {
        Self::Boolean(Boolean::from(value))
    }
}

impl TryFrom<IntegerLiteral> for Value {
    type Error = Error;

    fn try_from(value: IntegerLiteral) -> Result<Self, Self::Error> {
        Integer::try_from(value)
            .map(Self::Integer)
            .map_err(Error::Integer)
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
