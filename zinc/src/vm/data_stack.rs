use std::collections::HashMap;
use crate::RuntimeError;
use crate::primitive::Primitive;

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

    fn current_delta(&mut self) -> &mut DataStackDelta<P> {
        match self {
            DataStackBranch::IfThen(t) => t,
            DataStackBranch::IfThenElse(_, e) => e,
        }
    }

    fn switch(self) -> Option<Self> {
        match self {
            DataStackBranch::IfThen(t) => {
                Some(DataStackBranch::IfThenElse(t, HashMap::new()))
            },
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
            let mut extra = vec![None; address + 1];
            self.memory.append(&mut extra);
        }

        if let Some(branch) = self.branches.last_mut() {
            let delta = branch.current_delta();
            let old = match delta.get(&address) {
                Some(old_cd) => old_cd.old.clone(),
                None => {
                    self.memory[address].clone()
                },
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
        self.revert(&branch.current_delta());
        let new_branch = branch.switch().ok_or(RuntimeError::UnexpectedElse)?;
        self.branches.push(new_branch);
        Ok(())
    }

    /// Merge top-level branch or branches into parent branch.
    pub fn merge(&mut self, condition: P) -> Result<(), RuntimeError> {
        unimplemented!()
    }

    fn revert(&mut self, delta: &DataStackDelta<P>) {
        for (address, d) in delta.iter() {
            self.memory.insert(*address, d.old.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive::{SimplePrimitive, SimplePrimitiveOperations, PrimitiveOperations};
    use num_bigint::ToBigInt;

    #[test]
    fn data_stack_get_set() {
        let mut ds = DataStack::<SimplePrimitive>::new();
        let mut op = SimplePrimitiveOperations::new();
        let value = op.constant_bigint(&42.into()).unwrap();
        ds.set(4, Cell::Value(value));

        if let Ok(Cell::Value(v)) = ds.get(4) {
            assert_eq!(v.to_bigint().unwrap(), 42.into());
        } else {
            panic!("");
        }
    }
}
