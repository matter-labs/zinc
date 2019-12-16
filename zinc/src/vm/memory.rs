use crate::primitive::{Primitive, PrimitiveOperations};
use crate::RuntimeError;
use std::cmp;

#[derive(Debug, Clone)]
pub enum StorageCell<P: Primitive> {
    None,
    UnchangedValue(P),
    ChangedValue(P),
    UnchangedMerkleTree(P::MerkleTree),
    ChangedMerkleTree(P::MerkleTree),
}

/// StackFrame is a data structure that represents the state of function execution.
#[derive(Debug)]
pub struct Memory<P: Primitive> {
    //    arguments: Vec<E>,
    stack: Vec<P>,
    storage: Vec<StorageCell<P>>,
}


impl<P: Primitive> Memory<P> {
    /// Initialize new stack frame with given arguments.
    pub fn new(arguments: &[P]) -> Self {
        Self {
            stack: vec![],
            storage: {
                arguments
                    .iter()
                    .map(|arg| StorageCell::UnchangedValue((*arg).clone()))
                    .collect()
            },
        }
    }

//    /// Get argument by it's index.
//    pub fn argument(&mut self, index: usize) -> Result<E, RuntimeError>  {
//        self.arguments.get(index)
//            .ok_or(RuntimeError::MissingArgument)
//            .map(|value| (*value).clone())
//    }

    /// Push value onto evaluation stack.
    pub fn push(&mut self, value: P) -> Result<(), RuntimeError> {
        self.stack.push(value);
        Ok(())
    }

    /// Pop value from evaluation stack.
    pub fn pop(&mut self) -> Result<P, RuntimeError> {
        self.stack.pop().ok_or(RuntimeError::StackUnderflow)
    }

    /// Store value in the storage.
    pub fn store(&mut self, index: usize, value: P) -> Result<(), RuntimeError> {
        if self.storage.len() <= index {
            self.storage.append(vec![StorageCell::None; index * 2 + 2].as_mut());
        }

        self.storage[index] = StorageCell::ChangedValue(value);

        Ok(())
    }

    /// Load value from the storage.
    pub fn load(&mut self, index: usize) -> Result<P, RuntimeError> {
        match self.storage.get(index) {
            None => Err(RuntimeError::UninitializedStorageAccess),
            Some(option_value) => match option_value {
                StorageCell::None => Err(RuntimeError::UninitializedStorageAccess),

                StorageCell::UnchangedValue(value) |
                StorageCell::ChangedValue(value) => Ok((*value).clone()),

                StorageCell::ChangedMerkleTree(_) |
                StorageCell::UnchangedMerkleTree(_) => Err(RuntimeError::UsingMerkleTreeAsValue),
            },
        }
    }

    /// Temporary fix for compatibility
    #[deprecated(note = "")]
    pub fn copy(&mut self, index: usize) -> Result<P, RuntimeError> {
        self.stack.get(index)
            .ok_or(RuntimeError::StackIndexOutOfRange)
            .map(|value| (*value).clone())
    }

    pub fn fork(&self) -> Self {
        Self {
            stack: vec![],
            storage: self.storage.clone(),
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

        let len = cmp::min(left.storage.len(), right.storage.len());
        for i in 0..len {
            match (&left.storage[i], &right.storage[i]) {
                (StorageCell::None, _) |
                (_, StorageCell::None) |
                (StorageCell::UnchangedValue(_), StorageCell::UnchangedValue(_)) => {
                    // Do nothing...
                },
                (StorageCell::ChangedValue(left), StorageCell::ChangedValue(right)) |
                (StorageCell::ChangedValue(left), StorageCell::UnchangedValue(right)) |
                (StorageCell::UnchangedValue(left), StorageCell::ChangedValue(right)) => {
                    let merged = operator.conditional_select(
                        condition.clone(),
                        (*left).clone(),
                        (*right).clone(),
                    )?;
                    self.storage[i] = StorageCell::ChangedValue(merged);
                },

                (StorageCell::ChangedMerkleTree(_), StorageCell::ChangedMerkleTree(_)) |
                (StorageCell::ChangedMerkleTree(_), StorageCell::UnchangedMerkleTree(_)) |
                (StorageCell::UnchangedMerkleTree(_), StorageCell::ChangedMerkleTree(_)) |
                (StorageCell::UnchangedMerkleTree(_), StorageCell::UnchangedMerkleTree(_)) => {
                    unimplemented!();
                }

                _ => {
                    unimplemented!();
                }
            }
        }

        Ok(())
    }
}
