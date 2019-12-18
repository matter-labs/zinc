use std::collections::HashMap;

use crate::primitive::{Primitive, PrimitiveOperations};
use crate::RuntimeError;

#[derive(Debug, Clone, PartialEq)]
pub enum Cell<P: Primitive> {
    Value(P),
    Address(usize),
}

#[derive(Debug)]
struct CellDelta<P: Primitive> {
    old: Option<Cell<P>>,
    new: Cell<P>,
}

type DataStackDelta<P> = HashMap<usize, CellDelta<P>>;

#[derive(Debug)]
enum DataStackBranch<P: Primitive> {
    IfThen(DataStackDelta<P>),
    IfThenElse(DataStackDelta<P>, DataStackDelta<P>),
}

impl<P: Primitive> DataStackBranch<P> {
    fn new() -> Self {
        DataStackBranch::IfThen(HashMap::new())
    }

    fn active_delta(&mut self) -> &mut DataStackDelta<P> {
        match self {
            DataStackBranch::IfThen(t) => t,
            DataStackBranch::IfThenElse(_, e) => e,
        }
    }

    fn switch(self) -> Option<Self> {
        match self {
            DataStackBranch::IfThen(t) => {
                Some(DataStackBranch::IfThenElse(t, HashMap::new()))
            }
            DataStackBranch::IfThenElse(_, _) => None,
        }
    }
}

#[derive(Debug)]
pub struct DataStack<P: Primitive> {
    memory: Vec<Option<Cell<P>>>,
    branches: Vec<DataStackBranch<P>>,
}

impl<P: Primitive> DataStack<P> {
    pub fn new() -> Self {
        Self {
            memory: Vec::new(),
            branches: Vec::new(),
        }
    }

    pub fn get(&mut self, address: usize) -> Result<Cell<P>, RuntimeError> {
        if let Some(cell) = self.memory.get(address) {
            cell
                .clone()
                .ok_or(RuntimeError::UninitializedStorageAccess)
        } else {
            Err(RuntimeError::UninitializedStorageAccess)
        }
    }

    pub fn set(&mut self, address: usize, value: Cell<P>) -> Result<(), RuntimeError> {
        if self.memory.len() <= address {
            let mut extra = vec![None; address + 1 - self.memory.len()];
            self.memory.append(&mut extra);
        }

        if let Some(branch) = self.branches.last_mut() {
            let delta = branch.active_delta();
            let old = match delta.get(&address) {
                Some(old_cd) => old_cd.old.clone(),
                None => {
                    self.memory[address].clone()
                }
            };
            delta.insert(address, CellDelta { old, new: value.clone() });
        }

        self.memory[address] = Some(value);

        Ok(())
    }

    /// Create a new memory state branch
    pub fn fork(&mut self) {
        self.branches.push(DataStackBranch::new());
    }

    /// Create an alternative branch (same parent as current one).
    pub fn switch_branch(&mut self) -> Result<(), RuntimeError> {
        let mut branch = self.branches.pop().ok_or(RuntimeError::UnexpectedElse)?;
        self.revert(&branch.active_delta());
        let new_branch = branch.switch().ok_or(RuntimeError::UnexpectedElse)?;
        self.branches.push(new_branch);
        Ok(())
    }

    /// Merge top-level branch or branches into parent branch.
    pub fn merge<O: PrimitiveOperations<P>>(&mut self, condition: P, ops: &mut O) -> Result<(), RuntimeError> {
        let mut branch = self.branches.pop().ok_or(RuntimeError::UnexpectedEndIf)?;
        self.revert(branch.active_delta());

        match branch {
            DataStackBranch::IfThen(delta) => {
                self.merge_single(condition, &delta, ops)?
            },
            DataStackBranch::IfThenElse(t, f) => {
                self.merge_pair(condition, &t, &f, ops)?
            }
        }

        Ok(())
    }

    fn revert(&mut self, delta: &DataStackDelta<P>) {
        for (address, d) in delta.iter() {
            self.memory.insert(*address, d.old.clone());
        }
    }

