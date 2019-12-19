extern crate franklin_crypto;

use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{RuntimeError, VMInstruction, VirtualMachine, InternalVM};
use zinc_bytecode::instructions::Dbg;

impl<E, O> VMInstruction<E, O> for Dbg
    where
        E: Primitive,
        O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        print!("{}", self.string);

        for _ in 0..self.nargs {
            let v = vm.pop()?.value()?;
            print!(" {}", v)
        }

        Ok(())
    }
}
