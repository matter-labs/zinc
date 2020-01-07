use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::Cell;
use crate::RuntimeError;

#[derive(Debug)]
pub struct EvaluationStack<P: Primitive> {
    stack: Vec<Vec<Cell<P>>>,
}

impl<P: Primitive> EvaluationStack<P> {
    pub fn new() -> Self {
        Self {
            stack: vec![vec![]],
        }
    }

    pub fn push(&mut self, value: Cell<P>) -> Result<(), RuntimeError> {
        self.stack
            .last_mut()
            .ok_or_else(|| {
                RuntimeError::InternalError("Evaluation stack root frame missing".into())
            })?
            .push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Cell<P>, RuntimeError> {
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

    pub fn merge<O>(&mut self, condition: P, ops: &mut O) -> Result<(), RuntimeError>
    where
        O: PrimitiveOperations<P>,
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
                }
                _ => return Err(RuntimeError::MergingNonValueTypes),
            }
        }

        Ok(())
    }

    pub fn revert(&mut self) -> Result<(), RuntimeError> {
        self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
        Ok(())
    }
}
