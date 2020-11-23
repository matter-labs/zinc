//!
//! The virtual machine circuit facade.
//!

use std::marker::PhantomData;

use colored::Colorize;
use num::BigInt;

use franklin_crypto::bellman::groth16;
use franklin_crypto::bellman::groth16::Parameters;
use franklin_crypto::bellman::groth16::Proof;
use franklin_crypto::bellman::pairing::bn256::Bn256;
use franklin_crypto::circuit::test::TestConstraintSystem;

use zinc_const::UnitTestExitCode;

use crate::constraint_systems::main::Main as MainCS;
use crate::core::circuit::output::Output as CircuitOutput;
use crate::core::circuit::synthesizer::Synthesizer as CircuitSynthesizer;
use crate::core::circuit::State as CircuitState;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::IEngine;

pub struct Facade {
    inner: zinc_build::Circuit,
}

impl Facade {
    pub fn new(inner: zinc_build::Circuit) -> Self {
        Self { inner }
    }

    pub fn run<E: IEngine>(self, input: zinc_build::Value) -> Result<CircuitOutput, Error> {
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
        let output_value = zinc_build::Value::from_flat_values(output_type, &output_flat);

        Ok(CircuitOutput::new(output_value))
    }

    pub fn test<E: IEngine>(self) -> Result<UnitTestExitCode, Error> {
        let mut exit_code = UnitTestExitCode::Passed;

        for (name, unit_test) in self.inner.unit_tests.clone().into_iter() {
            if unit_test.is_ignored {
                println!("test {} ... {}", name, "ignore".yellow());
                return Ok(UnitTestExitCode::Ignored);
            }

            let cs = TestConstraintSystem::<Bn256>::new();

            let mut state = CircuitState::new(cs);

            let result = state.run(self.inner.clone(), Some(&[]), |_| {}, |_| Ok(()));
            match result {
                Err(_) if unit_test.should_panic => {
                    println!("test {} ... {} (failed)", name, "ok".green());
                }
                Ok(_) if unit_test.should_panic => {
                    println!(
                        "test {} ... {} (should have failed)",
                        name,
                        "error".bright_red()
                    );
                    exit_code = UnitTestExitCode::Failed;
                }

                Ok(_) => {
                    println!("test {} ... {}", name, "ok".green());
                }
                Err(error) => {
                    println!("test {} ... {} ({})", name, "error".bright_red(), error);
                    exit_code = UnitTestExitCode::Failed;
                }
            };
        }

        Ok(exit_code)
    }

    pub fn setup<E: IEngine>(self) -> Result<Parameters<E>, Error> {
        let rng = &mut rand::thread_rng();
        let mut result = None;

        let synthesizable = CircuitSynthesizer {
            inputs: None,
            output: &mut result,
            bytecode: self.inner,

            _pd: PhantomData,
        };

        let params = groth16::generate_random_parameters::<E, _, _>(synthesizable, rng)?;

        match result.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS) {
            Ok(_) => Ok(params),
            Err(error) => Err(error),
        }
    }

    pub fn prove<E: IEngine>(
        self,
        params: Parameters<E>,
        input: zinc_build::Value,
    ) -> Result<(zinc_build::Value, Proof<E>), Error> {
        let mut result = None;
        let rng = &mut rand::thread_rng();

        let inputs_flat = input.into_flat_values();
        let output_type = self.inner.output.clone();

        let synthesizable = CircuitSynthesizer {
            inputs: Some(inputs_flat),
            output: &mut result,
            bytecode: self.inner,

            _pd: PhantomData,
        };

        let proof = groth16::create_random_proof(synthesizable, &params, rng)
            .map_err(Error::SynthesisError)?;

        match result {
            None => Err(Error::InternalError(
                "circuit hasn't generate outputs".into(),
            )),
            Some(result) => match result {
                Ok(result) => {
                    let output_flat: Vec<BigInt> =
                        result.into_iter().filter_map(|value| value).collect();
                    let output_value =
                        zinc_build::Value::from_flat_values(output_type, &output_flat);

                    Ok((output_value, proof))
                }
                Err(err) => Err(err),
            },
        }
    }
}
