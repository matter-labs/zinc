use crate::primitive::{Primitive, PrimitiveOperations};
use crate::RuntimeError;

#[derive(Debug, Clone)]
pub enum StorageCell<P: Primitive> {
    None,
    UnchangedValue(P),
    ChangedValue(P),
    UnchangedMerkleTree(P::MerkleTree),
    ChangedMerkleTree(P::MerkleTree),
}

/// EvaluationStack is a data structure that represents the state of function execution.
#[derive(Debug)]
pub struct EvaluationStack<P: Primitive> {
    stack: Vec<P>,
}


impl<P: Primitive> EvaluationStack<P> {
    pub fn new() -> Self {
        Self {
            stack: vec![],
        }
    }

    pub fn push(&mut self, value: P) -> Result<(), RuntimeError> {
        self.stack.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<P, RuntimeError> {
        self.stack.pop().ok_or(RuntimeError::StackUnderflow)
    }

    pub fn fork(&self) -> Self {
        Self {
            stack: vec![],
        }
    }

    pub fn merge<O>(&mut self, condition: P, left: Self, right: Self, operator: &mut O)
                    -> Result<(), RuntimeError>
        where
            O: PrimitiveOperations<P>
    {
        let ls = left.stack;
        let rs = right.stack;

        if ls.len() != rs.len() {
            return Err(RuntimeError::BranchStacksDoNotMatch);
        }

        for (l, r) in ls.into_iter().zip(rs.into_iter()) {
            let merged = operator.conditional_select(condition.clone(), l, r)?;
            self.stack.push(merged);
        }

        Ok(())
    }
}
