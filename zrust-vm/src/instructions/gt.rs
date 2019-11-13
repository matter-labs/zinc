extern crate franklin_crypto;

use crate::{RuntimeError, VirtualMachine, VMInstruction, ElementOperator, Element};
use zrust_bytecode::instructions::Gt;

impl<E, O> VMInstruction<E, O> for Gt
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.stack_pop()?;
        let right = vm.stack_pop()?;

        let gt = vm.get_operator().gt(left, right)?;

        vm.stack_push(gt)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils;
    use zrust_bytecode::*;
    use num_bigint::BigInt;

    #[test]
    fn test_gt() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push { value: BigInt::from(1) }));
        bytecode.push(Box::new(Push { value: BigInt::from(2) }));
        bytecode.push(Box::new(Gt));
        bytecode.push(Box::new(Push { value: BigInt::from(2) }));
        bytecode.push(Box::new(Push { value: BigInt::from(2) }));
        bytecode.push(Box::new(Gt));
        bytecode.push(Box::new(Push { value: BigInt::from(2) }));
        bytecode.push(Box::new(Push { value: BigInt::from(1) }));
        bytecode.push(Box::new(Gt));

        let mut vm = testing_utils::create_vm();
        vm.run(bytecode.as_mut_slice())?;

        testing_utils::assert_stack_eq(&vm, &[0, 0, 1]);

        Ok(())
    }
}
