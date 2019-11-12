extern crate franklin_crypto;

use crate::{RuntimeError, VirtualMachine, VMInstruction, ElementOperator, Element};
use zrust_bytecode::instructions::Sub;

impl<E, O> VMInstruction<E, O> for Sub
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let right = vm.stack_pop()?;
        let left = vm.stack_pop()?;
        let diff = vm.get_operator().sub(left, right)?;

        vm.stack_push(diff)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils;
    use zrust_bytecode::*;
    use num_bigint::BigInt;

    #[test]
    fn test_sub() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push { value: BigInt::from(0x02) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x01) }));
        bytecode.push(Box::new(Sub));

        let stack = testing_utils::execute(bytecode.as_slice())?;

        assert_eq!(stack.len(), 1);
        testing_utils::assert_stack_value(&stack, 0, "0x01");

        Ok(())
    }
}
