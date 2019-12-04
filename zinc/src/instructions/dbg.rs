extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::{RuntimeError, VMInstruction, VirtualMachine};
use zinc_bytecode::instructions::Dbg;

impl<E, O> VMInstruction<E, O> for Dbg
    where
        E: Element,
        O: ElementOperator<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        Ok(())
    }
}
