use crate::gadgets::PrimitiveOperations;
use crate::vm::{InternalVM, VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use crate::ZincEngine;
use zinc_bytecode::instructions::Store;

impl<E, O> VMInstruction<E, O> for Store
where
    E: ZincEngine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let value = vm.pop()?;
        vm.store(self.index, value)
    }
}
