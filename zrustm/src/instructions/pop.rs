extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::Pop;

impl<E, O> VMInstruction<E, O> for Pop
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        for _ in 0..self.count {
            vm.stack_pop()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils;
    use num_bigint::BigInt;
    use zrust_bytecode::*;
    use crate::instructions::testing_utils::{VMTestRunner, TestingError};

    #[test]
    fn test_pop() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(Push { value: BigInt::from(1) })
            .add(Push { value: BigInt::from(2) })
            .add(Pop::new(1))
            .add(Push { value: BigInt::from(3) })
            .add(Push { value: BigInt::from(4) })
            .add(Push { value: BigInt::from(5) })
            .add(Pop::new(2))
            .test(&[3, 1])
    }
}
