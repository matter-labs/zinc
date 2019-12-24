extern crate franklin_crypto;

use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{VMInstruction, InternalVM, Cell};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::Xor;

impl<E, O> VMInstruction<E, O> for Xor
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.pop()?.value()?;
        let right = vm.pop()?.value()?;

        let xor = vm.get_operator().xor(left, right)?;

        vm.push(Cell::Value(xor))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_xor() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 0.into() })
            .add(PushConst { value: 0.into() })
            .add(Xor)
            .add(PushConst { value: 0.into() })
            .add(PushConst { value: 1.into() })
            .add(Xor)
            .add(PushConst { value: 1.into() })
            .add(PushConst { value: 0.into() })
            .add(Xor)
            .add(PushConst { value: 1.into() })
            .add(PushConst { value: 1.into() })
            .add(Xor)
            .test(&[0, 1, 1, 0])
    }
}
