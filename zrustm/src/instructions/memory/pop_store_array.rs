extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::PopStoreArray;

impl<E, O> VMInstruction<E, O> for PopStoreArray
    where
        E: Element,
        O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        for i in 0..self.len {
            let value = vm.pop()?;
            vm.store(self.address + i, value)?;
        }

        Ok(())
    }
}
