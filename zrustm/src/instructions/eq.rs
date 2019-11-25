extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::Eq;

impl<E, O> VMInstruction<E, O> for Eq
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.stack_pop()?;
        let right = vm.stack_pop()?;

        let eq = vm.get_operator().eq(left, right)?;

        vm.stack_push(eq)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils;
    use num_bigint::BigInt;
    use zrust_bytecode::*;

    #[test]
    fn test_eq() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push {
            value: BigInt::from(1),
        }));
        bytecode.push(Box::new(Push {
            value: BigInt::from(2),
        }));
        bytecode.push(Box::new(Eq));
        bytecode.push(Box::new(Push {
            value: BigInt::from(2),
        }));
        bytecode.push(Box::new(Push {
            value: BigInt::from(2),
        }));
        bytecode.push(Box::new(Eq));
        bytecode.push(Box::new(Push {
            value: BigInt::from(2),
        }));
        bytecode.push(Box::new(Push {
            value: BigInt::from(1),
        }));
        bytecode.push(Box::new(Eq));

        let mut vm = testing_utils::new_test_constrained_vm();
        vm.run(bytecode.as_mut_slice())?;

        testing_utils::assert_stack_eq(&vm, &[0, 1, 0]);

        let cs = vm.get_operator().constraint_system();
        assert_eq!(cs.find_unconstrained(), "", "unconstrained variables");
        assert!(cs.is_satisfied(), "satisfied");

        Ok(())
    }
}
