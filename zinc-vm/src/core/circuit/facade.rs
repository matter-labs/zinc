//!
//! The virtual machine circuit facade.
//!

use std::marker::PhantomData;

use colored::Colorize;
use num_bigint::BigInt;

use franklin_crypto::bellman::groth16;
use franklin_crypto::bellman::groth16::Parameters;
use franklin_crypto::bellman::groth16::Proof;
use franklin_crypto::bellman::pairing::bn256::Bn256;
use franklin_crypto::circuit::test::TestConstraintSystem;

use zinc_build::Circuit as BuildCircuit;
use zinc_build::Value as BuildValue;
use zinc_const::UnitTestExitCode;

use crate::constraint_systems::main::Main as MainCS;
use crate::core::circuit::output::Output as CircuitOutput;
use crate::core::circuit::synthesizer::Synthesizer as CircuitSynthesizer;
use crate::core::circuit::State as CircuitState;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::IEngine;

pub struct Facade {
    inner: BuildCircuit,
}

impl Facade {
    pub fn new(inner: BuildCircuit) -> Self {
        Self { inner }
    }

    pub fn run<E: IEngine>(self, input: BuildValue) -> Result<CircuitOutput, RuntimeError> {
        let cs = MainCS::<Bn256>::new();

        let inputs_flat = input.into_flat_values();
        let output_type = self.inner.output.clone();

        let mut state = CircuitState::new(cs, false);

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
                    return Err(RuntimeError::UnsatisfiedConstraint);
                }

                Ok(())
            },
        )?;

        let cs = state.constraint_system();
        if !cs.is_satisfied() {
            return Err(RuntimeError::UnsatisfiedConstraint);
        }

        let output_flat: Vec<BigInt> = result.into_iter().filter_map(|value| value).collect();
        let output_value = BuildValue::from_flat_values(output_type, &output_flat);

        Ok(CircuitOutput::new(output_value))
    }

    pub fn debug<E: IEngine>(self, input: BuildValue) -> Result<CircuitOutput, RuntimeError> {
        let cs = TestConstraintSystem::<Bn256>::new();

        let inputs_flat = input.into_flat_values();
        let output_type = self.inner.output.clone();

        let mut state = CircuitState::new(cs, true);

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
                    return Err(RuntimeError::UnsatisfiedConstraint);
                }

                Ok(())
            },
        )?;

        let cs = state.constraint_system();
        if !cs.is_satisfied() {
            log::trace!("{}", cs.pretty_print());
            log::error!(
                "Unsatisfied: {}",
                cs.which_is_unsatisfied()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
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

        let output_flat: Vec<BigInt> = result.into_iter().filter_map(|value| value).collect();
        let output_value = BuildValue::from_flat_values(output_type, &output_flat);

        Ok(CircuitOutput::new(output_value))
    }

    pub fn test<E: IEngine>(self) -> Result<UnitTestExitCode, RuntimeError> {
        let mut exit_code = UnitTestExitCode::Passed;

        for (name, unit_test) in self.inner.unit_tests.clone().into_iter() {
            if unit_test.is_ignored {
                println!("test {} ... {}", name, "ignore".yellow());
                return Ok(UnitTestExitCode::Ignored);
            }

            let cs = TestConstraintSystem::<Bn256>::new();

            let mut state = CircuitState::new(cs, true);

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
                Err(_) => {
                    println!("test {} ... {}", name, "error".bright_red());
                    exit_code = UnitTestExitCode::Failed;
                }
            };
        }

        Ok(exit_code)
    }

    pub fn setup<E: IEngine>(self) -> Result<Parameters<E>, RuntimeError> {
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
        input: BuildValue,
    ) -> Result<(BuildValue, Proof<E>), RuntimeError> {
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
            .map_err(RuntimeError::SynthesisError)?;

        match result {
            None => Err(RuntimeError::InternalError(
                "circuit hasn't generate outputs".into(),
            )),
            Some(result) => match result {
                Ok(result) => {
                    let output_flat: Vec<BigInt> =
                        result.into_iter().filter_map(|value| value).collect();
                    let output_value = BuildValue::from_flat_values(output_type, &output_flat);

                    Ok((output_value, proof))
                }
                Err(err) => Err(err),
            },
        }
    }
}