    /// Conditionally apply delta
    fn merge_single<O: PrimitiveOperations<P>>(&mut self, condition: P, delta: &DataStackDelta<P>, ops: &mut O) -> Result<(), RuntimeError> {
        for (&addr, diff) in delta.iter() {
            match (&self.memory[addr], &diff.new) {
                (None, _) => {}
                (Some(Cell::Value(old)), Cell::Value(new)) => {
                    let value = ops.conditional_select(condition.clone(), new.clone(), old.clone())?;
                    self.set(addr, Cell::Value(value));
                }
                (Some(old), new) => {
                    log::warn!("Merging {:?} into {:?}, ignoring...", new, old);
                }
            }
        }

        Ok(())
    }

    /// Conditionally apply one of two deltas.
    fn merge_pair<O>(&mut self, condition: P, delta_then: &DataStackDelta<P>, delta_else: &DataStackDelta<P>, ops: &mut O)
        -> Result<(), RuntimeError>
    where
        O: PrimitiveOperations<P>
    {
        for (addr, diff) in delta_then.iter() {
            let alt = if let Some(diff) = delta_else.get(addr) {
                Some(diff.new.clone())
            } else {
                self.memory[*addr].clone()
            };

            match (&alt, &diff.new) {
                (None, _) => {}
                (Some(Cell::Value(old)), Cell::Value(new)) => {
                    let value = ops.conditional_select(condition.clone(), new.clone(), old.clone())?;
                    self.set(*addr, Cell::Value(value));
                }
                (Some(old), new) => {
                    log::warn!("Merging {:?} into {:?}, ignoring...", new, old);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigInt;

    use crate::primitive::{PrimitiveOperations, SimplePrimitive, SimplePrimitiveOperations};

    use super::*;

    fn assert_cell_eq<P: Primitive>(cell: Cell<P>, value: BigInt) {
        if let Cell::Value(v) = cell {
            assert_eq!(v.to_bigint().unwrap(), value);
        } else {
            panic!("assert_cell_eq() failed");
        }
    }

    #[test]
    fn test_get_set() {
        let mut ds = DataStack::<SimplePrimitive>::new();
        let mut op = SimplePrimitiveOperations::new();
        let value = op.constant_bigint(&42.into()).unwrap();
        ds.set(4, Cell::Value(value));

        assert_cell_eq(ds.get(4).unwrap(), 42.into());
    }

    #[test]
    fn test_fork_merge_true() {
        let mut ds = DataStack::<SimplePrimitive>::new();
        let mut ops = SimplePrimitiveOperations::new();
        let value = ops.constant_bigint(&42.into()).unwrap();
        ds.set(4, Cell::Value(value));

        ds.fork();

        assert_cell_eq(ds.get(4).unwrap(), 42.into());

        let value2 = ops.constant_bigint(&13.into()).unwrap();
        ds.set(4, Cell::Value(value2));
        assert_cell_eq(ds.get(4).unwrap(), 13.into());

        let condition = ops.constant_bigint(&1.into()).unwrap();
        ds.merge(condition, &mut ops).unwrap();
        assert_cell_eq(ds.get(4).unwrap(), 13.into());
    }

    #[test]
    fn test_fork_merge_false() {
        let mut ds = DataStack::<SimplePrimitive>::new();
        let mut ops = SimplePrimitiveOperations::new();
        let value = ops.constant_bigint(&42.into()).unwrap();
        ds.set(4, Cell::Value(value));

        ds.fork();

        assert_cell_eq(ds.get(4).unwrap(), 42.into());

        let value2 = ops.constant_bigint(&13.into()).unwrap();
        ds.set(4, Cell::Value(value2));
        assert_cell_eq(ds.get(4).unwrap(), 13.into());

        let condition = ops.constant_bigint(&0.into()).unwrap();
        ds.merge(condition, &mut ops).unwrap();
        assert_cell_eq(ds.get(4).unwrap(), 42.into());
    }
}
