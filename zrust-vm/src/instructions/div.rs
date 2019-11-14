extern crate franklin_crypto;

use crate::{RuntimeError, VirtualMachine, VMInstruction, ElementOperator, Element};
use zrust_bytecode::instructions::Div;

impl<E, O> VMInstruction<E, O> for Div
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.stack_pop()?;
        let right = vm.stack_pop()?;
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
        bytecode.push(Box::new(Push { value: BigInt::from(4) }));
        bytecode.push(Box::new(Push { value: BigInt::from(16) }));
        bytecode.push(Box::new(Div));
        bytecode.push(Box::new(Push { value: BigInt::from(4) }));
        bytecode.push(Box::new(Push { value: BigInt::from(9) }));
        bytecode.push(Box::new(Div));
        bytecode.push(Box::new(Push { value: BigInt::from(-4) }));
        bytecode.push(Box::new(Push { value: BigInt::from(9) }));
        bytecode.push(Box::new(Div));

        let mut vm = testing_utils::create_vm();
        vm.run(bytecode.as_mut_slice())?;

        testing_utils::assert_stack_eq(&vm, &[-2, 2, 4]);

        Ok(())
    }
}
