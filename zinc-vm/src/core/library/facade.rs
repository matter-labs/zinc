//!
//! The virtual machine library facade.
//!

use colored::Colorize;

use franklin_crypto::bellman::pairing::bn256::Bn256;

use zinc_const::UnitTestExitCode;

use crate::constraint_systems::main::Main as MainCS;
use crate::core::library::State as LibraryState;
use crate::error::Error;
use crate::IEngine;

pub struct Facade {
    inner: zinc_types::Library,
}

impl Facade {
    pub fn new(inner: zinc_types::Library) -> Self {
        Self { inner }
    }

    pub fn test<E: IEngine>(self) -> Result<UnitTestExitCode, Error> {
        let mut exit_code = UnitTestExitCode::Passed;

        for (name, unit_test) in self.inner.unit_tests.clone().into_iter() {
            if unit_test.is_ignored {
                log::info!("test {} ... {}", name, "ignore".yellow());
                return Ok(UnitTestExitCode::Ignored);
            }

            let cs = MainCS::<Bn256>::new();

            let mut state = LibraryState::new(cs);

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
