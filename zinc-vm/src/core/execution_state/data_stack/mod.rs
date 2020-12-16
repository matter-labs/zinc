//!
//! The VM state data stack.
//!

#[cfg(test)]
mod tests;

pub mod branch;

use std::fmt;

use franklin_crypto::bellman::ConstraintSystem;

use crate::core::execution_state::cell::Cell;
use crate::error::Error;
use crate::error::MalformedBytecode;
use crate::gadgets;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

use self::branch::CellDelta;
use self::branch::DataStackBranch;
use self::branch::DataStackDelta;

#[derive(Debug)]
pub struct DataStack<E: IEngine> {
    pub memory: Vec<Option<Cell<E>>>,
    pub branches: Vec<DataStackBranch<E>>,
}

impl<E: IEngine> DataStack<E> {
    const MEMORY_INITIAL_CAPACITY: usize = 16384;
    const BRANCHES_INITIAL_CAPACITY: usize = 64;

    pub fn new() -> Self {
        Self {
            memory: Vec::with_capacity(Self::MEMORY_INITIAL_CAPACITY),
            branches: Vec::with_capacity(Self::BRANCHES_INITIAL_CAPACITY),
        }
    }

    pub fn get(&mut self, address: usize) -> Result<Cell<E>, Error> {
        self.memory
            .get(address)
            .ok_or(MalformedBytecode::UninitializedStorageAccess)
            .map_err(Error::MalformedBytecode)?
            .to_owned()
            .ok_or(MalformedBytecode::UninitializedStorageAccess)
            .map_err(Error::MalformedBytecode)
    }

    pub fn set(&mut self, address: usize, value: Cell<E>) -> Result<(), Error> {
        if self.memory.len() <= address {
            let mut extra = vec![None; address + 1 - self.memory.len()];
            self.memory.append(&mut extra);
        }

        if let Some(branch) = self.branches.last_mut() {
            let delta = branch.active_delta();
            let old = match delta.get(&address) {
                Some(old_cd) => old_cd.old.clone(),
                None => self.memory[address].clone(),
            };

            delta.insert(
                address,
                CellDelta {
                    old,
                    new: value.clone(),
                },
            );
        }

        self.memory[address] = Some(value);

        Ok(())
    }

    ///
    /// Create a new memory state branch.
    ///
    pub fn fork(&mut self) {
        self.branches.push(DataStackBranch::new());
    }

    ///
    /// Create an alternative branch (same parent as current one).
    ///
    pub fn switch_branch(&mut self) -> Result<(), Error> {
        let mut branch = self
            .branches
            .pop()
            .ok_or(MalformedBytecode::UnexpectedElse)?;
        self.revert(&branch.active_delta());
        let new_branch = branch.switch().ok_or(MalformedBytecode::UnexpectedElse)?;
        self.branches.push(new_branch);
        Ok(())
    }

    /// Merge top-level branch or branches into parent branch.
    pub fn merge<CS: ConstraintSystem<E>>(
        &mut self,
        cs: CS,
        condition: Scalar<E>,
    ) -> Result<(), Error> {
        let mut branch = self
            .branches
            .pop()
            .ok_or(MalformedBytecode::UnexpectedEndIf)?;
        self.revert(branch.active_delta());

        match branch {
            DataStackBranch::IfThen(delta) => self.merge_single(cs, condition, &delta)?,
            DataStackBranch::IfThenElse(t, f) => self.merge_pair(cs, condition, &t, &f)?,
        }

        Ok(())
    }

    fn revert(&mut self, delta: &DataStackDelta<E>) {
        for (address, delta) in delta.iter() {
            if let Some(cell) = self.memory.get_mut(*address) {
                *cell = delta.old.clone();
            }
        }
    }

    /// Conditionally apply delta
    fn merge_single<CS: ConstraintSystem<E>>(
        &mut self,
        mut cs: CS,
        condition: Scalar<E>,
        delta: &DataStackDelta<E>,
    ) -> Result<(), Error> {
        for (&addr, diff) in delta.iter() {
            if let (Some(Some(Cell::Value(old))), Cell::Value(new)) =
                (&self.memory.get(addr), &diff.new)
            {
                let cs = cs.namespace(|| format!("merge address {}", addr));
                let value = gadgets::select::conditional(cs, &condition, new, old)?;
                self.set(addr, Cell::Value(value))?;
            }
        }

        Ok(())
    }

    /// Conditionally apply one of two deltas.
    fn merge_pair<CS>(
        &mut self,
        mut cs: CS,
        condition: Scalar<E>,
        delta_then: &DataStackDelta<E>,
        delta_else: &DataStackDelta<E>,
    ) -> Result<(), Error>
    where
        CS: ConstraintSystem<E>,
    {
        for (addr, diff) in delta_then.iter() {
            let alt = if let Some(diff) = delta_else.get(addr) {
                Some(Some(diff.new.to_owned()))
            } else {
                self.memory.get(*addr).cloned()
            };

            if let (Some(Some(Cell::Value(old))), Cell::Value(new)) = (&alt, &diff.new) {
                let cs = cs.namespace(|| format!("merge address {}", addr));
                let value = gadgets::select::conditional(cs, &condition, new, old)?;
                self.set(*addr, Cell::Value(value))?;
            }
        }

        Ok(())
    }

    ///
    /// Erase the memory starting from `start_address`.
    ///
    pub fn drop_from(&mut self, start_address: usize) {
        self.memory.truncate(start_address);
    }
}

impl<E: IEngine> fmt::Display for DataStack<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Data stack:")?;

        for (address, opt_cell) in self.memory.iter().enumerate() {
            match opt_cell {
                None => writeln!(f, "\t{:4}: <empty>", address)?,
                Some(Cell::Value(value)) => writeln!(f, "\t{:4}: {}", address, value)?,
            }
        }

        Ok(())
    }
}
