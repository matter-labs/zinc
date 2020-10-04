//!
//! The logging constraint system.
//!

use std::marker::PhantomData;

use num::BigInt;
use num::Signed;

use franklin_crypto::bellman::pairing::Engine;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::bellman::Index;
use franklin_crypto::bellman::LinearCombination;
use franklin_crypto::bellman::SynthesisError;
use franklin_crypto::bellman::Variable;

use crate::gadgets;

pub struct Logging<E, CS>(CS, PhantomData<E>)
where
    E: Engine,
    CS: ConstraintSystem<E>;

impl<E, CS> Logging<E, CS>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    pub fn new(cs: CS) -> Self {
        Self(cs, PhantomData)
    }
}

impl<E, CS> ConstraintSystem<E> for Logging<E, CS>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    type Root = Self;

    fn alloc<F, A, AR>(&mut self, annotation: A, f: F) -> Result<Variable, SynthesisError>
    where
        F: FnOnce() -> Result<E::Fr, SynthesisError>,
        A: FnOnce() -> AR,
        AR: Into<String>,
    {
        let annotation = annotation().into();
        let mut value = None;
        let variable = self.0.alloc(
            || &annotation,
            || {
                let tmp = f()?;
                value = Some(tmp);
                Ok(tmp)
            },
        )?;
        log::trace!(
            "r1cs: witness: name = \"{}\", value = {:?}, index = {:?}",
            annotation,
            value,
            variable.get_unchecked()
        );
        Ok(variable)
    }

    fn alloc_input<F, A, AR>(&mut self, annotation: A, f: F) -> Result<Variable, SynthesisError>
    where
        F: FnOnce() -> Result<E::Fr, SynthesisError>,
        A: FnOnce() -> AR,
        AR: Into<String>,
    {
        let annotation = annotation().into();
        let mut value = None;
        let variable = self.0.alloc_input(
            || &annotation,
            || {
                let tmp = f()?;
                value = Some(tmp);
                Ok(tmp)
            },
        )?;
        log::trace!(
            "r1cs: input: name = \"{}\", value = {:?}, index = {:?}",
            annotation,
            value,
            variable.get_unchecked()
        );
        Ok(variable)
    }

    fn enforce<A, AR, LA, LB, LC>(&mut self, annotation: A, a: LA, b: LB, c: LC)
    where
        A: FnOnce() -> AR,
        AR: Into<String>,
        LA: FnOnce(LinearCombination<E>) -> LinearCombination<E>,
        LB: FnOnce(LinearCombination<E>) -> LinearCombination<E>,
        LC: FnOnce(LinearCombination<E>) -> LinearCombination<E>,
    {
        let annotation = annotation().into();
        let lc_a = a(LinearCombination::zero());
        let lc_b = b(LinearCombination::zero());
        let lc_c = c(LinearCombination::zero());
        log::trace!(
            "r1cs: constraint: ({}) * ({}) = ({}), name = {}",
            lc_to_string(&lc_a),
            lc_to_string(&lc_b),
            lc_to_string(&lc_c),
            annotation,
        );

        self.0.enforce(|| annotation, |_| lc_a, |_| lc_b, |_| lc_c)
    }

    fn push_namespace<NR, N>(&mut self, name_fn: N)
    where
        NR: Into<String>,
        N: FnOnce() -> NR,
    {
        self.0.get_root().push_namespace(name_fn);
    }

    fn pop_namespace(&mut self) {
        self.0.get_root().pop_namespace();
    }

    fn get_root(&mut self) -> &mut Self::Root {
        self
    }
}

fn lc_to_string<E: Engine>(lc: &LinearCombination<E>) -> String {
    let mut string = String::new();

    let mut is_first = true;
    for (var, c) in lc.as_ref() {
        let c_value = gadgets::scalar::fr_bigint::fr_to_bigint::<E>(c, true);

        let mut c_str = if c_value == BigInt::from(1) {
            " + ".into()
        } else if c_value == BigInt::from(-1) {
            " - ".into()
        } else if c_value.is_negative() {
            String::from(" - ") + &(-c_value).to_string() + " * "
        } else {
            String::from(" + ") + &c_value.to_string() + " * "
        };

        if c_str == " + " && is_first {
            c_str = "".into();
        }

        is_first = false;

        let var_str = match var.get_unchecked() {
            Index::Input(i) => format!("Input({})", i),
            Index::Aux(i) => format!("Witness({})", i),
        };

        string.push_str(&c_str);
        string.push_str(&var_str);
    }

    string
}
