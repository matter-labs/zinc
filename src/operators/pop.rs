use crate::{Operator, RuntimeError, Stack, Bytecode};
use franklin_crypto::bellman::ConstraintSystem;
use bellman::pairing::Engine;

/// Removes top element from the stack.
pub struct Pop;

impl<E, CS> Operator<E, CS> for Pop where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
            &self,
            _cs: &mut CS,
            stack: &mut Stack<E>,
            _bytecode: &mut Bytecode)
        -> Result<(), RuntimeError>
    {
        match stack.pop() {
            Some(_) => Ok(()),
            None => Err(RuntimeError::StackUnderflow),
        }
    }
}
