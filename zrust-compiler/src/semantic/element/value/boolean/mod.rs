//!
//! The semantic analyzer element boolean value.
//!

mod error;

pub use self::error::Error;

use std::fmt;

use zrust_bytecode::Push;

use crate::lexical::BooleanLiteral;

#[derive(Default, Clone)]
pub struct Boolean {
    pub value: Option<bool>,
}

impl Boolean {
    pub fn new_from_bool(value: bool) -> Self {
        Self { value: Some(value) }
    }

    pub fn new_from_literal(literal: BooleanLiteral) -> Self {
        Self {
            value: Some(literal.into()),
        }
    }

    pub fn or(self, other: Self) -> Self {
        match (self.value, other.value) {
            (Some(value_1), Some(value_2)) => Self::new_from_bool(value_1 || value_2),
            _ => Self::default(),
        }
    }

    pub fn xor(self, other: Self) -> Self {
        match (self.value, other.value) {
            (Some(value_1), Some(value_2)) => {
                Self::new_from_bool((!value_1 && value_2) || (value_1 && !value_2))
            }
            _ => Self::default(),
        }
    }

    pub fn and(self, other: Self) -> Self {
        match (self.value, other.value) {
            (Some(value_1), Some(value_2)) => Self::new_from_bool(value_1 && value_2),
            _ => Self::default(),
        }
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl PartialEq<Self> for Boolean {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Into<Push> for Boolean {
    fn into(self) -> Push {
        Push::new(
            false,
            crate::BITLENGTH_BYTE,
            vec![self.value.expect("Must contain a value") as u8],
        )
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl fmt::Debug for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}
