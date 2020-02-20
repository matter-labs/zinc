use std::marker::PhantomData;
use std::mem;

use ff::Field;
use bellman::{ConstraintSystem, Namespace, SynthesisError};
use franklin_crypto::circuit::num::AllocatedNum;
use num_bigint::BigInt;
use num_traits::ToPrimitive;

use crate::core::RuntimeError;
use crate::gadgets::{utils, Gadget, IntegerType, Scalar, ScalarType, ScalarTypeExpectation};
use crate::{Engine, gadgets};
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

    fn zero(&self, scalar_type: ScalarType) -> Scalar<E> {
        self.constant_bigint(&0.into(), scalar_type).expect("can't overflow")
    }

    fn one(&self, scalar_type: ScalarType) -> Scalar<E> {
        self.constant_bigint(&1.into(), scalar_type).expect("can't overflow")
    }

    #[allow(dead_code)]
    pub fn constraint_system(&mut self) -> &mut CS {
        &mut self.cs
    }

    fn abs(&mut self, value: Scalar<E>) -> Result<Scalar<E>, RuntimeError> {
        match value.get_type() {
            ScalarType::Field | ScalarType::Boolean => return Ok(value),
            ScalarType::Integer(int_type) => {
                if !int_type.signed {
                    return Ok(value);
                }
            }
        }

        let zero = self.zero(value.get_type());
        let neg = Gadgets::neg(self, value.clone())?;
        let lt0 = Gadgets::lt(self, value.clone(), zero)?;
        self.conditional_select(lt0, neg, value)
    }
}

impl<E, CS> Gadgets<E, CS>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    pub fn variable_none(&mut self, scalar_type: ScalarType) -> Result<Scalar<E>, RuntimeError> {
        let mut cs = self.cs_namespace();

        let variable = cs
            .alloc(
                || "variable value",
                || Err(SynthesisError::AssignmentMissing),
            )
            .map_err(RuntimeError::SynthesisError)?;

        mem::drop(cs);

