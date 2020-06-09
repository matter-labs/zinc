//!
//! The virtual machine counter namespace.
//!

use std::marker::PhantomData;

use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::bellman::Namespace;

use crate::IEngine;

pub struct NamespaceCounter<E: IEngine, CS: ConstraintSystem<E>> {
    pub cs: CS,
    pub counter: usize,

    _pd: PhantomData<E>,
}

impl<E: IEngine, CS: ConstraintSystem<E>> NamespaceCounter<E, CS> {
    pub fn new(cs: CS) -> Self {
        Self {
            cs,
            counter: 0,
            _pd: PhantomData,
        }
    }

    pub fn next(&mut self) -> Namespace<E, CS::Root> {
        let namespace = self.counter.to_string();
        self.counter += 1;
        self.cs.namespace(|| namespace)
    }
}
