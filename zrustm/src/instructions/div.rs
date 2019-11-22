extern crate franklin_crypto;

use crate::vm::VMInstruction;
use crate::element::{Element, ElementOperator};
use crate::vm::{VirtualMachine, RuntimeError};
use zrust_bytecode::instructions::Div;

impl<E, O> VMInstruction<E, O> for Div
    where E: Element, O: ElementOperator<E>
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.stack_pop()?;
        let right = vm.stack_pop()?;
        let (div, _rem) = vm.get_operator().div_rem(left, right)?;

        vm.stack_push(div)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zrust_bytecode::*;
    use crate::instructions::testing_utils::{VMTestRunner, TestingError};

    #[test]
    fn test_div() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(Push { value: (4).into() })
            .add(Push { value: (9).into() })
            .add(Div)

            .add(Push { value: (-4).into() })
            .add(Push { value: (9).into() })
            .add(Div)

            .add(Push { value: (4).into() })
            .add(Push { value: (-9).into() })
            .add(Div)

            .add(Push { value: (-4).into() })
            .add(Push { value: (-9).into() })
            .add(Div)

            .test(&[3, -3, -2, 2])
    }
}
