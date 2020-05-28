use std::marker::PhantomData;
use std::mem;

use num_bigint::BigInt;

use bellman::ConstraintSystem;
use bellman::Namespace;
use ff::Field;
use franklin_crypto::circuit::expression::Expression;
use franklin_crypto::circuit::Assignment;

use zinc_bytecode::ScalarType;

use crate::error::RuntimeError;
use crate::gadgets;
use crate::gadgets::fr_bigint;
use crate::gadgets::scalar::scalar_type::ScalarTypeExpectation;
use crate::gadgets::scalar::Scalar;
use crate::gadgets::scalar::ScalarVariant;
use crate::gadgets::Gadget;
use crate::Engine;

pub struct Gadgets<E, CS>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    cs: CS,
    counter: usize,
    pd: PhantomData<E>,
}

impl<E, CS> Gadgets<E, CS>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    pub fn new(cs: CS) -> Self {
        Self {
            cs,
            counter: 0,
            pd: PhantomData,
        }
    }

    fn cs_namespace(&mut self) -> Namespace<E, CS::Root> {
        let s = format!("{}", self.counter);
        self.counter += 1;
        self.cs.namespace(|| s)
    }

    #[allow(dead_code)]
    pub fn constraint_system(&mut self) -> &mut CS {
        &mut self.cs
    }
}

