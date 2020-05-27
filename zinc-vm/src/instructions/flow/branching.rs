extern crate franklin_crypto;


use crate::core::{VirtualMachine, VMInstruction};
use crate::core::{RuntimeError};

use zinc_bytecode::{Else, EndIf, If};

impl<VM: VirtualMachine> VMInstruction<VM> for If {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        vm.branch_then()
    }
}

impl<VM: VirtualMachine> VMInstruction<VM> for Else {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        vm.branch_else()
    }
}

impl<VM: VirtualMachine> VMInstruction<VM> for EndIf {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        vm.branch_end()
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use std::cmp;
    use zinc_bytecode::scalar::{IntegerType, ScalarType};
    use zinc_bytecode::*;

    #[test]
    fn test_stack() -> Result<(), TestingError> {
        // let a = _;
        // let b = _;
        //
        // if a > b {
        //     (a, b)
        // } else {
        //     (b, a)
        // }
        let data = [(5, 7), (7, 5), (6, 6)];

        for (a, b) in data.iter() {
            VMTestRunner::new()
                .add(PushConst::new((*a).into(), IntegerType::I8.into()))
                .add(Store::new(0))
                .add(PushConst::new((*b).into(), IntegerType::I8.into()))
                .add(Store::new(1))
                .add(Load::new(1))
                .add(Load::new(0))
                .add(Gt)
                .add(If)
                .add(Load::new(0))
                .add(Load::new(1))
                .add(Else)
                .add(Load::new(1))
                .add(Load::new(0))
                .add(EndIf)
                .test(&[cmp::max(*a, *b), cmp::min(*a, *b)])?;
        }

        Ok(())
    }

    #[test]
    fn test_storage() -> Result<(), TestingError> {
        let _ = env_logger::builder().is_test(true).try_init();
        // let mut a = 0;
        // let c = _;
        //
        // if c {
        //     a += 1;
        // } else {
        //     a -= 1;
        // }
        let data = [(1, 1), (0, -1)];

        for (c, r) in data.iter() {
            VMTestRunner::new()
                .add(PushConst::new(0.into(), IntegerType::I8.into()))
                .add(Store::new(0))
                .add(PushConst::new((*c).into(), ScalarType::Boolean))
                .add(If)
                .add(PushConst::new(1.into(), IntegerType::I8.into()))
                .add(Load::new(0))
                .add(Add)
                .add(Store::new(0))
                .add(Else)
                .add(Load::new(0))
                .add(PushConst::new(1.into(), IntegerType::I8.into()))
                .add(Sub)
                .add(Store::new(0))
                .add(EndIf)
                .add(Load::new(0))
                .test(&[*r])?;
        }
        Ok(())
    }
}
