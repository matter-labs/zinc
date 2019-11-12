extern crate franklin_crypto;

use crate::{RuntimeError, VirtualMachine, VMInstruction, ElementOperator, Element};
use zrust_bytecode::instructions::NoOperation;

impl<E, O> VMInstruction<E, O> for NoOperation
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        Ok(())
    }
}
