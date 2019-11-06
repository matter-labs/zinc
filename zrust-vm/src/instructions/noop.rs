use crate::{RuntimeError, Stack, VMInstruction};
use franklin_crypto::bellman::ConstraintSystem;
use bellman::pairing::Engine;
use zrust_bytecode::instructions::NoOperation;

impl<E, CS> VMInstruction<E, CS> for NoOperation where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
        &self,
        _cs: &mut CS,
        _stack: &mut Stack<E>)
        -> Result<(), RuntimeError>
    {
        Ok(())
    }
}
