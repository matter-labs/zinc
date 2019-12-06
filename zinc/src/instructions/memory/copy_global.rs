extern crate franklin_crypto;

use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::CopyGlobal;

impl<E, O> VMInstruction<E, O> for CopyGlobal
    where
        E: Primitive,
        O: PrimitiveOperations<E>,
{
    #[allow(deprecated)]
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let value = vm.memory()?.copy(self.index)?;
        vm.push(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_copy_global() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 1.into() })
            .add(PushConst { value: 2.into() })
            .add(PushConst { value: 3.into() })
            .add(CopyGlobal::new(0))
            .add(CopyGlobal::new(2))
            .test(&[3, 1, 3, 2, 1])
    }
}
