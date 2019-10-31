//!
//! The semantic analyzer element value.
//!

mod error;
mod integer;

pub use self::error::Error;
pub use self::integer::Error as IntegerError;
pub use self::integer::Integer;

use std::fmt;

use crate::lexical::IntegerLiteral;
use crate::syntax::TypeVariant;

#[derive(Clone, PartialEq)]
pub enum Value {
    Boolean,
    Integer(Integer),
}

impl Value {
    pub fn new_integer_from_literal(literal: IntegerLiteral, bitlength: Option<usize>) -> Self {
        Self::Integer(Integer::new_from_literal(literal, bitlength))
    }

    pub fn new_integer_from_usize(bitlength: usize) -> Self {
        Self::Integer(Integer::new_from_usize(bitlength))
    }

    pub fn type_variant(&self) -> TypeVariant {
        match self {
            Self::Boolean => TypeVariant::new_boolean(),
            Self::Integer(value) => value.type_variant(),
        }
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Boolean, Self::Boolean) => true,
            (Self::Integer(value_1), Self::Integer(value_2)) => {
                value_1.has_the_same_type_as(value_2)
            }
            _ => false,
        }
    }

    pub fn or(self, other: Self) -> Result<Self, Error> {
        match self {
            Self::Boolean { .. } => {}
            value => return Err(Error::ExpectedBoolean("or", value.type_variant())),
        }

        match other {
            Self::Boolean { .. } => {}
            value => return Err(Error::ExpectedBoolean("or", value.type_variant())),
        }

        Ok(Self::Boolean)
    }

    pub fn xor(self, other: Self) -> Result<Self, Error> {
        match self {
            Self::Boolean { .. } => {}
            value => return Err(Error::ExpectedBoolean("xor", value.type_variant())),
        }

        match other {
            Self::Boolean { .. } => {}
            value => return Err(Error::ExpectedBoolean("xor", value.type_variant())),
        }

        Ok(Self::Boolean)
    }

    pub fn and(self, other: Self) -> Result<Self, Error> {
        match self {
            Self::Boolean { .. } => {}
            value => return Err(Error::ExpectedBoolean("and", value.type_variant())),
        }

        match other {
            Self::Boolean { .. } => {}
            value => return Err(Error::ExpectedBoolean("and", value.type_variant())),
        }

        Ok(Self::Boolean)
    }

    pub fn equals(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Self::Boolean, Self::Boolean) => Ok(Self::Boolean),
            (Self::Boolean, value_2) => {
                Err(Error::ExpectedBoolean("equals", value_2.type_variant()))
            }
            (Self::Integer { .. }, Self::Integer { .. }) => Ok(Self::Boolean),
            (Self::Integer { .. }, value_2) => {
                Err(Error::ExpectedInteger("equals", value_2.type_variant()))
            }
        }
    }

    pub fn not_equals(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Self::Boolean, Self::Boolean) => Ok(Self::Boolean),
            (Self::Boolean, value_2) => {
                Err(Error::ExpectedBoolean("not_equals", value_2.type_variant()))
            }
            (Self::Integer { .. }, Self::Integer { .. }) => Ok(Self::Boolean),
            (Self::Integer { .. }, value_2) => {
                Err(Error::ExpectedInteger("not_equals", value_2.type_variant()))
            }
        }
    }

    pub fn add(self, other: Self) -> Result<Self, Error> {
        let integer = match self {
            Self::Integer(integer) => integer,
            value => return Err(Error::ExpectedInteger("add", value.type_variant())),
        };

        match other {
            Self::Integer { .. } => {}
            value => return Err(Error::ExpectedInteger("add", value.type_variant())),
        }

        Ok(Self::Integer(integer))
    }

    pub fn subtract(self, other: Self) -> Result<Self, Error> {
        let integer = match self {
            Self::Integer(integer) => integer,
            value => return Err(Error::ExpectedInteger("subtract", value.type_variant())),
        };

        match other {
            Self::Integer { .. } => {}
            value => return Err(Error::ExpectedInteger("subtract", value.type_variant())),
        }

        Ok(Self::Integer(integer))
    }

    pub fn multiply(self, other: Self) -> Result<Self, Error> {
        let integer = match self {
            Self::Integer(integer) => integer,
            value => return Err(Error::ExpectedInteger("multiply", value.type_variant())),
        };

        match other {
            Self::Integer { .. } => {}
            value => return Err(Error::ExpectedInteger("multiply", value.type_variant())),
        }

        Ok(Self::Integer(integer))
    }

    pub fn divide(self, other: Self) -> Result<Self, Error> {
        let integer = match self {
            Self::Integer(integer) => integer,
            value => return Err(Error::ExpectedInteger("divide", value.type_variant())),
        };

        match other {
            Self::Integer { .. } => {}
            value => return Err(Error::ExpectedInteger("divide", value.type_variant())),
        }

        Ok(Self::Integer(integer))
    }

    pub fn modulo(self, other: Self) -> Result<Self, Error> {
        let integer = match self {
            Self::Integer(integer) => integer,
            value => return Err(Error::ExpectedInteger("modulo", value.type_variant())),
        };

        match other {
            Self::Integer { .. } => {}
            value => return Err(Error::ExpectedInteger("modulo", value.type_variant())),
        }

        Ok(Self::Integer(integer))
    }

    pub fn negate(self) -> Result<Self, Error> {
        let integer = match self {
            Self::Integer(integer) => integer,
            value => return Err(Error::ExpectedInteger("negate", value.type_variant())),
        };

        Ok(Self::Integer(integer))
    }

    pub fn not(self) -> Result<Self, Error> {
        match self {
            Self::Boolean { .. } => {}
            value => return Err(Error::ExpectedBoolean("not", value.type_variant())),
        }

        Ok(Self::Boolean)
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean => write!(f, "boolean"),
            Self::Integer(integer) => write!(f, "{}", integer),
        }
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
