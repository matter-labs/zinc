use std::fmt::{Debug, Display, Error, Formatter};
use std::marker::PhantomData;

use crate::Engine;
use bellman::{ConstraintSystem, Variable};
use ff::{Field, PrimeField};
use franklin_crypto::bellman::{Namespace, SynthesisError};
use franklin_crypto::circuit::num::AllocatedNum;
use num_bigint::{BigInt, ToBigInt};

use crate::core::RuntimeError;
use crate::gadgets::utils::fr_to_bigint;
use crate::gadgets::{utils, Gadget, Primitive, PrimitiveType};
use num_traits::ToPrimitive;
use std::mem;

impl<E: Engine> Debug for Primitive<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let value_str = match self.value {
            Some(ref value) => fr_to_bigint(value).to_string(),
            None => "none".into(),
        };
        let type_str = match self.data_type {
            Some(data_type) => format!(
                "as {}{}",
                if data_type.signed { "i" } else { "u" },
                data_type.length,
            ),
            None => "(untyped)".into(),
        };

        f.write_str(format!("Primitive {{ {} as {} }}", value_str, type_str).as_str())
    }
}

impl<E: Engine> Primitive<E> {
    fn new(value: Option<E::Fr>, variable: Variable) -> Self {
        Self {
            value,
            variable,
            data_type: None,
        }
    }

    fn new_with_type(value: Option<E::Fr>, variable: Variable, data_type: PrimitiveType) -> Self {
        Self {
            value,
            variable,
            data_type: Some(data_type),
        }
    }

    pub fn as_allocated_num<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
    ) -> Result<AllocatedNum<E>, RuntimeError> {
        let num = AllocatedNum::alloc(cs.namespace(|| "allucated num"), || {
            self.value.ok_or(SynthesisError::AssignmentMissing)
        })
        .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "allocated num",
            |lc| lc + self.variable,
            |lc| lc + CS::one(),
            |lc| lc + num.get_variable(),
        );

        Ok(num)
    }
}

impl<E: Engine> Display for Primitive<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match &self.value {
            Some(value) => {
                let bigint = utils::fr_to_bigint(value);
                Display::fmt(&bigint, f)
            }
            None => Display::fmt("none", f),
        }
    }
}

