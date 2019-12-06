extern crate franklin_crypto;

use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{RuntimeError, VMInstruction, VirtualMachine};
use zinc_bytecode::instructions::Dbg;

impl<E, O> VMInstruction<E, O> for Dbg
    where
        E: Primitive,
        O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        Ok(())
    }
}
