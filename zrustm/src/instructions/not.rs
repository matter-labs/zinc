extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::Not;

impl<E, O> VMInstruction<E, O> for Not
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let value = vm.stack_pop()?;

        let not = vm.get_operator().not(value)?;

        vm.stack_push(not)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils;
    use num_bigint::BigInt;
    use zrust_bytecode::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};

    #[test]
    fn test_not() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(Push { value: 0.into() })
            .add(Not)
            .add(Push { value: 1.into() })
            .add(Not)
            .test(&[0, 1])
    }
}
