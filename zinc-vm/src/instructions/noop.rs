extern crate franklin_crypto;

use crate::gadgets::{PrimitiveOperations};
use crate::vm::{RuntimeError, VMInstruction, VirtualMachine};
use pairing::Engine;
use zinc_bytecode::instructions::NoOperation;

impl<E, O> VMInstruction<E, O> for NoOperation
where
    E: Engine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        Ok(())
    }
}
