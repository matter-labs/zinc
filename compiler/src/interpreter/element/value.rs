//!
//! The interpreter value.
//!

use std::fmt;

use failure::Fail;
use num_bigint::BigInt;
use num_traits::Num;

use bellman::ConstraintSystem;
use ff::PrimeField;
use franklin_crypto::circuit::boolean::AllocatedBit;
use franklin_crypto::circuit::boolean::Boolean;
use franklin_crypto::circuit::num::AllocatedNum;
use pairing::bn256::Bn256;
use pairing::bn256::Fr;

use crate::interpreter::Integer;
use crate::lexical::BooleanLiteral;
use crate::lexical::IntegerLiteral;
use crate::syntax::Type;
use crate::syntax::TypeVariant;

#[derive(Clone)]
pub enum Value {
    Void,
    Boolean(Boolean),
    Integer(Integer),
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
    pub fn new_boolean<CS: ConstraintSystem<Bn256>>(
        boolean: BooleanLiteral,
        mut system: CS,
    ) -> Result<Self, Error> {
        Ok(match boolean {
            BooleanLiteral::False => Self::Boolean(Boolean::from(
                AllocatedBit::alloc(system.namespace(|| "value_new_boolean"), Some(false))
                    .map_err(|error| Error::Synthesis(error.to_string()))?,
            )),
            BooleanLiteral::True => Self::Boolean(Boolean::from(
                AllocatedBit::alloc(system.namespace(|| "value_new_boolean"), Some(true))
                    .map_err(|error| Error::Synthesis(error.to_string()))?,
            )),
        })
    }

    pub fn new_integer<CS: ConstraintSystem<Bn256>>(
        integer: IntegerLiteral,
        mut system: CS,
    ) -> Result<Self, Error> {
        let (string, base) = match integer {
            IntegerLiteral::Decimal { value } => (value, 10),
            IntegerLiteral::Hexadecimal { value } => (value, 16),
        };

        let value = BigInt::from_str_radix(&string, base).expect("Always valid");
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

        let number = AllocatedNum::alloc(system.namespace(|| "value_new_integer"), || {
            Ok(Fr::from_str(&value.to_string()).expect("Always valid"))
        })
        .map_err(|error| Error::Synthesis(error.to_string()))?;

        Ok(Self::Integer(Integer::new(number, false, bitlength)))
    }