impl<E: Engine> ToBigInt for Primitive<E> {
    fn to_bigint(&self) -> Option<BigInt> {
        self.value.map(|fr| -> BigInt { utils::fr_to_bigint(&fr) })
    }
}

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

    fn zero_typed(&mut self, data_type: Option<PrimitiveType>) -> Result<Primitive<E>, RuntimeError> {
        let value = E::Fr::zero();
        let mut cs = self.cs_namespace();
        let variable = cs
            .alloc(|| "zero_var", || Ok(value))
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "zero constraint",
            |lc| lc + variable,
            |lc| lc + CS::one(),
            |lc| lc,
        );
        mem::drop(cs);

        match data_type {
            Some(data_type) => self.value_with_type_check(Some(value), variable, data_type),
            None => Ok(Primitive::new(Some(value), variable)),
        }
    }

    fn one() -> Primitive<E> {
        Primitive::new(Some(E::Fr::one()), CS::one())
    }

    fn one_typed(&mut self, data_type: Option<PrimitiveType>) -> Result<Primitive<E>, RuntimeError> {
        match data_type {
            None => Ok(Self::one()),
            Some(data_type) => self.value_with_type_check(Some(E::Fr::one()), CS::one(), data_type),
        }
    }

    #[allow(dead_code)]
    pub fn constraint_system(&mut self) -> &mut CS {
        &mut self.cs
    }

    fn abs(&mut self, value: Primitive<E>) -> Result<Primitive<E>, RuntimeError> {
        let zero = self.zero_typed(value.data_type)?;
        let neg = Gadgets::neg(self, value.clone())?;
        let lt0 = Gadgets::lt(self, value.clone(), zero)?;
        self.conditional_select(lt0, neg, value)
    }

    #[allow(dead_code)]
    fn bits(
        &mut self,
        index: Primitive<E>,
        array_length: usize,
    ) -> Result<Vec<Primitive<E>>, RuntimeError> {
        let length = match index.data_type {
            None => self.constant_bigint(&array_length.into())?,
            Some(data_type) => self.constant_bigint_typed(&array_length.into(), data_type)?,
        };
        let index_lt_length = self.lt(index.clone(), length)?;

        self.assert(index_lt_length)?;

        let mut cs = self.cs_namespace();
        let num = index.as_allocated_num(cs.namespace(|| "into_allocated_num"))?;
        let bit_length = utils::tree_height(array_length);
        let bits = num
            .into_bits_le_fixed(cs.namespace(|| "bits_le_fixed"), bit_length)
            .map_err(RuntimeError::SynthesisError)?;

        let bits = bits
            .into_iter()
            .map(|bit| {
                Primitive::new(
                    bit.get_value_field::<E>(),
                    bit.get_variable()
                        .expect("bit value expected")
                        .get_variable(),
                )
            })
            .collect();

        Ok(bits)
    }

    #[allow(dead_code)]
    fn recursive_select(
        &mut self,
        array: &[Primitive<E>],
        index_bits: &[Primitive<E>],
    ) -> Result<Primitive<E>, RuntimeError> {
        if array.len() == 1 {
            return Ok(array[0].clone());
        }

        let bit = index_bits.first().expect("recursion error");

        let mut new_array = Vec::new();
        for i in 0..(array.len() / 2) {
            let p = self.conditional_select(
                bit.clone(),
                array[i * 2 + 1].clone(),
                array[i * 2].clone(),
            )?;
            new_array.push(p);
        }

        if array.len() % 2 == 1 {
            new_array.push(array.last().unwrap().clone());
        }

        self.recursive_select(new_array.as_slice(), &index_bits[1..])
    }

    /// Create new typed value and check value's type.
    fn value_with_type_check(
        &mut self,
        value: Option<E::Fr>,
        variable: Variable,
        data_type: PrimitiveType,
    ) -> Result<Primitive<E>, RuntimeError> {
        let untyped = Primitive::new(value, variable);

        let adjusted = if data_type.signed {
            let offset_value = BigInt::from(1) << (data_type.length - 1);
            let offset = self.constant_bigint(&offset_value)?;
            self.add(untyped, offset)?
        } else {
            untyped
        };

        let mut cs = self.cs_namespace();
        let num = adjusted.as_allocated_num(cs.namespace(|| "as_allocated_num"))?;
        let _bits = num
            .into_bits_le_fixed(cs.namespace(|| "into_bits_le_fixed"), data_type.length)
            .map_err(RuntimeError::SynthesisError)?;

        Ok(Primitive::new_with_type(value, variable, data_type))
    }

    /// Create new typed value and check that all the operands have the same type
    fn value_with_arguments_type_check(
        &mut self,
        value: Option<E::Fr>,
        variable: Variable,
        operands: &[Primitive<E>],
    ) -> Result<Primitive<E>, RuntimeError> {
        assert!(!operands.is_empty());
        let data_type = operands.first().unwrap().data_type;

        for value in operands {
            if value.data_type != data_type {
                return Err(RuntimeError::TypeError);
            }
        }

        match data_type {
            Some(data_type) => self.value_with_type_check(value, variable, data_type),
            None => Ok(Primitive::new(value, variable)),
        }
    }
}

