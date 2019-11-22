extern crate franklin_crypto;

use zrust_bytecode::instructions::NoOperation;
use crate::element::{Element, ElementOperator};
use crate::vm::{VirtualMachine, RuntimeError, VMInstruction};

impl<E, O> VMInstruction<E, O> for NoOperation
    where E: Element, O: ElementOperator<E>
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        Ok(())
    }
}
