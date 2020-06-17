//!
//! The virtual machine circuit facade.
//!

use std::marker::PhantomData;

use num_bigint::BigInt;

use franklin_crypto::bellman::groth16;
use franklin_crypto::bellman::groth16::Parameters;
use franklin_crypto::bellman::groth16::Proof;
use franklin_crypto::bellman::pairing::bn256::Bn256;
use franklin_crypto::circuit::test::TestConstraintSystem;

use zinc_bytecode::Circuit as BytecodeCircuit;
use zinc_bytecode::TemplateValue;

use crate::constraint_systems::debug::DebugCS;
use crate::core::circuit::synthesizer::Synthesizer as CircuitSynthesizer;
use crate::core::circuit::Circuit;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::error::TypeSizeError;
use crate::facade::IFacade;
use crate::IEngine;

impl IFacade for BytecodeCircuit {
    fn debug<E: IEngine>(self, input: TemplateValue) -> Result<TemplateValue, RuntimeError> {
        let cs = TestConstraintSystem::<Bn256>::new();

        let inputs_flat = input.into_flat_values();
        let output_type = self.output.to_owned();

        let mut circuit = Circuit::new(cs, true);

        let mut num_constraints = 0;
        let result = circuit.run(
            self,
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

        let cs = circuit.constraint_system();

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

        let output_flat = result
            .into_iter()
            .map(|v| v.expect(crate::panic::VALUE_ALWAYS_EXISTS))
            .collect::<Vec<_>>();

        let value =
            TemplateValue::new_from_flat_values(output_type, &output_flat).ok_or_else(|| {
                TypeSizeError::Output {
                    expected: 0,
                    actual: 0,
                }
            })?;

        Ok(value)
    }

    fn run<E: IEngine>(self, input: TemplateValue) -> Result<TemplateValue, RuntimeError> {
        let cs = DebugCS::<Bn256>::default();

        let inputs_flat = input.into_flat_values();
        let output_type = self.output.to_owned();

        let mut circuit = Circuit::new(cs, true);

        let mut num_constraints = 0;
        let result = circuit.run(
            self,
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

        let cs = circuit.constraint_system();
        if !cs.is_satisfied() {
            return Err(RuntimeError::UnsatisfiedConstraint);
        }

        let output_flat = result
            .into_iter()
            .map(|v| v.expect(crate::panic::VALUE_ALWAYS_EXISTS))
            .collect::<Vec<_>>();

        let value =
            TemplateValue::new_from_flat_values(output_type, &output_flat).ok_or_else(|| {
                TypeSizeError::Output {
                    expected: 0,
                    actual: 0,
                }
            })?;

        Ok(value)
    }

    fn setup<E: IEngine>(self) -> Result<Parameters<E>, RuntimeError> {
        let rng = &mut rand::thread_rng();
        let mut result = None;

        let synthesizable = CircuitSynthesizer {
            inputs: None,
            output: &mut result,
            bytecode: self,

            _pd: PhantomData,
        };

        let params = groth16::generate_random_parameters::<E, _, _>(synthesizable, rng)?;

        match result.expect(crate::panic::VALUE_ALWAYS_EXISTS) {
            Ok(_) => Ok(params),
            Err(error) => Err(error),
        }
    }

    fn prove<E: IEngine>(
        self,
        params: Parameters<E>,
        witness: TemplateValue,
    ) -> Result<(TemplateValue, Proof<E>), RuntimeError> {
        let mut result = None;
        let rng = &mut rand::thread_rng();

        let witness_flat = witness.into_flat_values();
        let output_type = self.output.to_owned();

        let synthesizable = CircuitSynthesizer {
            inputs: Some(witness_flat),
            output: &mut result,
            bytecode: self,

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
                        .map(|v| v.expect(crate::panic::VALUE_ALWAYS_EXISTS))
                        .collect();

                    let value = TemplateValue::new_from_flat_values(output_type, &output_flat)
                        .ok_or_else(|| TypeSizeError::Output {
                            expected: 0,
                            actual: 0,
                        })?;

                    Ok((value, proof))
                }
                Err(err) => Err(err),
            },
        }
    }
}
