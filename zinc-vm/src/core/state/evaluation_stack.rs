use crate::core::Cell;
use crate::gadgets::{Gadgets, Primitive};
use crate::Engine;
use crate::RuntimeError;
use franklin_crypto::bellman::ConstraintSystem;

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
            .ok_or(RuntimeError::StackUnderflow)
    }

    pub fn fork(&mut self) {
        self.stack.push(vec![]);
    }

    pub fn merge<CS>(
        &mut self,
        condition: Primitive<E>,
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
            return Err(RuntimeError::BranchStacksDoNotMatch);
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
        self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
        Ok(())
    }
}
