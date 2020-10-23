pub mod expectation;
pub mod fr_bigint;
pub mod variant;

use std::fmt;

use num::bigint::ToBigInt;
use num::BigInt;
use num::ToPrimitive;

use franklin_crypto::bellman::pairing::ff::Field;
use franklin_crypto::bellman::pairing::ff::PrimeField;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::bellman::LinearCombination;
use franklin_crypto::bellman::SynthesisError;
use franklin_crypto::bellman::Variable;
use franklin_crypto::circuit::boolean::AllocatedBit;
use franklin_crypto::circuit::boolean::Boolean;
use franklin_crypto::circuit::expression::Expression;
use franklin_crypto::circuit::num::AllocatedNum;
use franklin_crypto::circuit::Assignment;

use zinc_build::IntegerType;
use zinc_build::ScalarType;

use crate::error::RuntimeError;
use crate::IEngine;

use self::expectation::ITypeExpectation as ScalarTypeExpectation;
use self::variant::constant::Constant as ScalarConstant;
use self::variant::variable::Variable as ScalarVariable;
use self::variant::Variant as ScalarVariant;

/// Scalar is a primitive value that can be stored on the stack and operated by VM's instructions.
#[derive(Debug, Clone)]
pub struct Scalar<E: IEngine> {
    variant: ScalarVariant<E>,
    scalar_type: ScalarType,
}

