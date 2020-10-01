//!
//! The constant constraint system.
//!

use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::bellman::LinearCombination;
use franklin_crypto::bellman::SynthesisError;
use franklin_crypto::bellman::Variable;

use crate::IEngine;

#[derive(Default)]
pub struct Constant {}

impl Constant {
    pub fn is_satisfied(&self) -> bool {
        true
    }

    pub fn num_constraints(&self) -> usize {
        0
    }
}

impl<E: IEngine> ConstraintSystem<E> for Constant {
    type Root = Self;

    fn alloc<F, A, AR>(&mut self, _annotation: A, f: F) -> Result<Variable, SynthesisError>
    where
        F: FnOnce() -> Result<E::Fr, SynthesisError>,
        A: FnOnce() -> AR,
        AR: Into<String>,
    {
        f()?;
        Ok(<Self as ConstraintSystem<E>>::one())
    }

    fn alloc_input<F, A, AR>(&mut self, _annotation: A, f: F) -> Result<Variable, SynthesisError>
    where
        F: FnOnce() -> Result<E::Fr, SynthesisError>,
        A: FnOnce() -> AR,
        AR: Into<String>,
    {
        f()?;
        Ok(<Self as ConstraintSystem<E>>::one())
    }

    fn enforce<A, AR, LA, LB, LC>(&mut self, _annotation: A, _a: LA, _b: LB, _c: LC)
    where
        A: FnOnce() -> AR,
        AR: Into<String>,
        LA: FnOnce(LinearCombination<E>) -> LinearCombination<E>,
        LB: FnOnce(LinearCombination<E>) -> LinearCombination<E>,
        LC: FnOnce(LinearCombination<E>) -> LinearCombination<E>,
    {
    }

    fn push_namespace<NR, N>(&mut self, _name_fn: N)
    where
        NR: Into<String>,
        N: FnOnce() -> NR,
    {
    }

    fn pop_namespace(&mut self) {}

    fn get_root(&mut self) -> &mut Self::Root {
        self
    }
}
