//!
//! The virtual machine unit test facade.
//!

use colored::Colorize;

use franklin_crypto::bellman::groth16::Parameters;
use franklin_crypto::bellman::groth16::Proof;
use franklin_crypto::bellman::pairing::bn256::Bn256;
use franklin_crypto::circuit::test::TestConstraintSystem;

use zinc_bytecode::TemplateValue;
use zinc_bytecode::UnitTest as BytecodeUnitTest;
use zinc_const::UnitTestExitCode;

use crate::core::facade::IFacade;
use crate::core::unit_test::UnitTest;
use crate::error::RuntimeError;
use crate::IEngine;

impl IFacade for BytecodeUnitTest {
    fn run<E: IEngine>(self, _input: TemplateValue) -> Result<TemplateValue, RuntimeError> {
        Err(RuntimeError::CommandForbidden)
    }

    fn debug<E: IEngine>(self, _input: TemplateValue) -> Result<TemplateValue, RuntimeError> {
        Err(RuntimeError::CommandForbidden)
    }

    fn test<E: IEngine>(self) -> Result<UnitTestExitCode, RuntimeError> {
        if self.is_ignored {
            println!("test {} ... {}", self.name, "ignore".yellow());
            return Ok(UnitTestExitCode::Ignored);
        }

        let name = self.name.clone();
        let should_panic = self.should_panic;

        let cs = TestConstraintSystem::<Bn256>::new();
        let mut unit_test = UnitTest::new(cs);

        let result = unit_test.run(self, |_| {}, |_| Ok(()));
        let code = match result {
            Ok(()) if should_panic => {
                println!(
                    "test {} ... {} (should have failed)",
                    name,
                    "error".bright_red()
                );
                UnitTestExitCode::Failed
            }
            Err(_) if should_panic => {
                println!("test {} ... {} (failed)", name, "ok".green());
                UnitTestExitCode::Passed
            }

            Ok(()) => {
                println!("test {} ... {}", name, "ok".green());
                UnitTestExitCode::Passed
            }
            Err(_) => {
                println!("test {} ... {}", name, "error".bright_red());
                UnitTestExitCode::Failed
            }
        };

        Ok(code)
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
