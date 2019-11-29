use crate::element::Element;
use crate::RuntimeError;

/// StackFrame is a data structure that represents the state of function execution.
#[derive(Debug, Clone)]
pub struct Memory<E: Element> {
//    arguments: Vec<E>,
    stack: Vec<E>,
    storage: Vec<Option<E>>,
}


impl<E: Element> Memory<E> {
    /// Initialize new stack frame with given arguments.
    pub fn new(arguments: &[E]) -> Self {
        // TODO: Remove arguments from evaluation stack.
        Self {
            stack: Vec::from(arguments),
            storage: {
                arguments
                    .iter()
                    .map(|arg| Some((*arg).clone()))
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
    pub fn push(&mut self, value: E) -> Result<(), RuntimeError> {
        self.stack.push(value);
        Ok(())
    }

    /// Pop value from evaluation stack.
    pub fn pop(&mut self) -> Result<E, RuntimeError>  {
        self.stack.pop().ok_or(RuntimeError::StackUnderflow)
    }

    /// Store value in the storage.
    pub fn store(&mut self, index: usize, value: E) -> Result<(), RuntimeError> {
        if self.storage.len() <= index {
            self.storage.append(vec![None; index*2 + 2].as_mut());
        }

        self.storage[index] = Some(value);

        Ok(())
    }

    /// Load value from the storage.
    pub fn load(&mut self, index: usize) -> Result<E, RuntimeError>  {
        match self.storage.get(index) {
            None => Err(RuntimeError::UninitializedStorageAccess),
            Some(option_value) => match option_value {
                None => Err(RuntimeError::UninitializedStorageAccess),
                Some(value) => Ok((*value).clone()),
            },
        }
    }

    /// Temporary fix for compatibility
    #[deprecated(note = "")]
    pub fn copy(&mut self, index: usize) -> Result<E, RuntimeError> {
        self.stack.get(index)
            .ok_or(RuntimeError::StackIndexOutOfRange)
            .map(|value| (*value).clone())
    }
}
