//!
//! The VM state data stack.
//!

mod tests;

pub mod branch;

use std::fmt;

use franklin_crypto::bellman::ConstraintSystem;

use crate::core::state::cell::Cell;
use crate::error::MalformedBytecode;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::gadgets::misc::Gadgets;
use crate::gadgets::scalar::Scalar;
use crate::Engine;

use self::branch::CellDelta;
use self::branch::DataStackBranch;
use self::branch::DataStackDelta;

#[derive(Debug)]
pub struct DataStack<E: Engine> {
    memory: Vec<Option<Cell<E>>>,
    branches: Vec<DataStackBranch<E>>,
}

impl<E: Engine> DataStack<E> {
    pub fn new() -> Self {
        Self {
            memory: Vec::new(),
            branches: Vec::new(),
        }
    }

    pub fn get(&mut self, address: usize) -> Result<Cell<E>, RuntimeError> {
        self.memory
            .get(address)
            .ok_or(MalformedBytecode::UninitializedStorageAccess)
            .map_err(RuntimeError::MalformedBytecode)?
            .to_owned()
            .ok_or(MalformedBytecode::UninitializedStorageAccess)
            .map_err(RuntimeError::MalformedBytecode)
    }

    pub fn set(&mut self, address: usize, value: Cell<E>) -> Result<(), RuntimeError> {
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

    /// Create a new memory state branch
    pub fn fork(&mut self) {
        self.branches.push(DataStackBranch::new());
    }

    /// Create an alternative branch (same parent as current one).
    pub fn switch_branch(&mut self) -> Result<(), RuntimeError> {
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
        condition: Scalar<E>,
        ops: &mut Gadgets<E, CS>,
    ) -> Result<(), RuntimeError> {
        let mut branch = self
            .branches
            .pop()
            .ok_or(MalformedBytecode::UnexpectedEndIf)?;
        self.revert(branch.active_delta());

        match branch {
            DataStackBranch::IfThen(delta) => self.merge_single(condition, &delta, ops)?,
            DataStackBranch::IfThenElse(t, f) => self.merge_pair(condition, &t, &f, ops)?,
        }

        Ok(())
    }

    fn revert(&mut self, delta: &DataStackDelta<E>) {
        for (address, d) in delta.iter() {
            self.memory[*address] = d.old.clone();
        }
    }

    /// Conditionally apply delta
    fn merge_single<CS: ConstraintSystem<E>>(
        &mut self,
        condition: Scalar<E>,
        delta: &DataStackDelta<E>,
        ops: &mut Gadgets<E, CS>,
    ) -> Result<(), RuntimeError> {
        for (&addr, diff) in delta.iter() {
            match (&self.memory[addr], &diff.new) {
                (None, _) => {}
                (Some(Cell::Value(old)), Cell::Value(new)) => {
                    let cs = ops
                        .constraint_system()
                        .namespace(|| format!("merge address {}", addr));
                    let value =
                        gadgets::conditional_select::conditional_select(cs, &condition, new, old)?;
                    self.set(addr, Cell::Value(value))?;
                }
            }
        }

        Ok(())
    }

    /// Conditionally apply one of two deltas.
    fn merge_pair<CS>(
        &mut self,
        condition: Scalar<E>,
        delta_then: &DataStackDelta<E>,
        delta_else: &DataStackDelta<E>,
        ops: &mut Gadgets<E, CS>,
    ) -> Result<(), RuntimeError>
    where
        CS: ConstraintSystem<E>,
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
                    let cs = ops
                        .constraint_system()
                        .namespace(|| format!("merge address {}", addr));
                    let value =
                        gadgets::conditional_select::conditional_select(cs, &condition, new, old)?;
                    self.set(*addr, Cell::Value(value))?;
                }
            }
        }

        Ok(())
    }
}

impl<E: Engine> fmt::Display for DataStack<E> {
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