        Ok(Scalar::new_unchecked_variable(None, variable, scalar_type))
    }

    pub fn variable_bigint(
        &mut self,
        value: &BigInt,
        data_type: ScalarType,
    ) -> Result<Scalar<E>, RuntimeError> {
        let value = utils::bigint_to_fr::<E>(value)
            .ok_or_else(|| RuntimeError::InternalError("bigint_to_fr".into()))?;

        let mut cs = self.cs_namespace();

        let variable = cs
            .alloc(|| "variable value", || Ok(value))
            .map_err(RuntimeError::SynthesisError)?;

        mem::drop(cs);

        Ok(Scalar::new_unchecked_variable(Some(value), variable, data_type))
    }

    pub fn constant_bigint(
        &self,
        value: &BigInt,
        scalar_type: ScalarType,
    ) -> Result<Scalar<E>, RuntimeError> {
        let value = utils::bigint_to_fr::<E>(value)
            .ok_or_else(|| RuntimeError::ValueOverflow {
                value: value.clone(),
                scalar_type
            })?;

        Ok(Scalar::new_unchecked_constant(value, scalar_type))
    }

    pub fn output(&mut self, element: Scalar<E>) -> Result<Scalar<E>, RuntimeError> {
        let mut cs = self.cs_namespace();

        let variable = cs
            .alloc_input(
                || "output value",
                || element.grab_value(),
            )
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "enforce output equality",
            |lc| lc + variable,
            |lc| lc + CS::one(),
            |lc| lc + &element.lc::<CS>(),
        );

        Ok(Scalar::new_unchecked_variable(element.get_value(), variable, element.get_type()))
    }

    pub fn add(
        &mut self,
        left: Scalar<E>,
        right: Scalar<E>,
    ) -> Result<Scalar<E>, RuntimeError> {
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

        Ok(Scalar::new_unchecked_variable(sum, sum_var, ScalarType::Field))
    }

    pub fn sub(
        &mut self,
        left: Scalar<E>,
        right: Scalar<E>,
    ) -> Result<Scalar<E>, RuntimeError> {
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
        Ok(Scalar::new_unchecked_variable(diff, diff_var, ScalarType::Field))
    }

    pub fn mul(
        &mut self,
        left: Scalar<E>,
        right: Scalar<E>,
    ) -> Result<Scalar<E>, RuntimeError> {
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
            .alloc(
                || "prod variable",
                || prod.grab(),
            )
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "prod constraint",
            |lc| lc + &left.lc::<CS>(),
            |lc| lc + &right.lc::<CS>(),
            |lc| lc + prod_var,
        );

        Ok(Scalar::new_unchecked_variable(prod, prod_var, ScalarType::Field))
    }

    // Make condition
    pub fn div_rem_conditional(
        &mut self,
        left: Scalar<E>,
        right: Scalar<E>,
        condition: Scalar<E>,
    ) -> Result<(Scalar<E>, Scalar<E>), RuntimeError> {
        let one = self.one(right.get_type());
        let denom = self.conditional_select(condition, right, one)?;
        self.div_rem(left, denom)
    }

    fn div_rem(
        &mut self,
        left: Scalar<E>,
        right: Scalar<E>,
    ) -> Result<(Scalar<E>, Scalar<E>), RuntimeError> {
        let nominator = left;
        let denominator = right;

        let mut quotient_value: Option<E::Fr> = None;
        let mut remainder_value: Option<E::Fr> = None;

        if let (Some(nom), Some(denom)) = (nominator.get_value(), denominator.get_value()) {
            let nom_bi = utils::fr_to_bigint(&nom, nominator.is_signed());
            let denom_bi = utils::fr_to_bigint(&denom, denominator.is_signed());

            let (q, r) = utils::euclidean_div_rem(&nom_bi, &denom_bi)
                .ok_or(RuntimeError::DivisionByZero)?;

            quotient_value = utils::bigint_to_fr::<E>(&q);
            remainder_value = utils::bigint_to_fr::<E>(&r);
        }

        let (quotient, remainder) = {
            let mut cs = self.cs_namespace();

            let qutioent_var = cs
                .alloc(
                    || "qutioent",
                    || quotient_value.grab(),
                )
                .map_err(RuntimeError::SynthesisError)?;

            let remainder_var = cs
                .alloc(
                    || "remainder",
                    || remainder_value.grab(),
                )
                .map_err(RuntimeError::SynthesisError)?;

            cs.enforce(
                || "equality",
                |lc| lc + qutioent_var,
                |lc| lc + &denominator.lc::<CS>(),
                |lc| lc + &nominator.lc::<CS>() - remainder_var,
            );

            mem::drop(cs);
            let quotient = Scalar::new_unchecked_variable(quotient_value, qutioent_var, ScalarType::Field);
            let remainder = Scalar::new_unchecked_variable(remainder_value, remainder_var, ScalarType::Field);

            (quotient, remainder)
        };

        let abs_denominator = self.abs(denominator)?;
        let lt = self.lt(remainder.as_field(), abs_denominator.as_field())?;
        let zero = self.zero(remainder.get_type());
        let ge = self.ge(remainder.clone(), zero)?;
        let mut cs = self.cs_namespace();
        cs.enforce(
            || "0 <= rem < |denominator|",
            |lc| lc + CS::one() - &lt.lc::<CS>(),
            |lc| lc + CS::one() - &ge.lc::<CS>(),
            |lc| lc,
        );

        Ok((quotient, remainder))
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
            .alloc(
                || "neg variable",
                || neg_value.grab(),
            )
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "neg constraint",
            |lc| lc + &element.lc::<CS>(),
            |lc| lc + CS::one(),
            |lc| lc - neg_variable,
        );

        mem::drop(cs);

        let new_type = match element.get_type() {
            t @ ScalarType::Field => t,
            _t @ ScalarType::Boolean => IntegerType {
                signed: true,
                length: 1,
            }
            .into(),
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
        Ok(Scalar::new_unchecked_variable(neg_value, neg_variable, new_type))
    }

    pub fn not(&mut self, element: Scalar<E>) -> Result<Scalar<E>, RuntimeError> {
        let one = self.one(element.get_type());
        self.sub(one, element)
    }

    pub fn and(
        &mut self,
        left: Scalar<E>,
        right: Scalar<E>,
    ) -> Result<Scalar<E>, RuntimeError> {
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

        Ok(Scalar::new_unchecked_variable(value, variable, ScalarType::Field))
    }

    pub fn or(
        &mut self,
        left: Scalar<E>,
        right: Scalar<E>,
    ) -> Result<Scalar<E>, RuntimeError> {
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

        Ok(Scalar::new_unchecked_variable(value, variable, ScalarType::Field))
    }

    pub fn xor(
        &mut self,
        left: Scalar<E>,
        right: Scalar<E>,
    ) -> Result<Scalar<E>, RuntimeError> {
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
            .alloc(
                || "conjunction",
                || value.grab(),
            )
            .map_err(RuntimeError::SynthesisError)?;

        // (a + a) * (b) = (a + b - c)
        cs.enforce(
            || "equality",
            |lc| lc + &left.lc::<CS>() + &left.lc::<CS>(),
            |lc| lc + &right.lc::<CS>(),
            |lc| lc + &left.lc::<CS>() + &right.lc::<CS>() - variable,
        );

        Ok(Scalar::new_unchecked_variable(value, variable, ScalarType::Boolean))
    }

    pub fn lt(
        &mut self,
        left: Scalar<E>,
        right: Scalar<E>,
    ) -> Result<Scalar<E>, RuntimeError> {
        let cs = self.cs_namespace();
        gadgets::comparison::less_than(cs, left, right)
    }

    pub fn le(
        &mut self,
        left: Scalar<E>,
        right: Scalar<E>,
    ) -> Result<Scalar<E>, RuntimeError> {
        let gt = self.gt(left, right)?;
        self.not(gt)
    }

    pub fn eq(
        &mut self,
        left: Scalar<E>,
        right: Scalar<E>,
    ) -> Result<Scalar<E>, RuntimeError> {
        let mut cs = self.cs_namespace();

        let l_num = AllocatedNum::alloc(cs.namespace(|| "l_num"), || {
            left.grab_value()
        })
        .map_err(RuntimeError::SynthesisError)?;

        let r_num = AllocatedNum::alloc(cs.namespace(|| "r_num"), || {
            right.grab_value()
        })
        .map_err(RuntimeError::SynthesisError)?;

        let eq = AllocatedNum::equals(cs, &l_num, &r_num).map_err(RuntimeError::SynthesisError)?;

        Ok(Scalar::new_unchecked_variable(
            eq.get_value_field::<E>(),
            eq.get_variable(),
            ScalarType::Boolean,
        ))
    }

    pub fn ne(
        &mut self,
        left: Scalar<E>,
        right: Scalar<E>,
    ) -> Result<Scalar<E>, RuntimeError> {
        let eq = self.eq(left, right)?;
        self.not(eq)
    }

    pub fn ge(
        &mut self,
        left: Scalar<E>,
        right: Scalar<E>,
    ) -> Result<Scalar<E>, RuntimeError> {
        self.le(right, left)
    }

    pub fn gt(
        &mut self,
        left: Scalar<E>,
        right: Scalar<E>,
    ) -> Result<Scalar<E>, RuntimeError> {
        self.lt(right, left)
    }

    pub fn conditional_select(
        &mut self,
        condition: Scalar<E>,
        if_true: Scalar<E>,
        if_false: Scalar<E>,
    ) -> Result<Scalar<E>, RuntimeError> {
        let mut cs = self.cs_namespace();
        let scalar_type = ScalarType::expect_same(if_true.get_type(), if_false.get_type())?;

        let value = match condition.get_value() {
            Some(value) => {
                if !value.is_zero() {
                    if_true.get_value()
                } else {
                    if_false.get_value()
                }
            }
            None => None,
        };

        let variable = cs
            .alloc(
                || "variable",
                || value.grab(),
            )
            .map_err(RuntimeError::SynthesisError)?;

        // Selected, Right, Left, Condition
        // s = r + c * (l - r)
        // (l - r) * (c) = (s - r)
        cs.enforce(
            || "constraint",
            |lc| lc + &if_true.lc::<CS>() - &if_false.lc::<CS>(),
            |lc| lc + &condition.lc::<CS>(),
            |lc| lc + variable - &if_false.lc::<CS>(),
        );

        Ok(Scalar::new_unchecked_variable(value, variable, scalar_type))
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
            .alloc(
                || "inverse",
                || inverse_value.grab(),
            )
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "assertion",
            |lc| lc + &element.lc::<CS>(),
            |lc| lc + inverse_variable,
            |lc| lc + CS::one(),
        );

        Ok(())
    }

    pub fn array_get(
        &mut self,
        array: &[Scalar<E>],
        index: Scalar<E>,
    ) -> Result<Scalar<E>, RuntimeError> {
        // TODO: Enable linear scan

        match index.get_value() {
            None => Err(RuntimeError::Unimplemented(
                "runtime-variable array indices are not supported".into(),
            )),
            Some(f) => {
                let bi = utils::fr_to_bigint(&f, index.is_signed());
                let i = bi.to_usize().ok_or(RuntimeError::ExpectedUsize(bi))?;
                if i >= array.len() {
                    return Err(RuntimeError::IndexOutOfBounds {
                        lower_bound: 0,
                        upper_bound: array.len(),
                        actual: i,
                    });
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
        array: &[Scalar<E>],
        index: Scalar<E>,
        value: Scalar<E>,
    ) -> Result<Vec<Scalar<E>>, RuntimeError> {
        // TODO: Enable linear scan

        let mut new_array = Vec::from(array);

        match index.get_value() {
            None => {
                return Err(RuntimeError::Unimplemented(
                    "runtime-variable array indices are not supported".into(),
                ))
            }
            Some(f) => {
                let bi = utils::fr_to_bigint(&f, index.is_signed());
                let i = bi.to_usize().ok_or(RuntimeError::ExpectedUsize(bi))?;
                if i >= array.len() {
                    return Err(RuntimeError::IndexOutOfBounds {
                        lower_bound: 0,
                        upper_bound: array.len(),
                        actual: i,
                    });
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
        input: &[Scalar<E>],
    ) -> Result<Vec<Scalar<E>>, RuntimeError> {
        let cs = self.cs_namespace();
        gadget.synthesize_vec(cs, input)
    }

    /// Asserts that value is in its type range if condition is true.
    pub fn assert_type(
        &mut self,
        condition: Scalar<E>,
        scalar: Scalar<E>,
        scalar_type: ScalarType,
    ) -> Result<Scalar<E>, RuntimeError> {
        match scalar_type {
            ScalarType::Field => {
                // Always safe to cast into field
                Ok(scalar.as_field())
            }
            ScalarType::Boolean => {
                let checked = self.assert_type(condition, scalar, IntegerType::BIT.into())?;
                Ok(checked.with_type_unchecked(scalar_type))
            }
            ScalarType::Integer(int_type) => {
                let scalar_with_offset = if !int_type.signed {
                    scalar.clone()
                } else {
                    let offset_value = BigInt::from(1) << (int_type.length - 1);
                    let offset = self.constant_bigint(&offset_value, ScalarType::Field)?;
                    self.add(scalar.clone(), offset)?
                };

                let zero = self.zero(scalar_with_offset.get_type());
                let value_or_zero =
                    self.conditional_select(condition.clone(), scalar_with_offset, zero)?;

                {
                    let mut cs = self.cs_namespace();
                    let _bits = value_or_zero
                        .to_expression::<CS>()
                        .into_bits_le_fixed(cs.namespace(|| "into_bits"), int_type.length)?;
                }

                if let (Some(value), Some(condition)) = (&scalar.get_value(), &condition.get_value()) {
                    if !condition.is_zero() {
                        let value_bigint = utils::fr_to_bigint(value, int_type.signed);
                        let (lower_bound, upper_bound) = if int_type.signed {
                            let lower_bound = -(BigInt::from(1) << (int_type.length - 1));
                            let upper_bound = (-lower_bound.clone()) - 1;
                            (lower_bound, upper_bound)
                        } else {
                            let lower_bound = BigInt::from(0);
                            let upper_bound = (BigInt::from(1) << int_type.length) - 1;
                            (lower_bound, upper_bound)
                        };

                        if value_bigint < lower_bound || value_bigint > upper_bound {
                            return Err(RuntimeError::ValueOverflow {
                                value: value_bigint,
                                scalar_type,
                            });
                        }
                    }
                }

                Ok(scalar.with_type_unchecked(scalar_type))
            }
        }
    }
}
