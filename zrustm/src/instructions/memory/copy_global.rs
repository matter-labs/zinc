extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::CopyGlobal;

impl<E, O> VMInstruction<E, O> for CopyGlobal
    where
        E: Element,
        O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let value = vm.frame()?.copy(self.index)?;
        vm.frame()?.push(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zrust_bytecode::*;

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
