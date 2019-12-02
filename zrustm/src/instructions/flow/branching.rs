extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::{If, Else, EndIf};

impl<E, O> VMInstruction<E, O> for If
    where
        E: Element,
        O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.branch_then()
    }
}

impl<E, O> VMInstruction<E, O> for Else
    where
        E: Element,
        O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.branch_else()
    }
}

impl<E, O> VMInstruction<E, O> for EndIf
    where
        E: Element,
        O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.branch_end()
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::testing_utils::{VMTestRunner, TestingError};
    use num_bigint::BigInt;
    use zrust_bytecode::*;
    use std::cmp;

    #[test]
    fn test_branching() -> Result<(), TestingError> {
        // let a = _;
        // let b = _;
        //
        // if a > b {
        //     (a, b)
        // } else {
        //     (b, a)
        // }
        let data = [
            (5, 7),
            (7, 5),
            (6, 6),
        ];

        for (a, b) in data.iter() {
            VMTestRunner::new()
                .add(PushConst { value: (*a).into() })
                .add(PopStore::new(0))
                .add(PushConst { value: (*b).into() })
                .add(PopStore::new(1))
                .add(LoadPush::new(0))
                .add(LoadPush::new(1))
                .add(Gt)
                .add(If)
                .add(LoadPush::new(0))
                .add(LoadPush::new(1))
                .add(Else)
                .add(LoadPush::new(1))
                .add(LoadPush::new(0))
                .add(EndIf)
                .test(&[cmp::max((*a), (*b)), cmp::min((*a), (*b))])?;
        }

        Ok(())
    }
}
