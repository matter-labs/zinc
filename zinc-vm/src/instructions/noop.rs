extern crate franklin_crypto;


use crate::core::{RuntimeError, VMInstruction, VirtualMachine};
use crate::Engine;
use zinc_bytecode::instructions::NoOperation;
use self::franklin_crypto::bellman::ConstraintSystem;

impl<E, CS> VMInstruction<E, CS> for NoOperation
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        Ok(())
    }
}
