pub mod expectation;
pub mod variant;

use std::fmt;
use std::ops::Div;
use std::ops::Neg;

use num_bigint::BigInt;
use num_bigint::Sign;
use num_bigint::ToBigInt;
use num_traits::Signed;
use num_traits::ToPrimitive;

use ff::Field;
use ff::PrimeField;
use ff::PrimeFieldRepr;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::bellman::LinearCombination;
use franklin_crypto::bellman::SynthesisError;
use franklin_crypto::bellman::Variable;
use franklin_crypto::circuit::boolean::AllocatedBit;
use franklin_crypto::circuit::boolean::Boolean;
use franklin_crypto::circuit::expression::Expression;
use franklin_crypto::circuit::num::AllocatedNum;
use franklin_crypto::circuit::Assignment;
use pairing::Engine;

use zinc_bytecode::ScalarType;

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
            variant: ScalarVariant::Constant(ScalarConstant::new_fr(value)),
            scalar_type,
        }
    }

    pub fn new_constant_bigint(
        value: &BigInt,
        scalar_type: ScalarType,
    ) -> Result<Self, RuntimeError> {
        let fr = bigint_to_fr::<E>(value).ok_or(RuntimeError::ValueOverflow {
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
        let bigint = fr_to_bigint::<E>(&fr, false);
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
                boolean.get_value_field::<E>().unwrap(),
                ScalarType::Boolean,
            )),
        }
    }
}

pub fn fr_to_bigint<E: Engine>(fr: &E::Fr, signed: bool) -> BigInt {
    if signed {
        fr_to_bigint_signed::<E>(fr)
    } else {
        fr_to_bigint_unsigned::<E>(fr)
    }
}

pub fn bigint_to_fr<E: Engine>(bigint: &BigInt) -> Option<E::Fr> {
    if bigint.is_positive() {
        E::Fr::from_str(&bigint.to_str_radix(10))
    } else {
        let abs = E::Fr::from_str(&bigint.neg().to_str_radix(10))?;
        let mut fr = E::Fr::zero();
        fr.sub_assign(&abs);
        Some(fr)
    }
}

fn fr_to_bigint_signed<E: Engine>(fr: &E::Fr) -> BigInt {
    let mut buffer = Vec::<u8>::new();
    E::Fr::char()
        .write_be(&mut buffer)
        .expect("failed to write into Vec<u8>");
    let modulus = BigInt::from_bytes_be(Sign::Plus, &buffer);
    buffer.clear();

    fr.into_repr()
        .write_be(&mut buffer)
        .expect("failed to write into Vec<u8>");
    let value = BigInt::from_bytes_be(Sign::Plus, &buffer);

    if value < (modulus.clone().div(2)) {
        value
    } else {
        value - modulus
    }
}

fn fr_to_bigint_unsigned<E: Engine>(fr: &E::Fr) -> BigInt {
    let mut buffer = Vec::<u8>::new();
    fr.into_repr()
        .write_be(&mut buffer)
        .expect("failed to write into Vec<u8>");
    BigInt::from_bytes_be(Sign::Plus, &buffer)
}

impl<E: IEngine> ToBigInt for Scalar<E> {
    fn to_bigint(&self) -> Option<BigInt> {
        self.get_value()
            .map(|fr| fr_to_bigint::<E>(&fr, self.is_signed()))
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
            .map(|f| fr_to_bigint::<E>(&f, self.is_signed()).to_string())
            .unwrap_or_else(|| "none".into());

        let det = if self.is_constant() { "det" } else { "witness" };
        write!(f, "{} as {} ({})", value_str, self.scalar_type, det)
    }
}

#[cfg(test)]
mod test {
    use num_bigint::BigInt;
    use num_traits::ToPrimitive;

    use ff::PrimeField;
    use franklin_crypto::bellman::pairing::bn256::Bn256;
    use franklin_crypto::bellman::pairing::bn256::Fr;

    use crate::gadgets;

    #[test]
    fn test_fr_to_bigint() {
        let values = [0, 1, 2, 42, 1_234_567_890];

        for value in values.iter() {
            let fr = Fr::from_str(value.to_string().as_str()).unwrap();
            let bigint = gadgets::scalar::fr_to_bigint::<Bn256>(&fr, true);
            assert_eq!(bigint.to_i32(), Some(*value));
        }
    }

    #[test]
    fn test_bigint_to_fr() {
        let values = [0, 1, 2, 42, 1_234_567_890];

        for value in values.iter() {
            let bigint = BigInt::from(*value);
            let fr = gadgets::scalar::bigint_to_fr::<Bn256>(&bigint);
            assert_eq!(fr, Fr::from_str(value.to_string().as_str()));
        }
    }

    #[test]
    fn test_negatives() {
        let values = [-1 as isize, -42, -123_456_789_098_761];

        for value in values.iter() {
            let expected = BigInt::from(*value);
            let fr = gadgets::scalar::bigint_to_fr::<Bn256>(&expected).expect("bigint_to_fr");
            let actual = gadgets::scalar::fr_to_bigint::<Bn256>(&fr, true);
            assert_eq!(actual, expected);
        }
    }
}
