use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::Lt;

use crate::core::state::cell::Cell;
use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;

impl<VM: VirtualMachine> VMInstruction<VM> for Lt {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;

        let cs = vm.constraint_system();
        let lt = gadgets::comparison::lesser_than(cs.namespace(|| "lt"), &left, &right)?;

        vm.push(Cell::Value(lt))
    }
}

#[cfg(test)]
mod test {
    use ff::Field;
    use pairing::bn256::Fr;

    use zinc_bytecode::IntegerType;
    use zinc_bytecode::ScalarType;

    use crate::gadgets::fr_bigint::fr_to_bigint;
    use crate::tests::TestingError;
    use crate::tests::VMTestRunner;

    #[test]
    fn simple() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(zinc_bytecode::Push::new(2.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Push::new(1.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Lt)
            .add(zinc_bytecode::Push::new(2.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Push::new(2.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Lt)
            .add(zinc_bytecode::Push::new(1.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Push::new(2.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Lt)
            .test(&[1, 0, 0])
    }

    #[test]
    fn edge_cases() -> Result<(), TestingError> {
        let mut max_fr = Fr::zero();
        max_fr.sub_assign(&Fr::one());
        let max = fr_to_bigint(&max_fr, false);

        VMTestRunner::new()
            .add(zinc_bytecode::Push::new(max.clone(), ScalarType::Field))
            .add(zinc_bytecode::Push::new(0.into(), ScalarType::Field))
            .add(zinc_bytecode::Lt)
            .add(zinc_bytecode::Push::new(0.into(), ScalarType::Field))
            .add(zinc_bytecode::Push::new(max.clone(), ScalarType::Field))
            .add(zinc_bytecode::Lt)
            .add(zinc_bytecode::Push::new(1.into(), ScalarType::Field))
            .add(zinc_bytecode::Push::new(max.clone(), ScalarType::Field))
            .add(zinc_bytecode::Lt)
            .test(&[1, 1, 0])
    }
}
