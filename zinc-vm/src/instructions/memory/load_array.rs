use crate::gadgets::{PrimitiveOperations};
use crate::vm::{InternalVM, VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use pairing::Engine;
use zinc_bytecode::instructions::LoadSequence;

impl<E, O> VMInstruction<E, O> for LoadSequence
where
    E: Engine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        for i in 0..self.len {
            let value = vm.load(self.address + self.len - i - 1)?;
            vm.push(value)?;
        }

        Ok(())
    }
}
