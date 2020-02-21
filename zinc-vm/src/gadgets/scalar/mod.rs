mod scalar_type;
pub use scalar_type::*;

use std::fmt;
use bellman::{Variable, ConstraintSystem};
use num_traits::ToPrimitive;

use ff::Field;
use crate::{Result, Engine, RuntimeError};
use crate::gadgets::utils;
use franklin_crypto::circuit::num::AllocatedNum;
use franklin_crypto::bellman::{SynthesisError, LinearCombination};
use num_bigint::{ToBigInt, BigInt};
use franklin_crypto::circuit::boolean::Boolean;
use franklin_crypto::circuit::Assignment;
use franklin_crypto::circuit::expression::Expression;

/// Scalar is a primitive value that can be stored on the stack and operated by VM's instructions.
#[derive(Debug, Clone)]
pub struct Scalar<E: Engine> {
    variant: ScalarVariant<E>,
    scalar_type: ScalarType,
}

#[derive(Debug, Clone)]
enum ScalarVariant<E: Engine> {
    Constant(ScalarConstant<E>),
    Variable(ScalarVariable<E>),
}

impl<E: Engine> From<ScalarConstant<E>> for ScalarVariant<E> {
    fn from(constant: ScalarConstant<E>) -> Self {
        Self::Constant(constant)
    }
}

impl<E: Engine> From<ScalarVariable<E>> for ScalarVariant<E> {
    fn from(variable: ScalarVariable<E>) -> Self {
        Self::Variable(variable)
    }
}

#[derive(Debug, Clone)]
struct ScalarConstant<E: Engine> {
    value: E::Fr
}

#[derive(Debug, Clone)]
struct ScalarVariable<E: Engine> {
    value: Option<E::Fr>,
    variable: Variable,
}

impl<E: Engine> Scalar<E> {
    pub fn new_unchecked_constant(value: E::Fr, scalar_type: ScalarType) -> Self {
        Self {
            variant: ScalarConstant { value }.into(),
            scalar_type,
        }
    }

    pub fn new_unchecked_variable(value: Option<E::Fr>, variable: Variable, scalar_type: ScalarType) -> Self {
        Self {
            variant: ScalarVariable { value, variable }.into(),
            scalar_type,
        }
    }

    pub fn to_expression<CS: ConstraintSystem<E>>(&self) -> Expression<E> {
        Expression::new(
            self.get_value(),
            self.lc::<CS>()
        )
    }

    pub fn get_type(&self) -> ScalarType {
        self.scalar_type
    }

    pub fn get_value(&self) -> Option<E::Fr> {
        match &self.variant {
            ScalarVariant::Constant(constant) => Some(constant.value),
            ScalarVariant::Variable(variable) => variable.value,
        }
    }

    pub fn grab_value(&self) -> std::result::Result<E::Fr, SynthesisError> {
        self.get_value().grab()
    }

    pub fn get_constant(&self) -> Result<E::Fr> {
        match &self.variant {
            ScalarVariant::Constant(constant) => Ok(constant.value),
            _ => Err(RuntimeError::ExpectedConstant),
        }
    }

    pub fn get_constant_usize(&self) -> Result<usize> {
        let fr = self.get_constant()?;
        let bigint = utils::fr_to_bigint(&fr, false);
        bigint
            .to_usize()
            .ok_or_else(|| RuntimeError::ExpectedUsize(bigint))
    }

    pub fn as_field(&self) -> Self {
        Self {
            variant: self.variant.clone(),
            scalar_type: ScalarType::Field,
        }
    }

    pub fn is_signed(&self) -> bool {
        self.scalar_type.is_signed()
    }

    pub fn lc<CS: ConstraintSystem<E>>(&self) -> LinearCombination<E> {
        match &self.variant {
            ScalarVariant::Constant(constant) => {
                LinearCombination::zero() + (constant.value, CS::one())
            },
            ScalarVariant::Variable(variable) => {
                LinearCombination::zero() + variable.variable
            },
        }
    }

    #[allow(dead_code)]
    pub fn get_bits<CS: ConstraintSystem<E>>(&self, mut cs: CS) -> Result<Vec<Self>> {
        let num = self.to_expression::<CS>();
        let bits = match self.scalar_type {
            ScalarType::Field => num.into_bits_le_strict(
                cs.namespace(|| "into_bits_le_strict")
            ),
            scalar_type => num.into_bits_le_fixed(
                cs.namespace(|| "into_bits_le_fixed"),
                scalar_type.bit_length::<E>()
            )
        }?;

        bits
            .into_iter()
            .enumerate()
            .map(|(i, bit)| Self::from_boolean(
                cs.namespace(|| format!("bit {}", i)),
                bit,
            ))
            .collect()
    }

    pub fn with_type_unchecked(self, scalar_type: ScalarType) -> Self {
        Self {
            variant: self.variant,
            scalar_type
        }
    }

    pub fn from_boolean<CS: ConstraintSystem<E>>(mut cs: CS, boolean: Boolean) -> Result<Self> {
        match boolean {
            Boolean::Is(bit) => Ok(Self::new_unchecked_variable(
                bit.get_value_field::<E>(),
                bit.get_variable(),
                ScalarType::Boolean
            )),
            Boolean::Not(bit) => {
                let expr = Expression::constant::<CS>(E::Fr::one()) - Expression::from(&bit);
                let num = expr.into_number(cs.namespace(|| "into_number"))?;
                let scalar = Self::from(num);
                Ok(scalar.with_type_unchecked(ScalarType::Boolean))
            },
            Boolean::Constant(_) => Ok(Self::new_unchecked_constant(
                boolean.get_value_field::<E>().unwrap(),
                ScalarType::Boolean,
            )),
        }
    }

}

//impl<E: Engine> fmt::Debug for Scalar<E> {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        let value_str = self
//            .value
//            .map(|f| utils::fr_to_bigint(&f, self.is_signed()).to_string())
//            .unwrap_or_else(|| "none".into());
//
//        write!(
//            f,
//            "Scalar {{ value: {}, type: {} }}",
//            value_str, self.scalar_type
//        )
//    }
//}

impl<E: Engine> fmt::Display for Scalar<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value_str = self
            .get_value()
            .map(|f| utils::fr_to_bigint(&f, self.is_signed()).to_string())
            .unwrap_or_else(|| "none".into());

        write!(f, "{} as {}", value_str, self.scalar_type)
    }
}

impl<E: Engine> ToBigInt for Scalar<E> {
    fn to_bigint(&self) -> Option<BigInt> {
        self.get_value().map(|fr| utils::fr_to_bigint(&fr, self.is_signed()))
    }
}

impl<E: Engine> From<AllocatedNum<E>> for Scalar<E> {
    fn from(num: AllocatedNum<E>) -> Self {
        Self {
            variant: ScalarVariable {
                value: num.get_value(),
                variable: num.get_variable(),
            }.into(),
            scalar_type: ScalarType::Field
        }
    }
}
