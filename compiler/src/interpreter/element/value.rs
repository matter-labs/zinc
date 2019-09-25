//!
//! The interpreter value.
//!

use std::fmt;

use failure::Fail;
use num_bigint::BigInt;
use num_traits::Num;

use bellman::ConstraintSystem;
use ff::Field;
use ff::PrimeField;
use franklin_crypto::circuit::boolean::AllocatedBit;
use franklin_crypto::circuit::num::AllocatedNum;
use pairing::bn256::Bn256;
use pairing::bn256::Fr;

use crate::lexical::BooleanLiteral;
use crate::lexical::IntegerLiteral;
use crate::syntax::TypeVariant;

#[derive(Clone)]
pub enum Value {
    Void,
    Boolean(AllocatedBit),
    Integer(AllocatedNum<Bn256>),
}

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "synthesis: {}", _0)]
    Synthesis(String),
    #[fail(
        display = "operand types mismatch: [{}] and [{}] have different types",
        _0, _1
    )]
    OperandTypesMismatch(Value, Value),
    #[fail(display = "integer literal is larger than {} bits", _0)]
    IntegerLiteralIsTooLarge(usize),
    #[fail(display = "casting to invalid type: from [{}] to '{}'", _0, _1)]
    CastingToInvalidType(Value, TypeVariant),
    #[fail(display = "casting to lesser bitlength: from {} to {}", _0, _1)]
    CastingToLesserOrEqualBitlength(usize, usize),
}

impl Value {
    pub fn new_boolean<S: ConstraintSystem<Bn256>>(
        boolean: BooleanLiteral,
        system: &mut S,
    ) -> Result<Self, Error> {
        Ok(match boolean {
            BooleanLiteral::False => Self::Boolean(
                AllocatedBit::alloc(system, Some(false))
                    .map_err(|error| Error::Synthesis(error.to_string()))?,
            ),
            BooleanLiteral::True => Self::Boolean(
                AllocatedBit::alloc(system, Some(true))
                    .map_err(|error| Error::Synthesis(error.to_string()))?,
            ),
        })
    }

    pub fn new_integer<S: ConstraintSystem<Bn256>>(
        integer: IntegerLiteral,
        system: &mut S,
    ) -> Result<Self, Error> {
        let (string, base) = match integer {
            IntegerLiteral::Decimal { value } => (value, 10),
            IntegerLiteral::Hexadecimal { value } => (value, 16),
        };

        let value = BigInt::from_str_radix(&string, base).expect("Integer literal parsing bug");
        let mut bitlength = 8;
        let mut exponent = BigInt::from(256);
        while value >= exponent {
            if bitlength == 248 {
                exponent *= 64;
                bitlength += 6;
            } else if bitlength == 254 {
                return Err(Error::IntegerLiteralIsTooLarge(bitlength));
            } else {
                exponent *= 256;
                bitlength += 8;
            }
        }

        Ok(Self::Integer(
            AllocatedNum::alloc(system, || Ok(Fr::from_str(&string).unwrap()))
                .map_err(|error| Error::Synthesis(error.to_string()))?,
        ))
    }

    pub fn new_from_type<S: ConstraintSystem<Bn256>>(
        type_variant: TypeVariant,
        system: &mut S,
    ) -> Result<Self, Error> {
        Ok(match type_variant {
            TypeVariant::Void => Self::Void,
            TypeVariant::Bool => Self::Boolean(
                AllocatedBit::alloc(system, Some(false))
                    .map_err(|error| Error::Synthesis(error.to_string()))?,
            ),
            TypeVariant::Int { bitlength } => Self::Integer(
                AllocatedNum::alloc(system, || Ok(Fr::zero()))
                    .map_err(|error| Error::Synthesis(error.to_string()))?,
            ),
            TypeVariant::Uint { bitlength } => Self::Integer(
                AllocatedNum::alloc(system, || Ok(Fr::zero()))
                    .map_err(|error| Error::Synthesis(error.to_string()))?,
            ),
            TypeVariant::Field => Self::Integer(
                AllocatedNum::alloc(system, || Ok(Fr::zero()))
                    .map_err(|error| Error::Synthesis(error.to_string()))?,
            ),
        })
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Void, Self::Void) => true,
            (Self::Boolean(..), Self::Boolean(..)) => true,
            (Self::Integer(..), Self::Integer(..)) => true,
            _ => false,
        }
    }

    pub fn equal<S: ConstraintSystem<Bn256>>(
        &self,
        other: &Self,
        system: &mut S,
    ) -> Result<AllocatedBit, Error> {
        if !self.has_the_same_type_as(other) {
            return Err(Error::OperandTypesMismatch(
                self.to_owned(),
                other.to_owned(),
            ));
        }

        unimplemented!();
        //        AllocatedBit::alloc(system, Some(self == other))
        //            .map_err(|error| Error::Synthesis(error.to_string()))
    }

    pub fn not_equal<S: ConstraintSystem<Bn256>>(
        &self,
        other: &Self,
        system: &mut S,
    ) -> Result<AllocatedBit, Error> {
        if !self.has_the_same_type_as(other) {
            return Err(Error::OperandTypesMismatch(
                self.to_owned(),
                other.to_owned(),
            ));
        }

        unimplemented!();
        //        AllocatedBit::alloc(system, Some(self != other))
        //            .map_err(|error| Error::Synthesis(error.to_string()))
    }
}

impl PartialEq<Self> for Value {
    fn eq(&self, other: &Self) -> bool {
        unimplemented!()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Void => write!(f, "(): ()"),
            Self::Boolean(..) => write!(f, "bool"),
            Self::Integer(..) => write!(f, "integer"),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Void => write!(f, "(): ()"),
            Self::Boolean(..) => write!(f, "bool"),
            Self::Integer(..) => write!(f, "integer"),
        }
    }
}
