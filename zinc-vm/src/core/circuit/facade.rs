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

use zinc_bytecode::Circuit as BytecodeCircuit;
use zinc_bytecode::TemplateValue;
use zinc_const::UnitTestExitCode;

use crate::constraint_systems::debug::DebugCS;
use crate::core::circuit::synthesizer::Synthesizer as CircuitSynthesizer;
use crate::core::circuit::State as CircuitState;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::IEngine;

pub struct Facade {
    inner: BytecodeCircuit,
}

impl Facade {
    pub fn new(inner: BytecodeCircuit) -> Self {
        Self { inner }
    }

    pub fn run<E: IEngine>(self, input: TemplateValue) -> Result<TemplateValue, RuntimeError> {
        let cs = DebugCS::<Bn256>::default();

        let inputs_flat = input.into_flat_values();
        let output_type = self.inner.output.to_owned();

        let mut state = CircuitState::new(cs, true);

        let mut num_constraints = 0;
        let result = state.run(
            self.inner,
            Some(&inputs_flat),
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

        let cs = state.constraint_system();
        if !cs.is_satisfied() {
            return Err(RuntimeError::UnsatisfiedConstraint);
        }

        let output_flat = result
            .into_iter()
            .map(|v| v.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS))
            .collect::<Vec<_>>();

        let value = TemplateValue::from_flat_values(output_type, &output_flat);

        Ok(value)
    }

    pub fn debug<E: IEngine>(self, input: TemplateValue) -> Result<TemplateValue, RuntimeError> {
        let cs = TestConstraintSystem::<Bn256>::new();

        let inputs_flat = input.into_flat_values();
        let output_type = self.inner.output.to_owned();

        let mut state = CircuitState::new(cs, true);

        let mut num_constraints = 0;
        let result = state.run(
            self.inner,
            Some(&inputs_flat),
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

        let cs = state.constraint_system();

        log::debug!("{}", cs.pretty_print());

        if !cs.is_satisfied() {
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

        let output_flat = result
            .into_iter()
            .map(|v| v.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS))
            .collect::<Vec<_>>();

        let value = TemplateValue::from_flat_values(output_type, &output_flat);

        Ok(value)
    }

    pub fn test<E: IEngine>(mut self) -> Result<UnitTestExitCode, RuntimeError> {
        let unit_test = self
            .inner
            .unit_test
            .take()
            .ok_or(RuntimeError::UnitTestDataMissing)?;

        if unit_test.is_ignored {
            println!("test {} ... {}", unit_test.name, "ignore".yellow());
            return Ok(UnitTestExitCode::Ignored);
        }

        let cs = TestConstraintSystem::<Bn256>::new();
        let mut state = CircuitState::new(cs, true);

        let result = state.run(self.inner, Some(&[]), |_| {}, |_| Ok(()));
        let code = match result {
            Ok(_) if unit_test.should_panic => {
                println!(
                    "test {} ... {} (should have failed)",
                    unit_test.name,
                    "error".bright_red()
                );
                UnitTestExitCode::Failed
            }
            Err(_) if unit_test.should_panic => {
                println!("test {} ... {} (failed)", unit_test.name, "ok".green());
                UnitTestExitCode::Passed
            }

            Ok(_) => {
                println!("test {} ... {}", unit_test.name, "ok".green());
                UnitTestExitCode::Passed
            }
            Err(_) => {
                println!("test {} ... {}", unit_test.name, "error".bright_red());
                UnitTestExitCode::Failed
            }
        };

        Ok(code)
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
        witness: TemplateValue,
    ) -> Result<(TemplateValue, Proof<E>), RuntimeError> {
        let mut result = None;
        let rng = &mut rand::thread_rng();

        let inputs_flat = witness.into_flat_values();
        let output_type = self.inner.output.to_owned();

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
                Ok(values) => {
                    let output_flat: Vec<BigInt> = values
                        .into_iter()
                        .map(|v| v.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS))
                        .collect();

                    let value = TemplateValue::from_flat_values(output_type, &output_flat);

                    Ok((value, proof))
                }
                Err(err) => Err(err),
            },
        }
    }
}
