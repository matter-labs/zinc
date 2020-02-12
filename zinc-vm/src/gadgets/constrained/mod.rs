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
use crate::gadgets::{utils, Gadget, Primitive, ScalarType, IntegerType};
use num_traits::ToPrimitive;
use std::mem;
use crate::gadgets::tmp_lt::less_than;

impl<E: Engine> Debug for Primitive<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let value_str = self.value
            .map(|f| fr_to_bigint(&f).to_string())
            .unwrap_or_else(|| "none".into());

        write!(f, "Scalar {{ value: {}, type: {} }}", value_str, self.scalar_type)
    }
}

impl<E: Engine> Primitive<E> {
    fn new(value: Option<E::Fr>, variable: Variable, scalar_type: ScalarType) -> Self {
        Self { value, variable, scalar_type }
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
        let value_str = self.value
            .map(|f| fr_to_bigint(&f).to_string())
            .unwrap_or_else(|| "none".into());

        write!(f, "{} as {}", value_str, self.scalar_type)
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

    fn zero(
        &mut self,
        scalar_type: ScalarType,
    ) -> Result<Primitive<E>, RuntimeError> {
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

        Ok(Primitive::new(Some(value), variable, scalar_type))
    }

    fn one(scalar_type: ScalarType) -> Primitive<E> {
        Primitive::new(Some(E::Fr::one()), CS::one(), scalar_type)
    }

    #[allow(dead_code)]
    pub fn constraint_system(&mut self) -> &mut CS {
        &mut self.cs
    }

    fn abs(&mut self, value: Primitive<E>) -> Result<Primitive<E>, RuntimeError> {
        match value.scalar_type {
            ScalarType::Field => return Ok(value),
            ScalarType::Integer(int_type) => {
                if !int_type.signed {
                    return Ok(value);
                }
            },
        }

        let zero = self.zero(value.scalar_type)?;
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
    pub fn variable_none(
        &mut self,
        scalar_type: ScalarType,
    ) -> Result<Primitive<E>, RuntimeError> {
        let mut cs = self.cs_namespace();

        let variable = cs
            .alloc(
                || "variable value",
                || Err(SynthesisError::AssignmentMissing),
            )
            .map_err(RuntimeError::SynthesisError)?;

        mem::drop(cs);

        Ok(Primitive::new(None, variable, scalar_type))
    }

    pub fn variable_bigint(
        &mut self,
        value: &BigInt,
        data_type: ScalarType,
    ) -> Result<Primitive<E>, RuntimeError> {
        let value = utils::bigint_to_fr::<E>(value)
            .ok_or_else(|| RuntimeError::InternalError("bigint_to_fr".into()))?;

        let mut cs = self.cs_namespace();

        let variable = cs
            .alloc(|| "variable value", || Ok(value))
            .map_err(RuntimeError::SynthesisError)?;

        mem::drop(cs);

        Ok(Primitive::new(Some(value), variable, data_type))
    }

    pub fn constant_bigint(&mut self, value: &BigInt, scalar_type: ScalarType) -> Result<Primitive<E>, RuntimeError> {
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

        Ok(Primitive::new(Some(value), variable, scalar_type))
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

        Ok(Primitive::new(element.value, variable, element.scalar_type))
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

        Ok(Primitive::new(sum, sum_var, ScalarType::Field))
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
        Ok(Primitive::new(diff, diff_var, ScalarType::Field))
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

        Ok(Primitive::new(prod, prod_var, ScalarType::Field))
    }

    // Make condition
    pub fn div_rem_conditional(
        &mut self,
        left: Primitive<E>,
        right: Primitive<E>,
        condition: Primitive<E>,
    ) -> Result<(Primitive<E>, Primitive<E>), RuntimeError> {
        let one = Self::one(right.scalar_type);
        let denom = self.conditional_select(condition, right, one)?;
        self.div_rem(left, denom)
    }

    fn div_rem(
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

            let (q, r) = utils::euclidean_div_rem(&nom_bi, &denom_bi)
                .ok_or(RuntimeError::ZeroDivisionError)?;

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
            let _args = &[left, right];
            let quotient = Primitive::new(quotient_value, qutioent_var, ScalarType::Field);
            let remainder = Primitive::new(remainder_value, remainder_var, ScalarType::Field);

            (quotient, remainder)
        };

        let abs_denominator = self.abs(denominator)?;
        let lt = self.lt(remainder.clone(), abs_denominator)?;
        let zero = self.zero(remainder.scalar_type)?;
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

        let new_type = match element.scalar_type {
            t @ ScalarType::Field => t,
            t @ScalarType::Integer(IntegerType { signed: true, .. }) => t,
            ScalarType::Integer(IntegerType { signed: false, length }) => IntegerType {
                signed: true,
                length: length + 1,
            }.into(),
        };
        Ok(Primitive::new(neg_value, neg_variable, new_type))
    }

    pub fn not(&mut self, element: Primitive<E>) -> Result<Primitive<E>, RuntimeError> {
        let one = Self::one(element.scalar_type);
        self.sub(one, element.clone())
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

        Ok(Primitive::new(value, variable, ScalarType::Field))
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

        Ok(Primitive::new(value, variable, ScalarType::Field))
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

        Ok(Primitive::new(value, variable, IntegerType::BOOLEAN.into()))
    }

    pub fn lt(
        &mut self,
        mut left: Primitive<E>,
        mut right: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError> {
        let mut cs = self.cs_namespace();

        let l = left.as_allocated_num(cs.namespace(|| "left num"))?;
        let r = right.as_allocated_num(cs.namespace(|| "right num"))?;

        let lt = less_than(cs.namespace(|| "less than"), &l, &r)?;

        Ok(Primitive::new(
            lt.get_value_field::<E>(),
            lt.get_variable().expect("must allocate").get_variable(),
            IntegerType::BOOLEAN.into()
        ))
    }

    pub fn le(
        &mut self,
        mut left: Primitive<E>,
        mut right: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError> {
        let gt = self.gt(left, right)?;
        self.not(gt)
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

        Ok(Primitive::new(eq.get_value_field::<E>(), eq.get_variable(), IntegerType::BOOLEAN.into()))
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
        self.le(right, left)
    }

    pub fn gt(
        &mut self,
        left: Primitive<E>,
        right: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError> {
        self.lt(right, left)
    }

    pub fn conditional_select(
        &mut self,
        condition: Primitive<E>,
        if_true: Primitive<E>,
        if_false: Primitive<E>,
    ) -> Result<Primitive<E>, RuntimeError> {
        let mut cs = self.cs_namespace();
        let scalar_type = ScalarType::expect_same(if_true.get_type(), if_false.get_type())?;

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

        Ok(Primitive::new(value, variable, scalar_type))
    }

    pub fn assert(
        &mut self,
        element: Primitive<E>,
        message: Option<&str>,
    ) -> Result<(), RuntimeError> {
        if let Some(value) = element.value {
            if value.is_zero() {
                let s = message.unwrap_or("<no message>".into());
                return Err(RuntimeError::AssertionError(s.into()));
            }
        }

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
        input: &[Primitive<E>],
    ) -> Result<Vec<Primitive<E>>, RuntimeError> {
        let cs = self.cs_namespace();
        gadget.synthesize_vec(cs, input)
    }

    /// Asserts that value is in its type range if condition is true.
    pub fn assert_type(&mut self, condition: Primitive<E>, scalar: Primitive<E>, scalar_type: ScalarType) -> Result<Primitive<E>, RuntimeError>{
        match scalar_type {
            ScalarType::Field => {
                // Always safe to cast into field
                Ok(Primitive::new(
                    scalar.value,
                    scalar.variable,
                    scalar_type
                ))
            },
            ScalarType::Integer(int_type) => {
                let scalar_with_offset = if !int_type.signed {
                    scalar.clone()
                } else {
                    let offset_value = BigInt::from(1) << (int_type.length - 1);
                    let offset = self.constant_bigint(&offset_value, ScalarType::Field)?;
                    self.add(scalar.clone(), offset)?
                };

                let upper_bound_value = BigInt::from(1) << int_type.length;
                let upper_bound = self.constant_bigint(&upper_bound_value, ScalarType::Field)?;
                let lt = self.lt(scalar_with_offset.clone(), upper_bound.clone())?;
                let false_branch = self.not(condition.clone())?;
                let required = self.or(lt.clone(), false_branch)?;


                // Since we are not forcing type checks in false branch we will reset value.
                let zero = self.zero(scalar.get_type())?;
                let new_scalar = self.conditional_select(condition, scalar.clone(), zero)?;

                match self.assert(required, None) {
                    Ok(()) => Ok(Primitive::new(new_scalar.value, new_scalar.variable, scalar_type)),
                    Err(RuntimeError::AssertionError(_)) => {
                        Err(RuntimeError::ValueOverflow {
                            value: fr_to_bigint(&scalar.value.expect("if assert failed, value is known")),
                            scalar_type
                        })
                    },
                    Err(e) => Err(e)
                }
            },
        }
    }
}
