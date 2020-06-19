//!
//! The virtual machine unit test facade.
//!

use franklin_crypto::bellman::groth16::Parameters;
use franklin_crypto::bellman::groth16::Proof;
use franklin_crypto::bellman::pairing::bn256::Bn256;
use franklin_crypto::circuit::test::TestConstraintSystem;

use zinc_bytecode::TemplateValue;
use zinc_bytecode::UnitTest as BytecodeUnitTest;

use crate::core::facade::IFacade;
use crate::core::unit_test::UnitTest;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::IEngine;

impl IFacade for BytecodeUnitTest {
    fn run<E: IEngine>(self, _input: TemplateValue) -> Result<TemplateValue, RuntimeError> {
        Err(RuntimeError::CommandForbidden)
    }

    fn debug<E: IEngine>(self, _input: TemplateValue) -> Result<TemplateValue, RuntimeError> {
        Err(RuntimeError::CommandForbidden)
    }

    fn test<E: IEngine>(self) -> Result<(), RuntimeError> {
        let cs = TestConstraintSystem::<Bn256>::new();

        let mut unit_test = UnitTest::new(cs);

        let mut num_constraints = 0;
        unit_test.run(
            self,
            |cs| {
                let num = cs.num_constraints() - num_constraints;
                num_constraints += num;
                log::debug!("Constraints: {}", num);
            },
            |cs| {
                if !cs.is_satisfied() {
                    return Err(RuntimeError::UnsatisfiedConstraint);
                }

                Ok(())
            },
        )?;

        let cs = unit_test.constraint_system();

        log::debug!("{}", cs.pretty_print());

        if !cs.is_satisfied() {
            log::error!(
                "Unsatisfied: {}",
                cs.which_is_unsatisfied()
                    .expect(crate::panic::VALUE_ALWAYS_EXISTS)
            );
            return Err(RuntimeError::UnsatisfiedConstraint);
        }

        let unconstrained = cs.find_unconstrained();
        if !unconstrained.is_empty() {
            log::error!("Unconstrained: {}", unconstrained);
            return Err(RuntimeError::InternalError(
                "Generated unconstrained variables".into(),
            ));
        }

        Ok(())
    }

    fn setup<E: IEngine>(self) -> Result<Parameters<E>, RuntimeError> {
        Err(RuntimeError::CommandForbidden)
    }

    fn prove<E: IEngine>(
        self,
        _params: Parameters<E>,
        _witness: TemplateValue,
    ) -> Result<(TemplateValue, Proof<E>), RuntimeError> {
        Err(RuntimeError::CommandForbidden)
    }
}
