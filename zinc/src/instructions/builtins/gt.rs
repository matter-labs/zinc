extern crate franklin_crypto;

use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::Gt;

impl<E, O> VMInstruction<E, O> for Gt
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.pop()?;
        let right = vm.pop()?;

        let gt = vm.get_operator().gt(left, right)?;

        vm.push(gt)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_gt() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 1.into() })
            .add(PushConst { value: 2.into() })
            .add(Gt)
            .add(PushConst { value: 2.into() })
            .add(PushConst { value: 2.into() })
            .add(Gt)
            .add(PushConst { value: 2.into() })
            .add(PushConst { value: 1.into() })
            .add(Gt)
            .test(&[0, 0, 1])
    }
}
