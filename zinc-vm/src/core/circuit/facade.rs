//!
//! The virtual machine circuit facade.
//!

use colored::Colorize;
use num::BigInt;

use franklin_crypto::bellman::pairing::bn256::Bn256;

use zinc_const::UnitTestExitCode;

use crate::constraint_systems::main::Main as MainCS;
use crate::core::circuit::output::Output as CircuitOutput;
use crate::core::circuit::State as CircuitState;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::IEngine;

pub struct Facade {
    inner: zinc_types::Circuit,
}

impl Facade {
    pub fn new(inner: zinc_types::Circuit) -> Self {
        Self { inner }
    }

    pub fn run<E: IEngine>(self, input: zinc_types::Value) -> Result<CircuitOutput, Error> {
        let cs = MainCS::<Bn256>::new();

        let inputs_flat = input.into_flat_values();
        let output_type = self.inner.output.clone();

        let mut state = CircuitState::new(cs);

        let mut num_constraints = 0;
        let result = state.run(
            self.inner,
            Some(&inputs_flat),
            |cs| {
                let num = cs.num_constraints() - num_constraints;
                num_constraints += num;
                log::trace!("Constraints: {}", num);
            },
            |cs| {
                if !cs.is_satisfied() {
                    return Err(Error::UnsatisfiedConstraint);
                }

                Ok(())
            },
        )?;

        let cs = state.constraint_system();
        if !cs.is_satisfied() {
            return Err(Error::UnsatisfiedConstraint);
        }

        let output_flat: Vec<BigInt> = result.into_iter().filter_map(|value| value).collect();
        let output_value = zinc_types::Value::from_flat_values(output_type, &output_flat);

        Ok(CircuitOutput::new(output_value))
    }

    pub fn test<E: IEngine>(self) -> Result<UnitTestExitCode, Error> {
        let mut exit_code = UnitTestExitCode::Passed;

        for (name, unit_test) in self.inner.unit_tests.clone().into_iter() {
            if unit_test.is_ignored {
                log::info!("test {} ... {}", name, "ignore".yellow());
                return Ok(UnitTestExitCode::Ignored);
            }

            let cs = MainCS::<Bn256>::new();

            let mut state = CircuitState::new(cs);

            match state.test(self.inner.clone(), unit_test.address) {
                Err(_) if unit_test.should_panic => {
                    log::info!("test {} ... {} (failed)", name, "ok".green());
                }
                Ok(_) if unit_test.should_panic => {
                    log::error!(
                        "test {} ... {} (should have failed)",
                        name,
                        "error".bright_red()
                    );
                    exit_code = UnitTestExitCode::Failed;
                }

                Ok(_) => {
                    log::info!("test {} ... {}", name, "ok".green());
                }
                Err(error) => {
                    log::error!("test {} ... {} ({})", name, "error".bright_red(), error);
                    exit_code = UnitTestExitCode::Failed;
                }
            };
        }

        Ok(exit_code)
    }
}
