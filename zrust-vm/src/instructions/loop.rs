extern crate franklin_crypto;

use crate::{RuntimeError, VirtualMachine, VMInstruction, ElementOperator, Element};
use zrust_bytecode::{LoopBegin, LoopEnd};

impl<E, O> VMInstruction<E, O> for LoopBegin
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.loop_begin(self.iterations, self.io_size)
    }
}

impl<E, O> VMInstruction<E, O> for LoopEnd
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.loop_end()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils;
    use num_bigint::BigInt;
    use zrust_bytecode::{Push, Add};

    #[test]
    fn test_loop() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push { value: BigInt::from(42) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0) }));
        bytecode.push(Box::new(LoopBegin::new(10, 1)));
        bytecode.push(Box::new(Push { value: BigInt::from(1) }));
        bytecode.push(Box::new(Add));
        bytecode.push(Box::new(LoopEnd));

        let mut vm = testing_utils::new_test_constrained_vm();
        vm.run(bytecode.as_mut_slice())?;

        testing_utils::assert_stack_eq(&vm, &[10, 42]);

        let cs = vm.get_operator().constraint_system();
        assert_eq!(cs.find_unconstrained(), "", "unconstrained variables");
        assert!(cs.is_satisfied(), "satisfied");

        Ok(())
    }
}
