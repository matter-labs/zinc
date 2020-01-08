extern crate franklin_crypto;

use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{Cell, InternalVM, VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::Rem;

impl<E, O> VMInstruction<E, O> for Rem
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.pop()?.value()?;
        let right = vm.pop()?.value()?;
        let (_div, rem) = vm.operations().div_rem(left, right)?;

        vm.push(Cell::Value(rem))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_rem() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 4.into() })
            .add(PushConst { value: 9.into() })
            .add(Rem)
            .add(PushConst { value: (-4).into() })
            .add(PushConst { value: 9.into() })
            .add(Rem)
            .add(PushConst { value: 4.into() })
            .add(PushConst { value: (-9).into() })
            .add(Rem)
            .add(PushConst { value: (-4).into() })
            .add(PushConst { value: (-9).into() })
            .add(Rem)
            .test(&[3, 3, 1, 1])
    }
}