impl<E, CS> Gadgets<E, CS>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn witness_fr(
        &mut self,
        value: Option<E::Fr>,
        scalar_type: ScalarType,
    ) -> Result<Scalar<E>, RuntimeError> {
        let mut cs = self.cs_namespace();

        let variable = cs.alloc(|| "variable", || value.grab())?;
        let scalar = Scalar::new_unchecked_variable(value, variable, scalar_type.clone());

        match scalar_type {
            ScalarType::Field => {
                // Create some constraints to avoid unconstrained variable errors.
                let one = Scalar::new_constant_fr(E::Fr::one(), ScalarType::Field);
                gadgets::arithmetic::add::add(cs.namespace(|| "dummy constraint"), &scalar, &one)?;
                Ok(scalar)
            }
            scalar_type => {
                let condition = Scalar::new_constant_fr(E::Fr::one(), ScalarType::Boolean);
                gadgets::types::conditional_type_check(
                    cs.namespace(|| "type check"),
                    &condition,
                    &scalar,
                    scalar_type,
                )
            }
        }
    }

    pub fn allocate_witness(
        &mut self,
        value: Option<&BigInt>,
        scalar_type: ScalarType,
    ) -> Result<Scalar<E>, RuntimeError> {
        let fr = if let Some(bigint) = value {
            Some(
                fr_bigint::bigint_to_fr::<E>(bigint).ok_or(RuntimeError::ValueOverflow {
                    value: bigint.clone(),
                    scalar_type: scalar_type.clone(),
                })?,
            )
        } else {
            None
        };

        self.witness_fr(fr, scalar_type)
    }

    pub fn constant_bigint(
        &self,
        value: &BigInt,
        scalar_type: ScalarType,
    ) -> Result<Scalar<E>, RuntimeError> {
        let value =
            fr_bigint::bigint_to_fr::<E>(value).ok_or_else(|| RuntimeError::ValueOverflow {
                value: value.clone(),
                scalar_type: scalar_type.clone(),
            })?;

        Ok(Scalar::new_constant_fr(value, scalar_type))
    }

    pub fn output(&mut self, element: Scalar<E>) -> Result<Scalar<E>, RuntimeError> {
        let mut cs = self.cs_namespace();

        let variable = cs
            .alloc_input(|| "output value", || element.grab_value())
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "enforce output equality",
            |lc| lc + variable,
            |lc| lc + CS::one(),
            |lc| lc + &element.lc::<CS>(),
        );

        Ok(Scalar::new_unchecked_variable(
            element.get_value(),
            variable,
            element.get_type(),
        ))
    }

    pub fn and(&mut self, left: Scalar<E>, right: Scalar<E>) -> Result<Scalar<E>, RuntimeError> {
        left.get_type().assert_type(ScalarType::Boolean)?;
        right.get_type().assert_type(ScalarType::Boolean)?;

        let mut cs = self.cs_namespace();

        let value = match (left.get_value(), right.get_value()) {
            (Some(a), Some(b)) => {
                let mut conj = a;
                conj.mul_assign(&b);
                Some(conj)
            }
            _ => None,
        };

        let variable = cs
            .alloc(|| "and", || value.grab())
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "equality",
            |lc| lc + &left.lc::<CS>(),
            |lc| lc + &right.lc::<CS>(),
            |lc| lc + variable,
        );

        Ok(Scalar::new_unchecked_variable(
            value,
            variable,
            ScalarType::Boolean,
        ))
    }

    pub fn or(&mut self, left: Scalar<E>, right: Scalar<E>) -> Result<Scalar<E>, RuntimeError> {
        left.get_type().assert_type(ScalarType::Boolean)?;
        right.get_type().assert_type(ScalarType::Boolean)?;

        let mut cs = self.cs_namespace();

        let value = match (left.get_value(), right.get_value()) {
            (Some(a), Some(b)) => {
                if a.is_zero() && b.is_zero() {
                    Some(E::Fr::zero())
                } else {
                    Some(E::Fr::one())
                }
            }
            _ => None,
        };

        let variable = cs
            .alloc(|| "or", || value.grab())
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "equality",
            |lc| lc + CS::one() - &left.lc::<CS>(),
            |lc| lc + CS::one() - &right.lc::<CS>(),
            |lc| lc + CS::one() - variable,
        );

        Ok(Scalar::new_unchecked_variable(
            value,
            variable,
            ScalarType::Boolean,
        ))
    }

    pub fn xor(&mut self, left: Scalar<E>, right: Scalar<E>) -> Result<Scalar<E>, RuntimeError> {
        left.get_type().assert_type(ScalarType::Boolean)?;
        right.get_type().assert_type(ScalarType::Boolean)?;

        let mut cs = self.cs_namespace();

        let value = match (left.get_value(), right.get_value()) {
            (Some(a), Some(b)) => {
                if a.is_zero() == b.is_zero() {
                    Some(E::Fr::zero())
                } else {
                    Some(E::Fr::one())
                }
            }
            _ => None,
        };

        let variable = cs
            .alloc(|| "conjunction", || value.grab())
            .map_err(RuntimeError::SynthesisError)?;

        // (a + a) * (b) = (a + b - c)
        cs.enforce(
            || "equality",
            |lc| lc + &left.lc::<CS>() + &left.lc::<CS>(),
            |lc| lc + &right.lc::<CS>(),
            |lc| lc + &left.lc::<CS>() + &right.lc::<CS>() - variable,
        );

        Ok(Scalar::new_unchecked_variable(
            value,
            variable,
            ScalarType::Boolean,
        ))
    }

    pub fn eq(&mut self, left: Scalar<E>, right: Scalar<E>) -> Result<Scalar<E>, RuntimeError> {
        let cs = self.cs_namespace();

        let l_num = left.to_expression::<CS>();
        let r_num = right.to_expression::<CS>();

        let eq = Expression::equals(cs, l_num, r_num)?;

        Ok(Scalar::new_unchecked_variable(
            eq.get_value_field::<E>(),
            eq.get_variable(),
            ScalarType::Boolean,
        ))
    }

    pub fn assert(
        &mut self,
        element: Scalar<E>,
        message: Option<&str>,
    ) -> Result<(), RuntimeError> {
        if let Some(value) = element.get_value() {
            if value.is_zero() {
                let s = message.unwrap_or("<no message>");
                return Err(RuntimeError::AssertionError(s.into()));
            }
        }

        let inverse_value = element
            .get_value()
            .map(|fr| fr.inverse().unwrap_or_else(E::Fr::zero));

        let mut cs = self.cs_namespace();
        let inverse_variable = cs
            .alloc(|| "inverse", || inverse_value.grab())
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "assertion",
            |lc| lc + &element.lc::<CS>(),
            |lc| lc + inverse_variable,
            |lc| lc + CS::one(),
        );

        Ok(())
    }

    /// This gadget only enforces 0 <= index < array.len() if condition is true
    pub fn conditional_array_get(
        &mut self,
        _condition: &Scalar<E>,
        array: &[Scalar<E>],
        index: &Scalar<E>,
    ) -> Result<Scalar<E>, RuntimeError> {
        if !index.is_constant() {
            return Err(RuntimeError::WitnessArrayIndex);
        }
        // let zero = Scalar::new_constant_int(0, index.get_type());
        // let index = gadgets::conditional_select::conditional_select(self.cs_namespace(), condition, index, &zero)?;
        self.enforcing_array_get(array, &index)
    }

    /// This gadget enforces 0 <= index < array.len()
    pub fn enforcing_array_get(
        &mut self,
        array: &[Scalar<E>],
        index: &Scalar<E>,
    ) -> Result<Scalar<E>, RuntimeError> {
        let mut cs = self.cs_namespace();

        assert!(!array.is_empty(), "reading from empty array");

        let length = Scalar::new_constant_bigint(&array.len().into(), index.get_type())?;
        let lt = gadgets::comparison::lt(cs.namespace(|| "cs"), index, &length)?;
        mem::drop(cs);
        self.assert(lt, Some("index out of bounds"))?;

        match index.get_variant() {
            ScalarVariant::Constant(_) => {
                let i = index.get_constant_usize()?;
                if i >= array.len() {
                    return Err(RuntimeError::IndexOutOfBounds {
                        lower_bound: 0,
                        upper_bound: array.len(),
                        actual: i,
                    });
                }
                Ok(array[i].clone())
            }
            _ => {
                Err(RuntimeError::WitnessArrayIndex)
                // let mut cs = self.cs_namespace();
                // let num_bits = math::log2ceil(array.len());
                // let bits_le = index.to_expression::<CS>().into_bits_le_fixed(
                //     cs.namespace(|| "into_bits"),
                //     num_bits
                // )?;
                // let bits_be = bits_le
                //     .into_iter()
                //     .rev()
                //     .enumerate()
                //     .map(|(i, bit)| {
                //         Scalar::from_boolean(cs.namespace(|| format!("bit {}", i)), bit)
                //     })
                //     .collect::<Result<Vec<Scalar<E>>, RuntimeError>>()?;

                // gadgets::recursive_select(
                //     cs.namespace(|| "recursive_select"),
                //     &bits_be,
                //     array
                // )
            }
        }
    }

    pub fn array_set(
        &mut self,
        array: &[Scalar<E>],
        index: Scalar<E>,
        value: Scalar<E>,
    ) -> Result<Vec<Scalar<E>>, RuntimeError> {
        let mut new_array = Vec::from(array);

        match index.get_variant() {
            ScalarVariant::Constant(_) => {
                let i = index.get_constant_usize()?;
                if i >= array.len() {
                    return Err(RuntimeError::IndexOutOfBounds {
                        lower_bound: 0,
                        upper_bound: array.len(),
                        actual: i,
                    });
                }
                new_array[i] = value;
            }
            _ => {
                return Err(RuntimeError::WitnessArrayIndex);
                // let mut new_array = Vec::new();

                // for (i, p) in array.iter().enumerate() {
                //     let curr_index = Scalar::new_constant_int(i, ScalarType::Field);
                //     let is_current_index = self.eq(curr_index, index.clone())?;
                //     let cs = self.cs_namespace();
                //     let value = gadgets::conditional_select::conditional_select(cs, &is_current_index, &value, p)?;
                //     new_array.push(value);
                // }
            }
        };

        Ok(new_array)
    }

    pub fn execute<G: Gadget<E>>(
        &mut self,
        gadget: G,
        input: &[Scalar<E>],
    ) -> Result<Vec<Scalar<E>>, RuntimeError> {
        let cs = self.cs_namespace();
        gadget.synthesize_vec(cs, input)
    }
}
