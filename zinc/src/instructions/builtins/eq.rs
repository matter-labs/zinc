extern crate franklin_crypto;

use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{VMInstruction, InternalVM, Cell};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::Eq;

impl<E, O> VMInstruction<E, O> for Eq
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.pop()?.value()?;
        let right = vm.pop()?.value()?;

        let eq = vm.operations().eq(left, right)?;

        vm.push(Cell::Value(eq))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_eq() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 1.into() })
            .add(PushConst { value: 2.into() })
            .add(Eq)
            .add(PushConst { value: 2.into() })
            .add(PushConst { value: 2.into() })
            .add(Eq)
            .add(PushConst { value: 2.into() })
            .add(PushConst { value: 1.into() })
            .add(Eq)
            .test(&[0, 1, 0])
    }
}