impl<E: IEngine> Scalar<E> {
    pub fn new_constant_usize(value: usize, scalar_type: ScalarType) -> Self {
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
            variant: ScalarVariant::Constant(ScalarConstant::new_fr(value)),
            scalar_type,
        }
    }

    pub fn new_constant_bigint(
        value: BigInt,
        scalar_type: ScalarType,
    ) -> Result<Self, RuntimeError> {
        let fr = fr_bigint::bigint_to_fr::<E>(&value).ok_or(RuntimeError::ValueOverflow {
            value,
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
            variant: ScalarVariant::Variable(ScalarVariable::new_unchecked(value, variable)),
            scalar_type,
        }
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

    pub fn to_field(&self) -> Self {
        Self {
            variant: self.variant.clone(),
            scalar_type: ScalarType::Field,
        }
    }

    pub fn to_type_unchecked(&self, scalar_type: ScalarType) -> Self {
        Self {
            variant: self.variant.clone(),
            scalar_type,
        }
    }

    pub fn to_constant_unchecked(&self) -> Result<Self, RuntimeError> {
        Ok(Self::new_constant_fr(self.grab_value()?, self.get_type()))
    }

    pub fn to_expression<CS: ConstraintSystem<E>>(&self) -> Expression<E> {
        Expression::new(self.get_value(), self.to_linear_combination::<CS>())
    }

    pub fn to_linear_combination<CS: ConstraintSystem<E>>(&self) -> LinearCombination<E> {
        match &self.variant {
            ScalarVariant::Constant(constant) => {
                LinearCombination::zero() + (constant.value.to_owned(), CS::one())
            }
            ScalarVariant::Variable(variable) => LinearCombination::zero() + variable.variable,
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

    pub fn grab_value(&self) -> Result<E::Fr, SynthesisError> {
        self.get_value().grab()
    }

    pub fn get_variant(&self) -> &ScalarVariant<E> {
        &self.variant
    }

    pub fn get_constant(&self) -> Result<E::Fr, RuntimeError> {
        match &self.variant {
            ScalarVariant::Constant(constant) => Ok(constant.value.to_owned()),
            _ => Err(RuntimeError::ExpectedConstant),
        }
    }

    pub fn get_constant_usize(&self) -> Result<usize, RuntimeError> {
        let fr = self.get_constant()?;
        let bigint = fr_bigint::fr_to_bigint::<E>(&fr, false);
        bigint
            .to_usize()
            .ok_or_else(|| RuntimeError::ExpectedUsize(bigint))
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
                scalar_type.bitlength::<E>(),
            ),
        }?;

        bits.into_iter()
            .enumerate()
            .map(|(i, bit)| Self::from_boolean(cs.namespace(|| format!("bit {}", i)), bit))
            .collect()
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
                Ok(scalar.to_type_unchecked(ScalarType::Boolean))
            }
            Boolean::Constant(_) => Ok(Self::new_constant_fr(
                boolean
                    .get_value_field::<E>()
                    .ok_or(RuntimeError::ExpectedConstant)?,
                ScalarType::Boolean,
            )),
        }
    }

    pub fn conditional_type_check<CS>(
        cs: CS,
        condition: &Self,
        scalar: &Self,
        scalar_type: ScalarType,
    ) -> Result<Self, RuntimeError>
    where
        CS: ConstraintSystem<E>,
    {
        condition.get_type().assert_type(ScalarType::Boolean)?;

        match scalar_type {
            ScalarType::Boolean => {
                // Check as u1 integer, then changet type to Boolean
                let checked =
                    Self::conditional_type_check(cs, condition, scalar, IntegerType::U1.into())?;
                Ok(checked.to_type_unchecked(scalar_type))
            }
            ScalarType::Integer(int_type) => {
                Self::conditional_int_type_check(cs, condition, scalar, int_type)
            }
            ScalarType::Field => {
                // Always safe to cast into field
                Ok(scalar.to_field())
            }
        }
    }

    fn conditional_int_type_check<CS>(
        mut cs: CS,
        condition: &Self,
        scalar: &Self,
        int_type: IntegerType,
    ) -> Result<Self, RuntimeError>
    where
        CS: ConstraintSystem<E>,
    {
        // Throw runtime error if value is known.
        if let (Some(value_fr), Some(condition_fr)) = (scalar.get_value(), condition.get_value()) {
            let value = fr_bigint::fr_to_bigint::<E>(&value_fr, int_type.is_signed);
            if !condition_fr.is_zero() && (value < int_type.min() || value > int_type.max()) {
                return Err(RuntimeError::ValueOverflow {
                    value,
                    scalar_type: int_type.into(),
                });
            }
        }

        // If scalar is constant and have passed the check, no need to create constraints.
        if scalar.is_constant() {
            return Ok(scalar.to_type_unchecked(int_type.into()));
        }

        let scalar_expr = scalar.to_expression::<CS>();
        let offset_expr = if !int_type.is_signed {
            Expression::u64::<CS>(0)
        } else {
            let offset = BigInt::from(1) << (int_type.bitlength - 1);
            let offset_fr =
                fr_bigint::bigint_to_fr::<E>(&offset).expect("invalid integer type length");
            Expression::constant::<CS>(offset_fr)
        };
        let zero = Expression::u64::<CS>(0);

        // If checking inside the false branch, use zero instead to avoid throwing an error.
        let condition_bool = condition.to_boolean(cs.namespace(|| "to_boolean"))?;
        let value_to_check = Expression::conditionally_select(
            cs.namespace(|| "select value to check"),
            scalar_expr + offset_expr,
            zero,
            &condition_bool,
        )?;

        // If value is overflowing, `into_bits_le_fixed` will be unsatisfiable.
        let _bits =
            value_to_check.into_bits_le_fixed(cs.namespace(|| "into_bits"), int_type.bitlength)?;

        Ok(scalar.to_type_unchecked(int_type.into()))
    }
}

impl<E: IEngine> PartialEq<Self> for Scalar<E> {
    fn eq(&self, other: &Self) -> bool {
        self.to_bigint() == other.to_bigint()
    }
}

impl<E: IEngine> ToBigInt for Scalar<E> {
    fn to_bigint(&self) -> Option<BigInt> {
        self.get_value()
            .map(|fr| fr_bigint::fr_to_bigint::<E>(&fr, self.is_signed()))
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

impl<E: IEngine> fmt::Display for Scalar<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value_str = self
            .get_value()
            .map(|f| fr_bigint::fr_to_bigint::<E>(&f, self.is_signed()).to_string())
            .unwrap_or_else(|| "none".into());

        let det = if self.is_constant() { "det" } else { "witness" };
        write!(f, "{} as {} ({})", value_str, self.scalar_type, det)
    }
}