    pub fn new_input<CS: ConstraintSystem<Bn256>>(
        r#type: Type,
        mut system: CS,
    ) -> Result<Self, Error> {
        Ok(match r#type.variant {
            TypeVariant::Void => Self::Void,
            TypeVariant::Bool => Self::Boolean(
                jab::allocate_input(
                    system.namespace(|| "value_new_input"),
                    || Ok(Fr::from_str("0").expect("Always valid")),
                    1,
                )
                .map_err(|error| Error::Synthesis(error.to_string()))?
                .1
                .pop()
                .expect("Always contains an element"),
            ),
            TypeVariant::Int { bitlength } => Self::Integer(Integer::new(
                jab::allocate_input(
                    system.namespace(|| "value_new_input"),
                    || Ok(Fr::from_str("0").expect("Always valid")),
                    bitlength,
                )
                .map_err(|error| Error::Synthesis(error.to_string()))?
                .0,
                true,
                bitlength,
            )),
            TypeVariant::Uint { bitlength } => Self::Integer(Integer::new(
                jab::allocate_input(
                    system.namespace(|| "value_new_input"),
                    || Ok(Fr::from_str("0").expect("Always valid")),
                    bitlength,
                )
                .map_err(|error| Error::Synthesis(error.to_string()))?
                .0,
                false,
                bitlength,
            )),
            TypeVariant::Field => Self::Integer(Integer::new(
                jab::allocate_input(
                    system.namespace(|| "value_new_input"),
                    || Ok(Fr::from_str("0").expect("Always valid")),
                    254,
                )
                .map_err(|error| Error::Synthesis(error.to_string()))?
                .0,
                false,
                254,
            )),
        })
    }

    pub fn new_witness<CS: ConstraintSystem<Bn256>>(
        r#type: Type,
        mut system: CS,
    ) -> Result<Self, Error> {
        Ok(match r#type.variant {
            TypeVariant::Void => Self::Void,
            TypeVariant::Bool => Self::Boolean(
                jab::allocate_witness(
                    system.namespace(|| "value_new_witness"),
                    || Ok(Fr::from_str("0").expect("Always valid")),
                    1,
                )
                .map_err(|error| Error::Synthesis(error.to_string()))?
                .1
                .pop()
                .expect("Always contains an element"),
            ),
            TypeVariant::Int { bitlength } => Self::Integer(Integer::new(
                jab::allocate_witness(
                    system.namespace(|| "value_new_witness"),
                    || Ok(Fr::from_str("0").expect("Always valid")),
                    bitlength,
                )
                .map_err(|error| Error::Synthesis(error.to_string()))?
                .0,
                true,
                bitlength,
            )),
            TypeVariant::Uint { bitlength } => Self::Integer(Integer::new(
                jab::allocate_witness(
                    system.namespace(|| "value_new_witness"),
                    || Ok(Fr::from_str("0").expect("Always valid")),
                    bitlength,
                )
                .map_err(|error| Error::Synthesis(error.to_string()))?
                .0,
                false,
                bitlength,
            )),
            TypeVariant::Field => Self::Integer(Integer::new(
                jab::allocate_witness(
                    system.namespace(|| "value_new_witness"),
                    || Ok(Fr::from_str("0").expect("Always valid")),
                    254,
                )
                .map_err(|error| Error::Synthesis(error.to_string()))?
                .0,
                false,
                254,
            )),
        })
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Void, Self::Void) => true,
            (Self::Boolean(..), Self::Boolean(..)) => true,
            (Self::Integer(integer_1), Self::Integer(integer_2)) => {
                integer_1.has_the_same_type_as(integer_2)
            }
            _ => false,
        }
    }

    pub fn equal<CS: ConstraintSystem<Bn256>>(
        &self,
        other: &Self,
        mut system: CS,
    ) -> Result<Boolean, Error> {
        if !self.has_the_same_type_as(other) {
            return Err(Error::OperandTypesMismatch(
                self.to_owned(),
                other.to_owned(),
            ));
        }

        Ok(Boolean::from(
            AllocatedBit::alloc(system.namespace(|| "value_equal"), Some(self == other))
                .map_err(|error| Error::Synthesis(error.to_string()))?,
        ))
    }

    pub fn not_equal<CS: ConstraintSystem<Bn256>>(
        &self,
        other: &Self,
        mut system: CS,
    ) -> Result<Boolean, Error> {
        if !self.has_the_same_type_as(other) {
            return Err(Error::OperandTypesMismatch(
                self.to_owned(),
                other.to_owned(),
            ));
        }

        Ok(Boolean::from(
            AllocatedBit::alloc(system.namespace(|| "value_not_equal"), Some(self != other))
                .map_err(|error| Error::Synthesis(error.to_string()))?,
        ))
    }
}

impl PartialEq<Self> for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Void, Self::Void) => true,
            (Self::Boolean(value_1), Self::Boolean(value_2)) => {
                value_1.get_value() == value_2.get_value()
            }
            (Self::Integer(value_1), Self::Integer(value_2)) => value_1.eq(value_2),
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Void => write!(f, "()"),
            Self::Boolean(boolean) => write!(
                f,
                "{}",
                boolean.get_value().expect("Always returns a value")
            ),
            Self::Integer(integer) => write!(f, "{}", integer),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Void => write!(f, "()"),
            Self::Boolean(boolean) => write!(
                f,
                "{}",
                boolean.get_value().expect("Always returns a value")
            ),
            Self::Integer(integer) => write!(f, "{}", integer),
        }
    }
}