impl<E, CS> Gadgets<E, CS>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    pub fn variable_none(
        &mut self,
        data_type: Option<PrimitiveType>,
    ) -> Result<Primitive<E>, RuntimeError> {
        let mut cs = self.cs_namespace();

        let variable = cs
            .alloc(
                || "variable value",
                || Err(SynthesisError::AssignmentMissing),
            )
            .map_err(RuntimeError::SynthesisError)?;

        mem::drop(cs);

        match data_type {
            None => Ok(Primitive::new(None, variable)),
            Some(t) => self.value_with_type_check(None, variable, t),
        }
    }

    pub fn variable_bigint(
        &mut self,
        value: &BigInt,
        data_type: Option<PrimitiveType>,
    ) -> Result<Primitive<E>, RuntimeError> {
        let value = utils::bigint_to_fr::<E>(value)
            .ok_or_else(|| RuntimeError::InternalError("bigint_to_fr".into()))?;

        let mut cs = self.cs_namespace();

        let variable = cs
            .alloc(|| "variable value", || Ok(value))
            .map_err(RuntimeError::SynthesisError)?;

        mem::drop(cs);

        match data_type {
            None => Ok(Primitive::new(Some(value), variable)),
            Some(t) => self.value_with_type_check(Some(value), variable, t),
        }
    }

    pub fn constant_bigint(&mut self, value: &BigInt) -> Result<Primitive<E>, RuntimeError> {
        let value = utils::bigint_to_fr::<E>(value)
            .ok_or_else(|| RuntimeError::InternalError("bigint_to_fr".into()))?;

        let mut cs = self.cs_namespace();

        let variable = cs
            .alloc(|| "constant value", || Ok(value))
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "constant equation",
            |lc| lc + CS::one(),
            |lc| lc + (value, CS::one()),
            |lc| lc + variable,
        );

        Ok(Primitive::new(Some(value), variable))
    }

    pub fn constant_bigint_typed(
        &mut self,
        value: &BigInt,
        data_type: PrimitiveType,
    ) -> Result<Primitive<E>, RuntimeError> {
        let p = self.constant_bigint(value)?;
        self.value_with_type_check(p.value, p.variable, data_type)
    }

    pub fn output(&mut self, element: Primitive<E>) -> Result<Primitive<E>, RuntimeError> {
        let mut cs = self.cs_namespace();

        let variable = cs
            .alloc_input(
                || "output value",
                || element.value.ok_or(SynthesisError::AssignmentMissing),
            )
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "enforce output equality",
            |lc| lc + variable,
            |lc| lc + CS::one(),
            |lc| lc + element.variable,
        );

        Ok(Primitive::new(element.value, variable))
    }

    pub fn set_type(
        &mut self,
        value: Primitive<E>,
        data_type: PrimitiveType,
    ) -> Result<Primitive<E>, RuntimeError> {
        self.value_with_type_check(value.value, value.variable, data_type)
    }

    pub fn add(
        &mut self,
        left: Primitive<E>,
        right: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError> {
        let sum = match (left.value, right.value) {
            (Some(l), Some(r)) => {
                let mut sum = l;
                sum.add_assign(&r);
                Some(sum)
            }
            _ => None,
        };

        let mut cs = self.cs_namespace();

        let sum_var = cs
            .alloc(
                || "sum variable",
                || sum.ok_or(SynthesisError::AssignmentMissing),
            )
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "sum constraint",
            |lc| lc + left.variable + right.variable,
            |lc| lc + CS::one(),
            |lc| lc + sum_var,
        );

        mem::drop(cs);
        self.value_with_arguments_type_check(sum, sum_var, &[left, right])
    }

    pub fn sub(
        &mut self,
        left: Primitive<E>,
        right: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError> {
        let diff = match (left.value, right.value) {
            (Some(l), Some(r)) => {
                let mut diff = l;
                diff.sub_assign(&r);
                Some(diff)
            }
            _ => None,
        };

        let mut cs = self.cs_namespace();

        let diff_var = cs
            .alloc(
                || "diff variable",
                || diff.ok_or(SynthesisError::AssignmentMissing),
            )
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "diff constraint",
            |lc| lc + left.variable - right.variable,
            |lc| lc + CS::one(),
            |lc| lc + diff_var,
        );

        mem::drop(cs);
        self.value_with_arguments_type_check(diff, diff_var, &[left, right])
    }

    pub fn mul(
        &mut self,
        left: Primitive<E>,
        right: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError> {
        let prod = match (left.value, right.value) {
            (Some(l), Some(r)) => {
                let mut prod = l;
                prod.mul_assign(&r);
                Some(prod)
            }
            _ => None,
        };

        let mut cs = self.cs_namespace();

        let prod_var = cs
            .alloc(
                || "prod variable",
                || prod.ok_or(SynthesisError::AssignmentMissing),
            )
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "prod constraint",
            |lc| lc + left.variable,
            |lc| lc + right.variable,
            |lc| lc + prod_var,
        );

        mem::drop(cs);
        self.value_with_arguments_type_check(prod, prod_var, &[left, right])
    }

    pub fn div_rem(
        &mut self,
        left: Primitive<E>,
        right: Primitive<E>,
    ) -> Result<(Primitive<E>, Primitive<E>), RuntimeError> {
        let nominator = left.clone();
        let denominator = right.clone();

        let mut quotient_value: Option<E::Fr> = None;
        let mut remainder_value: Option<E::Fr> = None;

        if let (Some(nom), Some(denom)) = (nominator.value, denominator.value) {
            let nom_bi = utils::fr_to_bigint(&nom);
            let denom_bi = utils::fr_to_bigint(&denom);

            let (q, r) = utils::euclidean_div_rem(&nom_bi, &denom_bi);

            quotient_value = utils::bigint_to_fr::<E>(&q);
            remainder_value = utils::bigint_to_fr::<E>(&r);
        }

        let (quotient, remainder) = {
            let mut cs = self.cs_namespace();

            let qutioent_var = cs
                .alloc(
                    || "qutioent",
                    || quotient_value.ok_or(SynthesisError::AssignmentMissing),
                )
                .map_err(RuntimeError::SynthesisError)?;

            let remainder_var = cs
                .alloc(
                    || "remainder",
                    || remainder_value.ok_or(SynthesisError::AssignmentMissing),
                )
                .map_err(RuntimeError::SynthesisError)?;

            cs.enforce(
                || "equality",
                |lc| lc + qutioent_var,
                |lc| lc + denominator.variable,
                |lc| lc + nominator.variable - remainder_var,
            );

            mem::drop(cs);
            let args = &[left, right];
            let quotient =
                self.value_with_arguments_type_check(quotient_value, qutioent_var, args)?;
            let remainder =
                self.value_with_arguments_type_check(remainder_value, remainder_var, args)?;

            (quotient, remainder)
        };

        let abs_denominator = self.abs(denominator)?;
        let lt = self.lt(remainder.clone(), abs_denominator)?;
        let zero = self.zero_typed(remainder.data_type)?;
        let ge = self.ge(remainder.clone(), zero)?;
        let mut cs = self.cs_namespace();
        cs.enforce(
            || "0 <= rem < |denominator|",
            |lc| lc + CS::one() - lt.variable,
            |lc| lc + CS::one() - ge.variable,
            |lc| lc,
        );

        Ok((quotient, remainder))
    }

    pub fn neg(&mut self, element: Primitive<E>) -> Result<Primitive<E>, RuntimeError> {
        let neg_value = match element.value {
            Some(value) => {
                let mut neg = E::Fr::zero();
                neg.sub_assign(&value);
                Some(neg)
            }
            _ => None,
        };

        let mut cs = self.cs_namespace();

        let neg_variable = cs
            .alloc(
                || "neg variable",
                || neg_value.ok_or(SynthesisError::AssignmentMissing),
            )
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "neg constraint",
            |lc| lc + element.variable,
            |lc| lc + CS::one(),
            |lc| lc - neg_variable,
        );

        mem::drop(cs);

        if let Some(mut data_type) = element.data_type {
            data_type.signed = true;
            self.value_with_arguments_type_check(neg_value, neg_variable, &[element])
        } else {
            Ok(Primitive::new(neg_value, neg_variable))
        }
    }

    pub fn not(&mut self, element: Primitive<E>) -> Result<Primitive<E>, RuntimeError> {
        let one = self.one_typed(element.data_type)?;
        let not = self.sub(one, element.clone())?;
        self.value_with_arguments_type_check(not.value, not.variable, &[element])
    }

    pub fn and(
        &mut self,
        left: Primitive<E>,
        right: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError> {
        let mut cs = self.cs_namespace();

        let value = match (left.value, right.value) {
            (Some(a), Some(b)) => {
                let mut conj = a;
                conj.mul_assign(&b);
                Some(conj)
            }
            _ => None,
        };

        let variable = cs
            .alloc(|| "and", || value.ok_or(SynthesisError::AssignmentMissing))
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "equality",
            |lc| lc + left.variable,
            |lc| lc + right.variable,
            |lc| lc + variable,
        );

        mem::drop(cs);
        self.value_with_arguments_type_check(value, variable, &[left, right])
    }

    pub fn or(
        &mut self,
        left: Primitive<E>,
        right: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError> {
        let mut cs = self.cs_namespace();

        let value = match (left.value, right.value) {
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
            .alloc(|| "or", || value.ok_or(SynthesisError::AssignmentMissing))
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "equality",
            |lc| lc + CS::one() - left.variable,
            |lc| lc + CS::one() - right.variable,
            |lc| lc + CS::one() - variable,
        );

        mem::drop(cs);
        self.value_with_arguments_type_check(value, variable, &[left, right])
    }

    pub fn xor(
        &mut self,
        left: Primitive<E>,
        right: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError> {
        let mut cs = self.cs_namespace();

        let value = match (left.value, right.value) {
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
            .alloc(
                || "conjunction",
                || value.ok_or(SynthesisError::AssignmentMissing),
            )
            .map_err(RuntimeError::SynthesisError)?;

        // (a + a) * (b) = (a + b - c)
        cs.enforce(
            || "equality",
            |lc| lc + left.variable + left.variable,
            |lc| lc + right.variable,
            |lc| lc + left.variable + right.variable - variable,
        );

        mem::drop(cs);
        self.value_with_arguments_type_check(value, variable, &[left, right])
    }

    pub fn lt(
        &mut self,
        left: Primitive<E>,
        right: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError> {
        let one = self.one_typed(right.data_type)?;
        let right_minus_one = self.sub(right, one)?;
        self.le(left, right_minus_one)
    }

    pub fn le(
        &mut self,
        left: Primitive<E>,
        right: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError> {
        let diff = self.sub(right, left)?;

        let mut cs = self.cs_namespace();

        let diff_num = AllocatedNum::alloc(cs.namespace(|| "diff_num variable"), || {
            diff.value.ok_or(SynthesisError::AssignmentMissing)
        })
        .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "allocated_num equality",
            |lc| lc + diff.variable,
            |lc| lc + CS::one(),
            |lc| lc + diff_num.get_variable(),
        );

        let bits = diff_num
            .into_bits_le(cs.namespace(|| "diff_num bits"))
            .map_err(RuntimeError::SynthesisError)?;

        let diff_num_repacked = AllocatedNum::pack_bits_to_element(
            cs.namespace(|| "diff_num_repacked"),
            &bits[0..(E::Fr::CAPACITY as usize - 1)],
        )?;

        let lt = AllocatedNum::equals(cs.namespace(|| "equals"), &diff_num, &diff_num_repacked)?;

        mem::drop(cs);
        self.value_with_type_check(
            lt.get_value_field::<E>(),
            lt.get_variable(),
            PrimitiveType::BOOLEAN,
        )
    }

    pub fn eq(
        &mut self,
        left: Primitive<E>,
        right: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError> {
        let mut cs = self.cs_namespace();

        let l_num = AllocatedNum::alloc(cs.namespace(|| "l_num"), || {
            left.value.ok_or(SynthesisError::AssignmentMissing)
        })
        .map_err(RuntimeError::SynthesisError)?;

        let r_num = AllocatedNum::alloc(cs.namespace(|| "r_num"), || {
            right.value.ok_or(SynthesisError::AssignmentMissing)
        })
        .map_err(RuntimeError::SynthesisError)?;

        let eq = AllocatedNum::equals(cs, &l_num, &r_num).map_err(RuntimeError::SynthesisError)?;

        self.value_with_type_check(
            eq.get_value_field::<E>(),
            eq.get_variable(),
            PrimitiveType::BOOLEAN,
        )
    }

    pub fn ne(
        &mut self,
        left: Primitive<E>,
        right: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError> {
        let eq = self.eq(left, right)?;
        self.not(eq)
    }

    pub fn ge(
        &mut self,
        left: Primitive<E>,
        right: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError> {
        let not_ge = self.lt(left, right)?;
        self.not(not_ge)
    }

    pub fn gt(
        &mut self,
        left: Primitive<E>,
        right: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError> {
        let not_gt = self.le(left, right)?;
        self.not(not_gt)
    }

    pub fn conditional_select(
        &mut self,
        condition: Primitive<E>,
        if_true: Primitive<E>,
        if_false: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError> {
        let mut cs = self.cs_namespace();

        let value = match condition.value {
            Some(value) => {
                if !value.is_zero() {
                    if_true.value
                } else {
                    if_false.value
                }
            }
            None => None,
        };

        let variable = cs
            .alloc(
                || "variable",
                || value.ok_or(SynthesisError::AssignmentMissing),
            )
            .map_err(RuntimeError::SynthesisError)?;

        // Selected, Right, Left, Condition
        // s = r + c * (l - r)
        // (l - r) * (c) = (s - r)
        cs.enforce(
            || "constraint",
            |lc| lc + if_true.variable - if_false.variable,
            |lc| lc + condition.variable,
            |lc| lc + variable - if_false.variable,
        );

        mem::drop(cs);
        self.value_with_arguments_type_check(value, variable, &[if_true, if_false])
    }

    pub fn assert(&mut self, element: Primitive<E>) -> Result<(), RuntimeError> {
        let inverse_value = element
            .value
            .map(|fr| fr.inverse().unwrap_or_else(E::Fr::zero));

        let mut cs = self.cs_namespace();
        let inverse_variable = cs
            .alloc(
                || "inverse",
                || inverse_value.ok_or(SynthesisError::AssignmentMissing),
            )
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "assertion",
            |lc| lc + element.variable,
            |lc| lc + inverse_variable,
            |lc| lc + CS::one(),
        );

        Ok(())
    }

    pub fn array_get(
        &mut self,
        array: &[Primitive<E>],
        index: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError> {
        // TODO: Enable linear scan

        match index.value {
            None => unimplemented!("Variable indices are not supported"),
            Some(f) => {
                let bi = fr_to_bigint(&f);
                let i = bi.to_usize().ok_or(RuntimeError::IndexOutOfBounds)?;
                if i >= array.len() {
                    return Err(RuntimeError::IndexOutOfBounds);
                }
                Ok(array[i].clone())
            }
        }

        //        let bits = self.bits(index, array.len())?;
        //
        //        self.recursive_select(array, bits.as_slice())
    }

    pub fn array_set(
        &mut self,
        array: &[Primitive<E>],
        index: Primitive<E>,
        value: Primitive<E>,
    ) -> Result<Vec<Primitive<E>>, RuntimeError> {
        // TODO: Enable linear scan

        let mut new_array = Vec::from(array);

        match index.value {
            None => unimplemented!("Variable indices are not supported"),
            Some(f) => {
                let bi = fr_to_bigint(&f);
                let i = bi.to_usize().ok_or(RuntimeError::IndexOutOfBounds)?;
                if i >= array.len() {
                    return Err(RuntimeError::IndexOutOfBounds);
                }
                new_array[i] = value;
            }
        };

        Ok(new_array)

        //        let mut new_array = Vec::new();
        //
        //        for (i, p) in array.iter().enumerate() {
        //            let curr_index = self.constant_bigint(&i.into())?;
        //
        //            let cond = self.eq(curr_index, index.clone())?;
        //            let value = self.conditional_select(cond, value.clone(), p.clone())?;
        //            new_array.push(value);
        //        }
        //
        //        Ok(new_array)
    }

    pub fn execute<G: Gadget<E>>(
        &mut self,
        gadget: G,
        input: &[Primitive<E>],
    ) -> Result<Vec<Primitive<E>>, RuntimeError> {
        let cs = self.cs_namespace();
        gadget.synthesize_vec(cs, input)
    }
}
