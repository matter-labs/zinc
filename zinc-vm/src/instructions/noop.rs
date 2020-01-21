extern crate franklin_crypto;

use crate::gadgets::PrimitiveOperations;
use crate::vm::{RuntimeError, VMInstruction, VirtualMachine};
use crate::ZincEngine;
use zinc_bytecode::instructions::NoOperation;

impl<E, O> VMInstruction<E, O> for NoOperation
where
    E: ZincEngine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        Ok(())
    }
}
