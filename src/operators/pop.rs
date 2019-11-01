use crate::vm::{Operator, RuntimeError};
use crate::stack::Stack;
use franklin_crypto::bellman::{ConstraintSystem, Variable};
use bellman::pairing::Engine;
use std::io;

/// Removes top element from the stack.
pub struct Pop;

impl<E, CS> Operator<E, CS> for Pop where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
            &self,
            _cs: &mut CS,
            stack: &mut Stack<Variable>,
            _bytecode: &mut dyn io::Read)
        -> Result<(), RuntimeError>
    {
        match stack.pop() {
            Some(_) => Ok(()),
            None => Err(RuntimeError::StackUnderflow),
        }
    }
}
