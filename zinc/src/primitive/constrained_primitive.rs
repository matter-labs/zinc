use crate::primitive::{utils, Primitive, PrimitiveOperations};
use crate::vm::RuntimeError;
use bellman::pairing::Engine;
use bellman::{ConstraintSystem, Variable};
use ff::{Field, PrimeField};
use franklin_crypto::bellman::{Namespace, SynthesisError};
use franklin_crypto::circuit::num::AllocatedNum;
use num_bigint::{BigInt, ToBigInt};
use std::fmt::{Debug, Display, Error, Formatter};
use std::marker::PhantomData;

/// ConstrainedElement is an implementation of Element
/// that for every operation on elements generates corresponding R1CS constraints.
#[derive(Debug, Clone)]
pub struct FrPrimitive<E: Engine> {
    value: Option<E::Fr>,
    variable: Variable,
}

impl<E: Engine> Display for FrPrimitive<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match &self.value {
            Some(value) => {
                let bigint = utils::fr_to_bigint::<E>(value);
                Display::fmt(&bigint, f)
            }
            None => Display::fmt("none", f),
        }
    }
}

impl<E: Engine> ToBigInt for FrPrimitive<E> {
    fn to_bigint(&self) -> Option<BigInt> {
        self.value
            .map(|fr| -> BigInt { utils::fr_to_bigint::<E>(&fr) })
    }
}

impl<EN: Debug + Engine> Primitive for FrPrimitive<EN> {}

pub struct ConstrainingFrOperations<E, CS>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    cs: CS,
    counter: usize,
    pd: PhantomData<E>,
}

impl<E, CS> ConstrainingFrOperations<E, CS>
where
    E: Engine + Debug,
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

    fn zero(&mut self) -> Result<FrPrimitive<E>, RuntimeError> {
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

        Ok(FrPrimitive {
            value: Some(value),
            variable,
        })
    }

    fn one() -> FrPrimitive<E> {
        FrPrimitive {
            value: Some(E::Fr::one()),
            variable: CS::one(),
        }
    }

    #[allow(dead_code)]
    pub fn constraint_system(&mut self) -> &mut CS {
        &mut self.cs
    }

    fn abs(&mut self, value: FrPrimitive<E>) -> Result<FrPrimitive<E>, RuntimeError> {
        let zero = self.zero()?;
        let neg = PrimitiveOperations::neg(self, value.clone())?;
        let lt0 = PrimitiveOperations::lt(self, value.clone(), zero)?;
        self.conditional_select(lt0, neg, value)
    }
}

