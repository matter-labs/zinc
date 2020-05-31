//!
//! The VM core counter namespace.
//!

use std::marker::PhantomData;

use franklin_crypto::bellman::ConstraintSystem;

use crate::Engine;

pub struct NamespaceCounter<E: Engine, CS: ConstraintSystem<E>> {
    pub cs: CS,
    pub counter: usize,

    _pd: PhantomData<E>,
}

impl<E: Engine, CS: ConstraintSystem<E>> NamespaceCounter<E, CS> {
    pub fn new(cs: CS) -> Self {
        Self {
            cs,
            counter: 0,
            _pd: PhantomData,
        }
    }

    pub fn next(&mut self) -> bellman::Namespace<E, CS::Root> {
        let namespace = self.counter.to_string();
        self.counter += 1;
        self.cs.namespace(|| namespace)
    }
}
