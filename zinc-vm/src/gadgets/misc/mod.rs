use std::marker::PhantomData;
use std::mem;

use bellman::{ConstraintSystem, Namespace};
use ff::Field;
use num_bigint::BigInt;

use crate::core::RuntimeError;
use crate::gadgets::{Gadget, IntegerType, Scalar, ScalarType, ScalarTypeExpectation, ScalarVariant, utils};
use crate::{gadgets, Engine};
use franklin_crypto::circuit::expression::Expression;
use franklin_crypto::circuit::Assignment;

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

    fn one(&self, scalar_type: ScalarType) -> Scalar<E> {
        self.constant_bigint(&1.into(), scalar_type)
            .expect("can't overflow")
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
        let scalar = Scalar::new_unchecked_variable(value, variable, scalar_type);

        match scalar_type {
            ScalarType::Field => {
                // Create some constraints to avoid unconstrained variable errors.
                let one = Scalar::new_constant_fr(E::Fr::one(), ScalarType::Field);
                gadgets::arithmetic::add(cs.namespace(|| "dummy constraint"), &scalar, &one)?;
                Ok(scalar)
            }
            _ => {
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
                utils::bigint_to_fr::<E>(bigint).ok_or(RuntimeError::ValueOverflow {
                    value: bigint.clone(),
                    scalar_type,
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
        let value = utils::bigint_to_fr::<E>(value).ok_or_else(|| RuntimeError::ValueOverflow {
            value: value.clone(),
            scalar_type,
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

    pub fn add(&mut self, left: Scalar<E>, right: Scalar<E>) -> Result<Scalar<E>, RuntimeError> {
        let sum = match (left.get_value(), right.get_value()) {
            (Some(l), Some(r)) => {
                let mut sum = l;
                sum.add_assign(&r);
                Some(sum)
            }
            _ => None,
        };

        let mut cs = self.cs_namespace();

        let sum_var = cs.alloc(|| "sum variable", || sum.grab())?;

        cs.enforce(
            || "sum constraint",
            |lc| lc + &left.lc::<CS>() + &right.lc::<CS>(),
            |lc| lc + CS::one(),
            |lc| lc + sum_var,
        );

        Ok(Scalar::new_unchecked_variable(
            sum,
            sum_var,
            ScalarType::Field,
        ))
    }

    pub fn sub(&mut self, left: Scalar<E>, right: Scalar<E>) -> Result<Scalar<E>, RuntimeError> {
        let diff = match (left.get_value(), right.get_value()) {
            (Some(l), Some(r)) => {
                let mut diff = l;
                diff.sub_assign(&r);
                Some(diff)
            }
            _ => None,
        };

        let mut cs = self.cs_namespace();

        let diff_var = cs.alloc(|| "diff variable", || diff.grab())?;
        cs.enforce(
            || "diff constraint",
            |lc| lc + &left.lc::<CS>() - &right.lc::<CS>(),
            |lc| lc + CS::one(),
            |lc| lc + diff_var,
        );

        mem::drop(cs);
        Ok(Scalar::new_unchecked_variable(
            diff,
            diff_var,
            ScalarType::Field,
        ))
    }

    pub fn mul(&mut self, left: Scalar<E>, right: Scalar<E>) -> Result<Scalar<E>, RuntimeError> {
        let prod = match (left.get_value(), right.get_value()) {
            (Some(l), Some(r)) => {
                let mut prod = l;
                prod.mul_assign(&r);
                Some(prod)
            }
            _ => None,
        };

        let mut cs = self.cs_namespace();

        let prod_var = cs
            .alloc(|| "prod variable", || prod.grab())
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "prod constraint",
            |lc| lc + &left.lc::<CS>(),
            |lc| lc + &right.lc::<CS>(),
            |lc| lc + prod_var,
        );

        Ok(Scalar::new_unchecked_variable(
            prod,
            prod_var,
            ScalarType::Field,
        ))
    }

    pub fn neg(&mut self, element: Scalar<E>) -> Result<Scalar<E>, RuntimeError> {
        let neg_value = match element.get_value() {
            Some(value) => {
                let mut neg = E::Fr::zero();
                neg.sub_assign(&value);
                Some(neg)
            }
            _ => None,
        };

        let mut cs = self.cs_namespace();

        let neg_variable = cs
            .alloc(|| "neg variable", || neg_value.grab())
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "neg constraint",
            |lc| lc + &element.lc::<CS>(),
            |lc| lc + CS::one(),
            |lc| lc - neg_variable,
        );

        mem::drop(cs);

        let new_type = match element.get_type() {
            t @ ScalarType::Boolean => {
                return Err(RuntimeError::TypeError {
                    expected: "field or integer type".to_string(),
                    actual: t.to_string(),
                })
            }
            t @ ScalarType::Field => t,
            t @ ScalarType::Integer(IntegerType { signed: true, .. }) => t,
            ScalarType::Integer(IntegerType {
                signed: false,
                length,
            }) => IntegerType {
                signed: true,
                length: length + 1,
            }
            .into(),
        };
        Ok(Scalar::new_unchecked_variable(
            neg_value,
            neg_variable,
            new_type,
        ))
    }

    pub fn not(&mut self, element: Scalar<E>) -> Result<Scalar<E>, RuntimeError> {
        element.get_type().assert_type(ScalarType::Boolean)?;
        let one = self.one(element.get_type());
        self.sub(one, element)
            .map(|scalar| scalar.with_type_unchecked(ScalarType::Boolean))
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

    pub fn lt(&mut self, left: Scalar<E>, right: Scalar<E>) -> Result<Scalar<E>, RuntimeError> {
        let cs = self.cs_namespace();
        gadgets::comparison::lt(cs, &left, &right)
    }

    pub fn le(&mut self, left: Scalar<E>, right: Scalar<E>) -> Result<Scalar<E>, RuntimeError> {
        let cs = self.cs_namespace();
        gadgets::comparison::le(cs, &left, &right)
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

    pub fn ne(&mut self, left: Scalar<E>, right: Scalar<E>) -> Result<Scalar<E>, RuntimeError> {
        let eq = self.eq(left, right)?;
        self.not(eq)
    }

    pub fn ge(&mut self, left: Scalar<E>, right: Scalar<E>) -> Result<Scalar<E>, RuntimeError> {
        self.le(right, left)
    }

    pub fn gt(&mut self, left: Scalar<E>, right: Scalar<E>) -> Result<Scalar<E>, RuntimeError> {
        self.lt(right, left)
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
        // let index = gadgets::conditional_select(self.cs_namespace(), condition, index, &zero)?;
        self.enforcing_array_get(array, &index)
    }

    /// This gadget enforces 0 <= index < array.len()
    pub fn enforcing_array_get(
        &mut self,
        array: &[Scalar<E>],
        index: &Scalar<E>,
    ) -> Result<Scalar<E>, RuntimeError> {
        assert!(!array.is_empty(), "reading from empty array");

        let length = self.constant_bigint(&array.len().into(), index.get_type())?;
        let lt = self.lt(index.clone(), length)?;
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
                //     let value = gadgets::conditional_select(cs, &is_current_index, &value, p)?;
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
