pub mod constant;
pub mod expectation;
pub mod fr_bigint;
pub mod variable;
pub mod variant;

use std::fmt;

use num_bigint::BigInt;
use num_bigint::ToBigInt;
use num_traits::ToPrimitive;

use ff::Field;
use ff::PrimeField;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::bellman::LinearCombination;
use franklin_crypto::bellman::SynthesisError;
use franklin_crypto::bellman::Variable;
use franklin_crypto::circuit::boolean::AllocatedBit;
use franklin_crypto::circuit::boolean::Boolean;
use franklin_crypto::circuit::expression::Expression;
use franklin_crypto::circuit::num::AllocatedNum;
use franklin_crypto::circuit::Assignment;

use zinc_bytecode::ScalarType;

use crate::error::RuntimeError;
use crate::IEngine;

use self::constant::Constant as ScalarConstant;
use self::expectation::ITypeExpectation as ScalarTypeExpectation;
use self::variable::Variable as ScalarVariable;
use self::variant::Variant as ScalarVariant;

/// Scalar is a primitive value that can be stored on the stack and operated by VM's instructions.
#[derive(Debug, Clone)]
pub struct Scalar<E: IEngine> {
    variant: ScalarVariant<E>,
    scalar_type: ScalarType,
}

impl<E: IEngine> Scalar<E> {
    pub fn new_constant_int(value: usize, scalar_type: ScalarType) -> Self {
        let value_string = value.to_string();
        let fr = E::Fr::from_str(&value_string).expect("failed to convert u64 into Fr");
        Self::new_constant_fr(fr, scalar_type)
    }

    pub fn new_constant_bool(value: bool) -> Self {
        let fr = if value { E::Fr::one() } else { E::Fr::zero() };
        Self::new_constant_fr(fr, ScalarType::Boolean)
    }

    pub fn new_constant_fr(value: E::Fr, scalar_type: ScalarType) -> Self {
        Self {
            variant: ScalarConstant { value }.into(),
            scalar_type,
        }
    }

    pub fn new_constant_bigint(
        value: &BigInt,
        scalar_type: ScalarType,
    ) -> Result<Self, RuntimeError> {
        let fr = fr_bigint::bigint_to_fr::<E>(value).ok_or(RuntimeError::ValueOverflow {
            value: value.clone(),
            scalar_type: scalar_type.clone(),
        })?;
        Ok(Self::new_constant_fr(fr, scalar_type))
    }

    pub fn new_unchecked_variable(
        value: Option<E::Fr>,
        variable: Variable,
        scalar_type: ScalarType,
    ) -> Self {
        Self {
            variant: ScalarVariable { value, variable }.into(),
            scalar_type,
        }
    }

    pub fn to_expression<CS: ConstraintSystem<E>>(&self) -> Expression<E> {
        Expression::new(self.get_value(), self.lc::<CS>())
    }

    pub fn to_boolean<CS: ConstraintSystem<E>>(&self, mut cs: CS) -> Result<Boolean, RuntimeError> {
        self.scalar_type.assert_type(ScalarType::Boolean)?;

        match &self.variant {
            ScalarVariant::Constant(constant) => Ok(Boolean::constant(!constant.value.is_zero())),
            ScalarVariant::Variable(variable) => {
                let bit = AllocatedBit::alloc(
                    cs.namespace(|| "allocate bit"),
                    variable.value.map(|value| !value.is_zero()),
                )?;

                cs.enforce(
                    || "bit equality",
                    |zero| zero + bit.get_variable(),
                    |zero| zero + CS::one(),
                    |zero| zero + variable.variable,
                );

                Ok(bit.into())
            }
        }
    }

    pub fn get_type(&self) -> ScalarType {
        self.scalar_type.to_owned()
    }

    pub fn get_value(&self) -> Option<E::Fr> {
        match &self.variant {
            ScalarVariant::Constant(constant) => Some(constant.value.to_owned()),
            ScalarVariant::Variable(variable) => variable.value.to_owned(),
        }
    }

    pub fn get_variant(&self) -> &ScalarVariant<E> {
        &self.variant
    }

    pub fn grab_value(&self) -> Result<E::Fr, SynthesisError> {
        self.get_value().grab()
    }

    pub fn get_constant(&self) -> Result<E::Fr, RuntimeError> {
        match &self.variant {
            ScalarVariant::Constant(constant) => Ok(constant.value.to_owned()),
            _ => Err(RuntimeError::ExpectedConstant),
        }
    }

