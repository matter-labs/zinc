//!
//! The `Lesser` instruction.
//!

use franklin_crypto::bellman::ConstraintSystem;

use zinc_build::Lt;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Lt {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;

        let cs = vm.constraint_system();
        let lt = gadgets::comparison::lesser_than(cs.namespace(|| "lt"), &left, &right)?;

        vm.push(Cell::Value(lt))
    }
}

#[cfg(test)]
mod test {
    use franklin_crypto::bellman::pairing::bn256::Bn256;
    use franklin_crypto::bellman::pairing::bn256::Fr;
    use franklin_crypto::bellman::pairing::ff::Field;

    use zinc_build::IntegerType;
    use zinc_build::ScalarType;

    use crate::gadgets;
    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn simple() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_build::Push::new(2.into(), IntegerType::I8.into()))
            .push(zinc_build::Push::new(1.into(), IntegerType::I8.into()))
            .push(zinc_build::Lt)
            .push(zinc_build::Push::new(2.into(), IntegerType::I8.into()))
            .push(zinc_build::Push::new(2.into(), IntegerType::I8.into()))
            .push(zinc_build::Lt)
            .push(zinc_build::Push::new(1.into(), IntegerType::I8.into()))
            .push(zinc_build::Push::new(2.into(), IntegerType::I8.into()))
            .push(zinc_build::Lt)
            .test(&[1, 0, 0])
    }

    #[test]
    fn edge_cases() -> Result<(), TestingError> {
        let mut max_fr = Fr::zero();
        max_fr.sub_assign(&Fr::one());
        let max = gadgets::scalar::fr_bigint::fr_to_bigint::<Bn256>(&max_fr, false);

        TestRunner::new()
            .push(zinc_build::Push::new(max.clone(), ScalarType::Field))
            .push(zinc_build::Push::new(0.into(), ScalarType::Field))
            .push(zinc_build::Lt)
            .push(zinc_build::Push::new(0.into(), ScalarType::Field))
            .push(zinc_build::Push::new(max.clone(), ScalarType::Field))
            .push(zinc_build::Lt)
            .push(zinc_build::Push::new(1.into(), ScalarType::Field))
            .push(zinc_build::Push::new(max.clone(), ScalarType::Field))
            .push(zinc_build::Lt)
            .test(&[1, 1, 0])
    }
}
