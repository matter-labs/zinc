extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::PopStore;

impl<E, O> VMInstruction<E, O> for PopStore
    where
        E: Element,
        O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let frame = vm.memory()?;

        let value = frame.pop()?;
        frame.store(self.index, value)
    }
}
