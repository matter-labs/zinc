extern crate franklin_crypto;

use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::PopStore;

impl<E, O> VMInstruction<E, O> for PopStore
    where
        E: Primitive,
        O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let value = vm.pop()?;
        vm.store(self.index, value)
    }
}
