extern crate franklin_crypto;

use crate::{RuntimeError, VirtualMachine, VMInstruction, ElementOperator, Element};
use zrust_bytecode::instructions::Push;

impl<E, O> VMInstruction<E, O> for Push
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let op = vm.get_operator();
        let value = op.constant_bigint(&self.value)?;

        vm.stack_push(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils;
    use num_bigint::BigInt;

    #[test]
    fn test_push() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push { value: BigInt::from(0) }));
        bytecode.push(Box::new(Push { value: BigInt::from(42) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0xABCD) }));
        bytecode.push(Box::new(Push { value: BigInt::from(-1) }));
        bytecode.push(Box::new(Push { value: BigInt::from(-1000) }));

        let mut vm = testing_utils::new_test_constrained_vm();
        vm.run(bytecode.as_mut_slice())?;

        testing_utils::assert_stack_eq(&vm, &[-1000, -1, 0xABCD, 42, 0]);

        let cs = vm.get_operator().constraint_system();
        assert_eq!(cs.find_unconstrained(), "", "unconstrained variables");
        assert!(cs.is_satisfied(), "satisfied");

        Ok(())
    }
}
