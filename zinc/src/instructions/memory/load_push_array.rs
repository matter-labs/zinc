extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::LoadPushArray;

impl<E, O> VMInstruction<E, O> for LoadPushArray
    where
        E: Element,
        O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        for i in 0..self.len {
            let value = vm.load(self.address + self.len - i - 1)?;
            vm.push(value)?;
        }

        Ok(())
    }
}
