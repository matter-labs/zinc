use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::Le;

use crate::core::state::cell::Cell;
use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;

impl<VM: VirtualMachine> VMInstruction<VM> for Le {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let cs = vm.constraint_system();
        let le = gadgets::comparison::le(cs.namespace(|| "le"), &left, &right)?;

        vm.push(Cell::Value(le))
    }
}

#[cfg(test)]
mod test {
    use crate::tests::TestingError;
    use crate::tests::VMTestRunner;

    use zinc_bytecode::IntegerType;

    #[test]
    fn test_le() -> Result<(), TestingError> {
        let _ = env_logger::builder().is_test(true).try_init();

        VMTestRunner::new()
            .add(zinc_bytecode::Push::new(2.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Push::new(1.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Le)
            .add(zinc_bytecode::Push::new(2.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Push::new(2.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Le)
            .add(zinc_bytecode::Push::new(1.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Push::new(2.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Le)
            .add(zinc_bytecode::Push::new(
                (-2).into(),
                IntegerType::I8.into(),
            ))
            .add(zinc_bytecode::Push::new(2.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Le)
            .add(zinc_bytecode::Push::new(2.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Push::new(
                (-2).into(),
                IntegerType::I8.into(),
            ))
            .add(zinc_bytecode::Le)
            .test(&[0, 1, 1, 1, 0])
    }
}
