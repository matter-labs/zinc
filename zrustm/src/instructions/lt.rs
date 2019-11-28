extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::Lt;

impl<E, O> VMInstruction<E, O> for Lt
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.stack_pop()?;
        let right = vm.stack_pop()?;

        let lt = vm.get_operator().lt(left, right)?;

        vm.stack_push(lt)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zrust_bytecode::*;

    #[test]
    fn test_lt() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 1.into() })
            .add(PushConst { value: 2.into() })
            .add(Lt)
            .add(PushConst { value: 2.into() })
            .add(PushConst { value: 2.into() })
            .add(Lt)
            .add(PushConst { value: 2.into() })
            .add(PushConst { value: 1.into() })
            .add(Lt)
            .test(&[1, 0, 0])
    }
}