    pub fn get_constant_usize(&self) -> Result<usize, RuntimeError> {
        let fr = self.get_constant()?;
        let bigint = fr_bigint::fr_to_bigint(&fr, false);
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

    pub fn is_constant(&self) -> bool {
        match self.variant {
            ScalarVariant::Constant(_) => true,
            ScalarVariant::Variable(_) => false,
        }
    }

    pub fn lc<CS: ConstraintSystem<E>>(&self) -> LinearCombination<E> {
        match &self.variant {
            ScalarVariant::Constant(constant) => {
                LinearCombination::zero() + (constant.value.to_owned(), CS::one())
            }
            ScalarVariant::Variable(variable) => LinearCombination::zero() + variable.variable,
        }
    }

    pub fn get_bits_le<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
    ) -> Result<Vec<Self>, RuntimeError> {
        let num = self.to_expression::<CS>();
        let bits = match self.scalar_type {
            ScalarType::Field => num.into_bits_le_strict(cs.namespace(|| "into_bits_le_strict")),
            ref scalar_type => num.into_bits_le_fixed(
                cs.namespace(|| "into_bits_le_fixed"),
                scalar_type.bit_length::<E>(),
            ),
        }?;

        bits.into_iter()
            .enumerate()
            .map(|(i, bit)| Self::from_boolean(cs.namespace(|| format!("bit {}", i)), bit))
            .collect()
    }

    pub fn with_type_unchecked(&self, scalar_type: ScalarType) -> Self {
        Self {
            variant: self.variant.clone(),
            scalar_type,
        }
    }

    pub fn from_boolean<CS: ConstraintSystem<E>>(
        mut cs: CS,
        boolean: Boolean,
    ) -> Result<Self, RuntimeError> {
        match boolean {
            Boolean::Is(bit) => Ok(Self::new_unchecked_variable(
                bit.get_value_field::<E>(),
                bit.get_variable(),
                ScalarType::Boolean,
            )),
            Boolean::Not(bit) => {
                let expr = Expression::constant::<CS>(E::Fr::one()) - Expression::from(&bit);
                let num = expr.into_number(cs.namespace(|| "into_number"))?;
                let scalar = Self::from(num);
                Ok(scalar.with_type_unchecked(ScalarType::Boolean))
            }
            Boolean::Constant(_) => Ok(Self::new_constant_fr(
                boolean.get_value_field::<E>().unwrap(),
                ScalarType::Boolean,
            )),
        }
    }

    pub fn as_constant_unchecked(&self) -> Result<Self, RuntimeError> {
        Ok(Self::new_constant_fr(self.grab_value()?, self.get_type()))
    }
}

//impl<E: Engine> fmt::Debug for Scalar<E> {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        let value_str = self
//            .value
//            .map(|f| fr_bigint::fr_to_bigint(&f, self.is_signed()).to_string())
//            .unwrap_or_else(|| "none".into());
//
//        write!(
//            f,
//            "Scalar {{ value: {}, type: {} }}",
//            value_str, self.scalar_type
//        )
//    }
//}

impl<E: IEngine> fmt::Display for Scalar<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value_str = self
            .get_value()
            .map(|f| fr_bigint::fr_to_bigint(&f, self.is_signed()).to_string())
            .unwrap_or_else(|| "none".into());

        let det = if self.is_constant() { "det" } else { "witness" };
        write!(f, "{} as {} ({})", value_str, self.scalar_type, det)
    }
}

impl<E: IEngine> ToBigInt for Scalar<E> {
    fn to_bigint(&self) -> Option<BigInt> {
        self.get_value()
            .map(|fr| fr_bigint::fr_to_bigint(&fr, self.is_signed()))
    }
}

impl<E: IEngine> From<&AllocatedNum<E>> for Scalar<E> {
    fn from(num: &AllocatedNum<E>) -> Self {
        Self {
            variant: ScalarVariable {
                value: num.get_value(),
                variable: num.get_variable(),
            }
            .into(),
            scalar_type: ScalarType::Field,
        }
    }
}

impl<E: IEngine> From<AllocatedNum<E>> for Scalar<E> {
    fn from(num: AllocatedNum<E>) -> Self {
        Self {
            variant: ScalarVariable {
                value: num.get_value(),
                variable: num.get_variable(),
            }
            .into(),
            scalar_type: ScalarType::Field,
        }
    }
}
