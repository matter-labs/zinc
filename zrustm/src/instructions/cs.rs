extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::ConditionalSelect;

impl<E, O> VMInstruction<E, O> for ConditionalSelect
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let condition = vm.stack_pop()?;
        let if_true = vm.stack_pop()?;
        let if_false = vm.stack_pop()?;

        let selected = vm
            .get_operator()
            .conditional_select(condition, if_true, if_false)?;

        vm.stack_push(selected)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils;
    use num_bigint::BigInt;
    use zrust_bytecode::*;

    #[test]
    fn test_cs() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push {
            value: BigInt::from(1337),
        }));
        bytecode.push(Box::new(Push {
            value: BigInt::from(42),
        }));
        bytecode.push(Box::new(Push {
            value: BigInt::from(0),
        }));
        bytecode.push(Box::new(ConditionalSelect));
        bytecode.push(Box::new(Push {
            value: BigInt::from(420),
        }));
        bytecode.push(Box::new(Push {
            value: BigInt::from(69),
        }));
        bytecode.push(Box::new(Push {
            value: BigInt::from(1),
        }));
        bytecode.push(Box::new(ConditionalSelect));

        let mut vm = testing_utils::new_test_constrained_vm();
        vm.run(bytecode.as_mut_slice())?;

        testing_utils::assert_stack_eq(&vm, &[69, 1337]);

        let cs = vm.get_operator().constraint_system();
        assert_eq!(cs.find_unconstrained(), "", "unconstrained variables");
        assert!(cs.is_satisfied(), "satisfied");

        Ok(())
    }
}
