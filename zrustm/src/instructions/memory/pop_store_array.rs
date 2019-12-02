extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::{VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::PopStoreArray;

impl<E, O> VMInstruction<E, O> for PopStoreArray
    where
        E: Element,
        O: ElementOperator<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}
