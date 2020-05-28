use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::Gt;

use crate::core::state::cell::Cell;
use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;

impl<VM: VirtualMachine> VMInstruction<VM> for Gt {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let cs = vm.constraint_system();
        let gt = gadgets::comparison::gt(cs.namespace(|| "gt"), &left, &right)?;

        vm.push(Cell::Value(gt))
    }
}

#[cfg(test)]
mod test {
    use crate::tests::TestingError;
    use crate::tests::VMTestRunner;

    use zinc_bytecode::IntegerType;

    #[test]
    fn test_gt() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(zinc_bytecode::Push::new(2.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Push::new(1.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Gt)
            .add(zinc_bytecode::Push::new(2.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Push::new(2.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Gt)
            .add(zinc_bytecode::Push::new(1.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Push::new(2.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Gt)
            .test(&[0, 0, 1])
    }
}
