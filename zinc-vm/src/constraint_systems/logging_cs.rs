use crate::gadgets::utils;
use bellman::ConstraintSystem;
use failure::_core::marker::PhantomData;
use franklin_crypto::bellman::{Index, LinearCombination, SynthesisError, Variable};
use num_bigint::BigInt;
use num_traits::Signed;
use pairing::Engine;

pub struct LoggingConstraintSystem<E, CS>(CS, PhantomData<E>)
where
    E: Engine,
    CS: ConstraintSystem<E>;

impl<E, CS> LoggingConstraintSystem<E, CS>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    pub fn new(cs: CS) -> Self {
        Self(cs, PhantomData)
    }

    pub fn inner(&self) -> &CS {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut CS {
        &mut self.0
    }

    pub fn into_inner(self) -> CS {
        self.0
    }
}

impl<E, CS> ConstraintSystem<E> for LoggingConstraintSystem<E, CS>
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
        let ar = annotation().into();
        let mut value = None;
        let variable = self.0.alloc(
            || &ar,
            || {
                let tmp = f()?;
                value = Some(tmp);
                Ok(tmp)
            },
        )?;
        log::trace!(
            "r1cs: witness: name = \"{}\", value = {:?}, index = {:?}",
            ar,
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
        let ar = annotation().into();
        let mut value = None;
        let variable = self.0.alloc_input(
            || &ar,
            || {
                let tmp = f()?;
                value = Some(tmp);
                Ok(tmp)
            },
        )?;
        log::trace!(
            "r1cs: input: name = \"{}\", value = {:?}, index = {:?}",
            ar,
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
        let ar = annotation().into();
        let lc_a = a(LinearCombination::zero());
        let lc_b = b(LinearCombination::zero());
        let lc_c = c(LinearCombination::zero());
        log::trace!(
            "r1cs: constraint: ({}) * ({}) = ({}), name = {}",
            lc_to_string(&lc_a),
            lc_to_string(&lc_b),
            lc_to_string(&lc_c),
            ar,
        );

        self.0.enforce(|| ar, |_| lc_a, |_| lc_b, |_| lc_c)
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
        let c_value = utils::fr_to_bigint_signed(c);

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
