extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::LoadPushArray;

impl<E, O> VMInstruction<E, O> for LoadPushArray
    where
        E: Element,
        O: ElementOperator<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}
