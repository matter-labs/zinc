extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::Le;

impl<E, O> VMInstruction<E, O> for Le
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.pop()?;
        let right = vm.pop()?;

        let le = vm.get_operator().le(left, right)?;

        vm.push(le)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_le() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 1.into() })
            .add(PushConst { value: 2.into() })
            .add(Le)
            .add(PushConst { value: 2.into() })
            .add(PushConst { value: 2.into() })
            .add(Le)
            .add(PushConst { value: 2.into() })
            .add(PushConst { value: 1.into() })
            .add(Le)
            .add(PushConst { value: 2.into() })
            .add(PushConst { value: (-2).into() })
            .add(Le)
            .add(PushConst { value: (-2).into() })
            .add(PushConst { value: 2.into() })
            .add(Le)
            .test(&[0, 1, 1, 1, 0])
    }
}
