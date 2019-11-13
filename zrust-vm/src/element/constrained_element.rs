use bellman::{Variable, ConstraintSystem};
use bellman::pairing::Engine;
use ff::{Field, PrimeField};
use franklin_crypto::bellman::{SynthesisError, Namespace};
use crate::element::{ElementOperator, Element, utils};
use crate::RuntimeError;
use std::marker::PhantomData;
use num_bigint::{BigInt, ToBigInt};
use num_integer::Integer;
use std::fmt::{Debug, Display, Formatter, Error};

#[derive(Debug, Clone)]
pub struct ConstrainedElement<E: Engine> {
    value: Option<E::Fr>,
    variable: Variable,
}

impl <E: Engine> Display for ConstrainedElement<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match &self.value {
            Some(value) => {
                let bigint = utils::fr_to_bigint::<E>(value);
                Display::fmt(&bigint, f)
            },
            None => Display::fmt("none", f)
        }
    }
}

impl <E: Engine> ToBigInt for ConstrainedElement<E> {
    fn to_bigint(&self) -> Option<BigInt> {
        self.value.map(|fr| -> BigInt { utils::fr_to_bigint::<E>(&fr) })
    }
}

impl <EN: Debug + Engine> Element for ConstrainedElement<EN> {}

pub struct ConstrainedElementOperator<E, CS>
where
    E: Engine,
    CS: ConstraintSystem<E>
{
    cs: CS,
    counter: usize,
    pd: PhantomData<E>,
}

impl <E, CS> ConstrainedElementOperator<E, CS>
    where
        E: Engine,
        CS: ConstraintSystem<E>
{
    pub fn new(cs: CS) -> Self {
        Self {
            cs,
            counter: 0,
            pd: PhantomData
        }
    }

    fn cs_namespace(&mut self) -> Namespace<E, CS::Root> {
        let s = format!("{}", self.counter);
        self.counter += 1;
        self.cs.namespace(|| s)
    }
}

