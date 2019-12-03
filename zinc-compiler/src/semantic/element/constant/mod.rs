//!
//! The semantic analyzer constant element.
//!

mod error;

pub use self::error::Error;

use std::convert::TryFrom;
use std::fmt;

use num_bigint::BigInt;
use num_traits::One;
use num_traits::ToPrimitive;
use num_traits::Zero;

use zinc_bytecode::PushConst;

use crate::semantic;
use crate::semantic::InferenceError;
use crate::semantic::Type;
use crate::syntax::BooleanLiteral;
use crate::syntax::IntegerLiteral;

#[derive(Clone, PartialEq)]
pub struct Constant {
    value: BigInt,
    is_signed: bool,
    bitlength: usize,
}

impl Constant {
    pub fn new(value: BigInt, is_signed: bool, bitlength: usize) -> Self {
        Self {
            value,
            is_signed,
            bitlength,
        }
    }

    pub fn new_one(bitlength: usize) -> Self {
        Self {
            value: BigInt::one(),
            is_signed: false,
            bitlength,
        }
    }

    pub fn new_range_bound(value: usize, bitlength: usize) -> Self {
        Self {
            value: BigInt::from(value),
            is_signed: false,
            bitlength,
        }
    }

    pub fn value(&self) -> BigInt {
        self.value.clone()
    }

    pub fn r#type(&self) -> Type {
        Type::new_numeric(self.is_signed, self.bitlength)
    }

    pub fn to_usize(&self) -> Result<usize, Error> {
        self.value
            .to_usize()
            .ok_or_else(|| Error::ConstantTooBigForIndex(self.value.to_owned()))
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.value, self.r#type())
    }
}

impl Into<PushConst> for Constant {
    fn into(self) -> PushConst {
        PushConst::new(self.value, self.is_signed, self.bitlength)
    }
}

impl From<bool> for Constant {
    fn from(value: bool) -> Self {
        Self {
            value: if value { BigInt::one() } else { BigInt::zero() },
            is_signed: false,
            bitlength: crate::BITLENGTH_BOOLEAN,
        }
    }
}

impl From<BooleanLiteral> for Constant {
    fn from(value: BooleanLiteral) -> Self {
        let value: bool = value.into();
        Self::from(value)
    }
}

impl From<(usize, usize)> for Constant {
    fn from((value, bitlength): (usize, usize)) -> Self {
        Self {
            value: BigInt::from(value),
            is_signed: false,
            bitlength,
        }
    }
}

impl TryFrom<IntegerLiteral> for Constant {
    type Error = InferenceError;

    fn try_from(value: IntegerLiteral) -> Result<Self, Self::Error> {
        let (value, bitlength) = semantic::infer_integer_literal(&value)?;

        Ok(Self {
            value,
            is_signed: false,
            bitlength,
        })
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl fmt::Debug for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}
