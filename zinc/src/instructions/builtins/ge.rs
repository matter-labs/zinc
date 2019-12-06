extern crate franklin_crypto;

use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::Ge;

impl<E, O> VMInstruction<E, O> for Ge
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.pop()?;
        let right = vm.pop()?;

        let ge = vm.get_operator().ge(left, right)?;

        vm.push(ge)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_ge() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 1.into() })
            .add(PushConst { value: 2.into() })
            .add(Ge)
            .add(PushConst { value: 2.into() })
            .add(PushConst { value: 2.into() })
            .add(Ge)
            .add(PushConst { value: 2.into() })
            .add(PushConst { value: 1.into() })
            .add(Ge)
            .test(&[0, 1, 1])
    }
}
