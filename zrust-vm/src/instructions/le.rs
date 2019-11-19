extern crate franklin_crypto;

use crate::{RuntimeError, VirtualMachine, VMInstruction, ElementOperator, Element};
use zrust_bytecode::instructions::Le;

impl<E, O> VMInstruction<E, O> for Le
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.stack_pop()?;
        let right = vm.stack_pop()?;

        let le = vm.get_operator().le(left, right)?;

        vm.stack_push(le)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils;
    use zrust_bytecode::*;
    use num_bigint::BigInt;

    #[test]
    fn test_le() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push { value: BigInt::from(1) }));
        bytecode.push(Box::new(Push { value: BigInt::from(2) }));
        bytecode.push(Box::new(Le));
        bytecode.push(Box::new(Push { value: BigInt::from(2) }));
        bytecode.push(Box::new(Push { value: BigInt::from(2) }));
        bytecode.push(Box::new(Le));
        bytecode.push(Box::new(Push { value: BigInt::from(2) }));
        bytecode.push(Box::new(Push { value: BigInt::from(1) }));
        bytecode.push(Box::new(Le));
        bytecode.push(Box::new(Push { value: BigInt::from(2) }));
        bytecode.push(Box::new(Push { value: BigInt::from(-2) }));
        bytecode.push(Box::new(Le));
        bytecode.push(Box::new(Push { value: BigInt::from(-2) }));
        bytecode.push(Box::new(Push { value: BigInt::from(2) }));
        bytecode.push(Box::new(Le));


        let mut vm = testing_utils::new_test_constrained_vm();
        vm.run(bytecode.as_mut_slice())?;

        testing_utils::assert_stack_eq(&vm, &[0, 1, 1, 1, 0]);

        let cs = vm.get_operator().constraint_system();
        assert_eq!(cs.find_unconstrained(), "", "unconstrained variables");
        assert!(cs.is_satisfied(), "satisfied");

        Ok(())
    }
}
