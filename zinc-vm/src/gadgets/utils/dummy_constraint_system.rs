use crate::ZincEngine;
use bellman::ConstraintSystem;
use franklin_crypto::bellman::{Index, LinearCombination, SynthesisError, Variable};

pub struct DummyConstraintSystem;

impl<E: ZincEngine> ConstraintSystem<E> for DummyConstraintSystem {
    type Root = Self;

    fn alloc<F, A, AR>(&mut self, _annotation: A, _f: F) -> Result<Variable, SynthesisError>
    where
        F: FnOnce() -> Result<E::Fr, SynthesisError>,
        A: FnOnce() -> AR,
        AR: Into<String>,
    {
        Ok(Variable::new_unchecked(Index::Input(0)))
    }

    fn alloc_input<F, A, AR>(&mut self, _annotation: A, _f: F) -> Result<Variable, SynthesisError>
    where
        F: FnOnce() -> Result<E::Fr, SynthesisError>,
        A: FnOnce() -> AR,
        AR: Into<String>,
    {
        Ok(Variable::new_unchecked(Index::Input(0)))
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