impl<E, CS> PrimitiveOperations<FrPrimitive<E>> for ConstrainingFrOperations<E, CS>
where
    E: Debug + Engine,
    CS: ConstraintSystem<E>,
{
    fn variable_none(&mut self) -> Result<FrPrimitive<E>, RuntimeError> {
        let mut cs = self.cs_namespace();

        let variable = cs
            .alloc(
                || "variable value",
                || Err(SynthesisError::AssignmentMissing),
            )
            .map_err(RuntimeError::SynthesisError)?;

        Ok(FrPrimitive {
            value: None,
            variable,
        })
    }

    fn variable_bigint(&mut self, value: &BigInt) -> Result<FrPrimitive<E>, RuntimeError> {
        let value = utils::bigint_to_fr::<E>(value)
            .ok_or_else(|| RuntimeError::InternalError("bigint_to_fr".into()))?;

        let mut cs = self.cs_namespace();

        let variable = cs
            .alloc(|| "variable value", || Ok(value))
            .map_err(RuntimeError::SynthesisError)?;

        Ok(FrPrimitive {
            value: Some(value),
            variable,
        })
    }

    fn constant_bigint(&mut self, value: &BigInt) -> Result<FrPrimitive<E>, RuntimeError> {
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

        Ok(FrPrimitive {
            value: Some(value),
            variable,
        })
    }

    fn output(
        &mut self,
        element: FrPrimitive<E>,
    ) -> Result<FrPrimitive<E>, RuntimeError> {
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

        Ok(FrPrimitive {
            value: element.value,
            variable,
        })
    }

    fn add(
        &mut self,
        left: FrPrimitive<E>,
        right: FrPrimitive<E>,
    ) -> Result<FrPrimitive<E>, RuntimeError> {
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

        Ok(FrPrimitive {
            value: sum,
            variable: sum_var,
        })
    }

    fn sub(
        &mut self,
        left: FrPrimitive<E>,
        right: FrPrimitive<E>,
    ) -> Result<FrPrimitive<E>, RuntimeError> {
        let diff = match (left.value, right.value) {
            (Some(l), Some(r)) => {
                let mut diff = l;
                diff.sub_assign(&r);
                Some(diff)
            }
            _ => None,
        };

        let mut cs = self.cs_namespace();

        let sum_var = cs
            .alloc(
                || "diff variable",
                || diff.ok_or(SynthesisError::AssignmentMissing),
            )
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "diff constraint",
            |lc| lc + left.variable - right.variable,
            |lc| lc + CS::one(),
            |lc| lc + sum_var,
        );

        Ok(FrPrimitive {
            value: diff,
            variable: sum_var,
        })
    }

    fn mul(
        &mut self,
        left: FrPrimitive<E>,
        right: FrPrimitive<E>,
    ) -> Result<FrPrimitive<E>, RuntimeError> {
        let prod = match (left.value, right.value) {
            (Some(l), Some(r)) => {
                let mut prod = l;
                prod.mul_assign(&r);
                Some(prod)
            }
            _ => None,
        };

        let mut cs = self.cs_namespace();

        let sum_var = cs
            .alloc(
                || "prod variable",
                || prod.ok_or(SynthesisError::AssignmentMissing),
            )
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "prod constraint",
            |lc| lc + left.variable,
            |lc| lc + right.variable,
            |lc| lc + sum_var,
        );

        Ok(FrPrimitive {
            value: prod,
            variable: sum_var,
        })
    }

    fn div_rem(
        &mut self,
        left: FrPrimitive<E>,
        right: FrPrimitive<E>,
    ) -> Result<(FrPrimitive<E>, FrPrimitive<E>), RuntimeError> {
        let nominator = left;
        let denominator = right;

        let mut quotient_value: Option<E::Fr> = None;
        let mut remainder_value: Option<E::Fr> = None;

        if let (Some(nom), Some(denom)) = (nominator.value, denominator.value) {
            let nom_bi = utils::fr_to_bigint::<E>(&nom);
            let denom_bi = utils::fr_to_bigint::<E>(&denom);

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

            let quotient = FrPrimitive {
                value: quotient_value,
                variable: qutioent_var,
            };
            let remainder = FrPrimitive {
                value: remainder_value,
                variable: remainder_var,
            };

            (quotient, remainder)
        };

        let abs_denominator = self.abs(denominator)?;
        let lt = self.lt(remainder.clone(), abs_denominator)?;
        let zero = self.zero()?;
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

    fn neg(
        &mut self,
        element: FrPrimitive<E>,
    ) -> Result<FrPrimitive<E>, RuntimeError> {
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

        Ok(FrPrimitive {
            value: neg_value,
            variable: neg_variable,
        })
    }

    fn not(
        &mut self,
        element: FrPrimitive<E>,
    ) -> Result<FrPrimitive<E>, RuntimeError> {
        let one = Self::one();
        self.sub(one, element)
    }

    fn and(
        &mut self,
        left: FrPrimitive<E>,
        right: FrPrimitive<E>,
    ) -> Result<FrPrimitive<E>, RuntimeError> {
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

        Ok(FrPrimitive { value, variable })
    }

    fn or(
        &mut self,
        left: FrPrimitive<E>,
        right: FrPrimitive<E>,
    ) -> Result<FrPrimitive<E>, RuntimeError> {
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

        Ok(FrPrimitive { value, variable })
    }

    fn xor(
        &mut self,
        left: FrPrimitive<E>,
        right: FrPrimitive<E>,
    ) -> Result<FrPrimitive<E>, RuntimeError> {
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

        Ok(FrPrimitive { value, variable })
    }

    fn lt(
        &mut self,
        left: FrPrimitive<E>,
        right: FrPrimitive<E>,
    ) -> Result<FrPrimitive<E>, RuntimeError> {
        let one = Self::one();
        let right_minus_one = self.sub(right, one)?;
        self.le(left, right_minus_one)
    }

    fn le(
        &mut self,
        left: FrPrimitive<E>,
        right: FrPrimitive<E>,
    ) -> Result<FrPrimitive<E>, RuntimeError> {
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
        )
        .map_err(RuntimeError::SynthesisError)?;

        let lt = AllocatedNum::equals(cs.namespace(|| "equals"), &diff_num, &diff_num_repacked)
            .map_err(RuntimeError::SynthesisError)?;

        Ok(FrPrimitive {
            value: lt.get_value_field::<E>(),
            variable: lt.get_variable(),
        })
    }

    fn eq(
        &mut self,
        left: FrPrimitive<E>,
        right: FrPrimitive<E>,
    ) -> Result<FrPrimitive<E>, RuntimeError> {
        let mut cs = self.cs_namespace();

        let l_num = AllocatedNum::alloc(cs.namespace(|| "l_num"), || {
            left.value.ok_or(SynthesisError::AssignmentMissing)
        })
        .map_err(RuntimeError::SynthesisError)?;

        let r_num = AllocatedNum::alloc(cs.namespace(|| "r_num"), || {
            right.value.ok_or(SynthesisError::AssignmentMissing)
        })
        .map_err(RuntimeError::SynthesisError)?;

        let eq =
            AllocatedNum::equals(cs, &l_num, &r_num).map_err(RuntimeError::SynthesisError)?;

        Ok(FrPrimitive {
            value: eq.get_value_field::<E>(),
            variable: eq.get_variable(),
        })
    }

    fn ne(
        &mut self,
        left: FrPrimitive<E>,
        right: FrPrimitive<E>,
    ) -> Result<FrPrimitive<E>, RuntimeError> {
        let eq = self.eq(left, right)?;
        self.not(eq)
    }

    fn ge(
        &mut self,
        left: FrPrimitive<E>,
        right: FrPrimitive<E>,
    ) -> Result<FrPrimitive<E>, RuntimeError> {
        let not_ge = self.lt(left, right)?;
        self.not(not_ge)
    }

    fn gt(
        &mut self,
        left: FrPrimitive<E>,
        right: FrPrimitive<E>,
    ) -> Result<FrPrimitive<E>, RuntimeError> {
        let not_gt = self.le(left, right)?;
        self.not(not_gt)
    }

    fn conditional_select(
        &mut self,
        condition: FrPrimitive<E>,
        if_true: FrPrimitive<E>,
        if_false: FrPrimitive<E>,
    ) -> Result<FrPrimitive<E>, RuntimeError> {
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

        Ok(FrPrimitive { value, variable })
    }

    fn assert(&mut self, element: FrPrimitive<E>) -> Result<(), RuntimeError> {
        let value = match element.value {
            None => Err(SynthesisError::AssignmentMissing),
            Some(fr) => fr.inverse().ok_or(SynthesisError::Unsatisfiable),
        };

        let mut cs = self.cs_namespace();
        let inverse_variable = cs
            .alloc(|| "inverse", || value)
            .map_err(RuntimeError::SynthesisError)?;

        cs.enforce(
            || "assertion",
            |lc| lc + element.variable,
            |lc| lc + inverse_variable,
            |lc| lc + CS::one(),
        );

        Ok(())
    }
}
