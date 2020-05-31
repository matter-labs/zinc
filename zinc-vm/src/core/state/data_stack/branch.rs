//!
//! The VM state data stack branch.
//!

use std::collections::BTreeMap;

use crate::core::state::cell::Cell;
use crate::Engine;

#[derive(Debug)]
pub enum DataStackBranch<E: Engine> {
    IfThen(DataStackDelta<E>),
    IfThenElse(DataStackDelta<E>, DataStackDelta<E>),
}

pub type DataStackDelta<E> = BTreeMap<usize, CellDelta<E>>;

#[derive(Debug)]
pub struct CellDelta<E: Engine> {
    pub old: Option<Cell<E>>,
    pub new: Cell<E>,
}

impl<E: Engine> DataStackBranch<E> {
    pub fn new() -> Self {
        DataStackBranch::IfThen(DataStackDelta::new())
    }

    pub fn active_delta(&mut self) -> &mut DataStackDelta<E> {
        match self {
            DataStackBranch::IfThen(t) => t,
            DataStackBranch::IfThenElse(_, e) => e,
        }
    }

    pub fn switch(self) -> Option<Self> {
        match self {
            DataStackBranch::IfThen(t) => {
                Some(DataStackBranch::IfThenElse(t, DataStackDelta::new()))
            }
            DataStackBranch::IfThenElse(_, _) => None,
        }
    }
}
