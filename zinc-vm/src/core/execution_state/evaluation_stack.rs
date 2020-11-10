//!
//! The VM state evaluation stack.
//!

use std::fmt;

use franklin_crypto::bellman::ConstraintSystem;

use crate::core::execution_state::cell::Cell;
use crate::error::Error;
use crate::error::MalformedBytecode;
use crate::gadgets;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

#[derive(Debug)]
pub struct EvaluationStack<E: IEngine> {
    stack: Vec<Vec<Cell<E>>>,
}

impl<E: IEngine> Default for EvaluationStack<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: IEngine> EvaluationStack<E> {
    const STACK_INITIAL_CAPACITY: usize = 16384;

    pub fn new() -> Self {
        Self {
            stack: vec![Vec::with_capacity(Self::STACK_INITIAL_CAPACITY)],
        }
    }

    pub fn push(&mut self, value: Cell<E>) -> Result<(), Error> {
        self.stack
            .last_mut()
            .ok_or_else(|| Error::InternalError("Evaluation stack root frame missing".into()))?
            .push(value);

        Ok(())
    }

    pub fn pop(&mut self) -> Result<Cell<E>, Error> {
        self.stack
            .last_mut()
            .ok_or_else(|| Error::InternalError("Evaluation stack root frame missing".into()))?
            .pop()
            .ok_or_else(|| MalformedBytecode::StackUnderflow.into())
    }

    pub fn fork(&mut self) {
        self.stack
            .push(Vec::with_capacity(Self::STACK_INITIAL_CAPACITY));
    }

    pub fn merge<CS>(&mut self, mut cs: CS, condition: &Scalar<E>) -> Result<(), Error>
    where
        CS: ConstraintSystem<E>,
    {
        let else_case = self
            .stack
            .pop()
            .ok_or_else(|| Error::InternalError("Evaluation stack root frame missing".into()))?;
        let then_case = self
            .stack
            .pop()
            .ok_or_else(|| Error::InternalError("Evaluation stack root frame missing".into()))?;

        if then_case.len() != else_case.len() {
            return Err(MalformedBytecode::BranchStacksDoNotMatch.into());
        }

        for (index, (main_value, else_value)) in
            then_case.into_iter().zip(else_case.into_iter()).enumerate()
        {
            match (main_value, else_value) {
                (Cell::Value(main_value), Cell::Value(else_value)) => {
                    let merged = gadgets::select::conditional(
                        cs.namespace(|| format!("merge {}", index)),
                        condition,
                        &main_value,
                        &else_value,
                    )?;

                    self.push(Cell::Value(merged))?;
                }
            }
        }

        Ok(())
    }

    pub fn revert(&mut self) -> Result<(), Error> {
        self.stack.pop().ok_or(MalformedBytecode::StackUnderflow)?;
        Ok(())
    }
}

impl<E: IEngine> fmt::Display for EvaluationStack<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Evaluation stack:")?;

        for frame in self.stack.iter().rev() {
            for cell in frame.iter().rev() {
                let Cell::Value(value) = cell;
                writeln!(f, "\t{}", value)?;
            }
        }

        Ok(())
    }
}
