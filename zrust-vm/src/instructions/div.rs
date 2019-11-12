extern crate franklin_crypto;

use crate::{RuntimeError, VirtualMachine, VMInstruction, ElementOperator, Element};
use zrust_bytecode::instructions::Div;

impl<E, O> VMInstruction<E, O> for Div
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let right = vm.stack_pop()?;
        let left = vm.stack_pop()?;
        let (div, _rem) = vm.get_operator().div_rem(left, right)?;

        vm.stack_push(div)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils;
    use zrust_bytecode::*;
    use num_bigint::BigInt;

    #[test]
    fn test_div() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push { value: BigInt::from(0x10) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x04) }));
        bytecode.push(Box::new(Div));
        bytecode.push(Box::new(Push { value: BigInt::from(0x9) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x4) }));
        bytecode.push(Box::new(Div));

        let stack = testing_utils::execute(bytecode.as_slice())?;

        assert_eq!(stack.len(), 2);
        testing_utils::assert_stack_value(&stack, 0, "0x02");
        testing_utils::assert_stack_value(&stack, 1, "0x04");

        Ok(())
    }
}