impl <E, CS> ElementOperator<ConstrainedElement<E>> for ConstrainedElementOperator<E, CS>
where
    E: Debug + Engine,
    CS: ConstraintSystem<E>
{
    fn constant_u64(&mut self, value: u64) -> Result<ConstrainedElement<E>, RuntimeError> {
        let val = E::Fr::from_str(&value.to_string()).ok_or(RuntimeError::InternalError)?;

        let mut cs = self.cs_namespace();

        let var = cs.alloc(
            || "constant value",
            || Ok(val))
            .map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "constant constraint",
            |lc| lc + CS::one(),
            |lc| lc + (val, CS::one()),
            |lc| lc + var,
        );

        Ok(ConstrainedElement {
            value: Some(val),
            variable: var
        })
    }

    fn constant_bigint(&mut self, value: &BigInt) -> Result<ConstrainedElement<E>, RuntimeError> {
        let value = utils::bigint_to_fr::<E>(value).ok_or(RuntimeError::InternalError)?;

        let mut cs = self.cs_namespace();

        let variable = cs.alloc(
            || "constant value",
            || Ok(value))
            .map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "constant equation",
            |lc| lc + CS::one(),
            |lc| lc + (value, CS::one()),
            |lc| lc + variable,
        );

        Ok(ConstrainedElement { value: Some(value), variable })
    }

    fn add(&mut self, left: ConstrainedElement<E>, right: ConstrainedElement<E>)
        -> Result<ConstrainedElement<E>, RuntimeError>
    {
        let sum = match (left.value, right.value) {
            (Some(l), Some(r)) => {
                let mut sum = l;
                sum.add_assign(&r);
                Some(sum)
            }
            _ => None
        };

        let mut cs = self.cs_namespace();

        let sum_var = cs.alloc(
            || "sum variable",
            || sum.ok_or(SynthesisError::AssignmentMissing))
            .map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "sum constraint",
            |lc| lc + left.variable + right.variable,
            |lc| lc + CS::one(),
            |lc| lc + sum_var,
        );

        Ok(ConstrainedElement {
            value: sum,
            variable: sum_var,
        })
    }

    fn sub(&mut self, left: ConstrainedElement<E>, right: ConstrainedElement<E>) -> Result<ConstrainedElement<E>, RuntimeError> {
        let diff = match (left.value, right.value) {
            (Some(l), Some(r)) => {
                let mut diff = l;
                diff.sub_assign(&r);
                Some(diff)
            }
            _ => None
        };

        let mut cs = self.cs_namespace();

        let sum_var = cs.alloc(
            || "diff variable",
            || diff.ok_or(SynthesisError::AssignmentMissing))
            .map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "diff constraint",
            |lc| lc + left.variable - right.variable,
            |lc| lc + CS::one(),
            |lc| lc + sum_var,
        );

        Ok(ConstrainedElement {
            value: diff,
            variable: sum_var,
        })
    }

    fn mul(&mut self, left: ConstrainedElement<E>, right: ConstrainedElement<E>) -> Result<ConstrainedElement<E>, RuntimeError> {
        let prod = match (left.value, right.value) {
            (Some(l), Some(r)) => {
                let mut prod = l;
                prod.mul_assign(&r);
                Some(prod)
            }
            _ => None
        };

        let mut cs = self.cs_namespace();

        let sum_var = cs.alloc(
            || "prod variable",
            || prod.ok_or(SynthesisError::AssignmentMissing))
            .map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "prod constraint",
            |lc| lc + left.variable,
            |lc| lc + right.variable,
            |lc| lc + sum_var,
        );

        Ok(ConstrainedElement {
            value: prod,
            variable: sum_var,
        })
    }

    fn div_rem(&mut self, left: ConstrainedElement<E>, right: ConstrainedElement<E>)
        -> Result<(ConstrainedElement<E>, ConstrainedElement<E>), RuntimeError>
    {
        let nominator = left;
        let denominator = right;

        let mut quotient: Option<E::Fr> = None;
        let mut remainder: Option<E::Fr> = None;

        if let (Some(nom), Some(denom)) = (nominator.value, denominator.value) {
            let nom_bi = utils::fr_to_bigint::<E>(&nom);
            let denom_bi = utils::fr_to_bigint::<E>(&denom);

            let (q, r) = nom_bi.div_rem(&denom_bi);

            quotient = utils::bigint_to_fr::<E>(&q);
            remainder = utils::bigint_to_fr::<E>(&r);
        }

        let mut cs = self.cs_namespace();

        let qutioent_var = cs.alloc(
            || "qutioent",
            || quotient.ok_or(SynthesisError::AssignmentMissing))
            .map_err(|_| RuntimeError::SynthesisError)?;

        let remainder_var = cs.alloc(
            || "remainder",
            || remainder.ok_or(SynthesisError::AssignmentMissing))
            .map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "equality",
            |lc| lc + qutioent_var,
            |lc| lc + denominator.variable,
            |lc| lc + nominator.variable - remainder_var
        );

        // TODO: add constraint `rem < denom`

        Ok((
            ConstrainedElement { value: quotient, variable: qutioent_var },
            ConstrainedElement { value: remainder, variable: remainder_var }
        ))
    }

    fn neg(&mut self, element: ConstrainedElement<E>) -> Result<ConstrainedElement<E>, RuntimeError> {
        let neg_value = match element.value {
            Some(value) => {
                let mut neg = E::Fr::zero();
                neg.sub_assign(&value);
                Some(neg)
            }
            _ => None
        };

        let mut cs = self.cs_namespace();

        let neg_variable = cs.alloc(
            || "neg variable",
            || neg_value.ok_or(SynthesisError::AssignmentMissing))
            .map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "neg constraint",
            |lc| lc + element.variable,
            |lc| lc + CS::one(),
            |lc| lc - neg_variable,
        );

        Ok(ConstrainedElement {
            value: neg_value,
            variable: neg_variable,
        })
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use franklin_crypto::circuit::test::TestConstraintSystem;
    use bellman::pairing::bn256::Bn256;

    #[test]
    fn test_constrained_element() {
        let cs = TestConstraintSystem::<Bn256>::new();
        let mut operator = ConstrainedElementOperator::new(cs);
        let a = operator.constant_u64(42).unwrap();
        let b = operator.constant_u64(7).unwrap();
        let _ = operator.add(a, b).unwrap();
    }
}
