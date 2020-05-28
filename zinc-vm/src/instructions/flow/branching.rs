use zinc_bytecode::Else;
use zinc_bytecode::EndIf;
use zinc_bytecode::If;

use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;

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
    use std::cmp;

    use zinc_bytecode::IntegerType;
    use zinc_bytecode::ScalarType;

    use crate::tests::TestingError;
    use crate::tests::VMTestRunner;

    #[ignore]
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
                .add(zinc_bytecode::Push::new(
                    (*a).into(),
                    IntegerType::I8.into(),
                ))
                .add(zinc_bytecode::Store::new(0, 1))
                .add(zinc_bytecode::Push::new(
                    (*b).into(),
                    IntegerType::I8.into(),
                ))
                .add(zinc_bytecode::Store::new(1, 1))
                .add(zinc_bytecode::Load::new(1, 1))
                .add(zinc_bytecode::Load::new(0, 1))
                .add(zinc_bytecode::Gt)
                .add(zinc_bytecode::If)
                .add(zinc_bytecode::Load::new(0, 1))
                .add(zinc_bytecode::Load::new(1, 1))
                .add(zinc_bytecode::Else)
                .add(zinc_bytecode::Load::new(1, 1))
                .add(zinc_bytecode::Load::new(0, 1))
                .add(zinc_bytecode::EndIf)
                .test(&[cmp::max(*a, *b), cmp::min(*a, *b)])?;
        }

        Ok(())
    }

    #[ignore]
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
                .add(zinc_bytecode::Push::new(0.into(), IntegerType::I8.into()))
                .add(zinc_bytecode::Store::new(0, 1))
                .add(zinc_bytecode::Push::new((*c).into(), ScalarType::Boolean))
                .add(zinc_bytecode::If)
                .add(zinc_bytecode::Push::new(1.into(), IntegerType::I8.into()))
                .add(zinc_bytecode::Load::new(0, 1))
                .add(zinc_bytecode::Add)
                .add(zinc_bytecode::Store::new(0, 1))
                .add(zinc_bytecode::Else)
                .add(zinc_bytecode::Load::new(0, 1))
                .add(zinc_bytecode::Push::new(1.into(), IntegerType::I8.into()))
                .add(zinc_bytecode::Sub)
                .add(zinc_bytecode::Store::new(0, 1))
                .add(zinc_bytecode::EndIf)
                .add(zinc_bytecode::Load::new(0, 1))
                .test(&[*r])?;
        }
        Ok(())
    }
}
