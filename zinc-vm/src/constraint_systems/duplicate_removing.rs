//!
//! The duplicate removing constraint system.
//!

use std::collections::BTreeMap;
use std::marker::PhantomData;

use bellman::ConstraintSystem;
use ff::Field;
use franklin_crypto::bellman::Index;
use franklin_crypto::bellman::LinearCombination;
use franklin_crypto::bellman::SynthesisError;
use franklin_crypto::bellman::Variable;
use pairing::Engine;

pub struct DuplicateRemovingCS<E, CS>(CS, PhantomData<E>)
where
    E: Engine,
    CS: ConstraintSystem<E>;

impl<E, CS> DuplicateRemovingCS<E, CS>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    pub fn new(cs: CS) -> Self {
        Self(cs, PhantomData)
    }
}

impl<E, CS> ConstraintSystem<E> for DuplicateRemovingCS<E, CS>
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
        self.0.alloc(annotation, f)
    }

    fn alloc_input<F, A, AR>(&mut self, annotation: A, f: F) -> Result<Variable, SynthesisError>
    where
        F: FnOnce() -> Result<E::Fr, SynthesisError>,
        A: FnOnce() -> AR,
        AR: Into<String>,
    {
        self.0.alloc_input(annotation, f)
    }

    fn enforce<A, AR, LA, LB, LC>(&mut self, annotation: A, a: LA, b: LB, c: LC)
    where
        A: FnOnce() -> AR,
        AR: Into<String>,
        LA: FnOnce(LinearCombination<E>) -> LinearCombination<E>,
        LB: FnOnce(LinearCombination<E>) -> LinearCombination<E>,
        LC: FnOnce(LinearCombination<E>) -> LinearCombination<E>,
    {
        self.0.enforce(
            annotation,
            |zero| remove_duplicates(a(zero)),
            |zero| remove_duplicates(b(zero)),
            |zero| remove_duplicates(c(zero)),
        )
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

fn remove_duplicates<E: Engine>(lc: LinearCombination<E>) -> LinearCombination<E> {
    let mut inputs_map = BTreeMap::<usize, E::Fr>::new();
    let mut aux_map = BTreeMap::<usize, E::Fr>::new();

    let zero = E::Fr::zero();
    for (var, c) in lc.as_ref() {
        match var.get_unchecked() {
            Index::Input(i) => {
                let mut tmp = *inputs_map.get(&i).unwrap_or(&zero);
                tmp.add_assign(c);
                inputs_map.insert(i, tmp);
            }
            Index::Aux(i) => {
                let mut tmp = *aux_map.get(&i).unwrap_or(&zero);
                tmp.add_assign(c);
                aux_map.insert(i, tmp);
            }
        }
    }

    let mut lc = LinearCombination::zero();

    for (i, c) in inputs_map.into_iter() {
        if c.is_zero() {
            continue;
        }
        let var = Variable::new_unchecked(Index::Input(i));
        lc = lc + (c, var);
    }

    for (i, c) in aux_map.into_iter() {
        if c.is_zero() {
            continue;
        }
        let var = Variable::new_unchecked(Index::Aux(i));
        lc = lc + (c, var);
    }

    lc
}
