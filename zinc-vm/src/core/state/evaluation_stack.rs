use crate::core::Cell;
use crate::errors::MalformedBytecode;
use crate::gadgets::{Gadgets, Scalar};
use crate::Engine;
use crate::RuntimeError;
use franklin_crypto::bellman::ConstraintSystem;
use std::fmt;

#[derive(Debug)]
pub struct EvaluationStack<E: Engine> {
    stack: Vec<Vec<Cell<E>>>,
}

impl<E: Engine> EvaluationStack<E> {
    pub fn new() -> Self {
        Self {
            stack: vec![vec![]],
        }
    }

    pub fn push(&mut self, value: Cell<E>) -> Result<(), RuntimeError> {
        self.stack
            .last_mut()
            .ok_or_else(|| {
                RuntimeError::InternalError("Evaluation stack root frame missing".into())
            })?
            .push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Cell<E>, RuntimeError> {
        self.stack
            .last_mut()
            .ok_or_else(|| {
                RuntimeError::InternalError("Evaluation stack root frame missing".into())
            })?
            .pop()
            .ok_or_else(|| MalformedBytecode::StackUnderflow.into())
    }

    pub fn fork(&mut self) {
        self.stack.push(vec![]);
    }

    pub fn merge<CS>(
        &mut self,
        condition: Scalar<E>,
        ops: &mut Gadgets<E, CS>,
    ) -> Result<(), RuntimeError>
    where
        CS: ConstraintSystem<E>,
    {
        let else_case = self.stack.pop().ok_or_else(|| {
            RuntimeError::InternalError("Evaluation stack root frame missing".into())
        })?;
        let then_case = self.stack.pop().ok_or_else(|| {
            RuntimeError::InternalError("Evaluation stack root frame missing".into())
        })?;

        if then_case.len() != else_case.len() {
            return Err(MalformedBytecode::BranchStacksDoNotMatch.into());
        }

        for (t, e) in then_case.into_iter().zip(else_case.into_iter()) {
            match (t, e) {
                (Cell::Value(tv), Cell::Value(ev)) => {
                    let merged = ops.conditional_select(condition.clone(), tv, ev)?;
                    self.push(Cell::Value(merged))?;
                } //                _ => return Err(RuntimeError::MergingNonValueTypes),
            }
        }

        Ok(())
    }

    pub fn revert(&mut self) -> Result<(), RuntimeError> {
        self.stack.pop().ok_or(MalformedBytecode::StackUnderflow)?;
        Ok(())
    }
}

impl<E: Engine> fmt::Display for EvaluationStack<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Evaluation Stack:")?;

        for frame in self.stack.iter().rev() {
            for cell in frame.iter().rev() {
                let Cell::Value(value) = cell;
                writeln!(f, "\t{}", value)?;
            }
        }

        Ok(())
    }
}
