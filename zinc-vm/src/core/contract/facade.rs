//!
//! The virtual machine contract facade.
//!

use std::collections::HashMap;
use std::marker::PhantomData;

use colored::Colorize;
use num::BigInt;
use num::Zero;

use franklin_crypto::bellman::groth16;
use franklin_crypto::bellman::groth16::Parameters;
use franklin_crypto::bellman::groth16::Proof;
use franklin_crypto::bellman::pairing::bn256::Bn256;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::test::TestConstraintSystem;

use zinc_const::UnitTestExitCode;
use zinc_zksync::TransactionMsg;

use crate::constraint_systems::constant::Constant as ConstantCS;
use crate::core::contract::input::Input as ContractInput;
use crate::core::contract::output::Output as ContractOutput;
use crate::core::contract::storage::database::Storage as DatabaseStorage;
use crate::core::contract::storage::keeper::DummyKeeper;
use crate::core::contract::storage::keeper::IKeeper;
use crate::core::contract::storage::setup::Storage as SetupStorage;
use crate::core::contract::synthesizer::Synthesizer as ContractSynthesizer;
use crate::core::contract::State as ContractState;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::gadgets::contract::merkle_tree::hasher::sha256::Hasher as Sha256Hasher;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::gadgets::contract::storage::StorageGadget;
use crate::IEngine;

pub struct Facade {
    inner: zinc_build::Contract,
    keeper: Box<dyn IKeeper>,
}

impl Facade {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(inner: zinc_build::Contract) -> Self {
        Self {
            inner,
            keeper: Box::new(DummyKeeper::default()),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_with_keeper(inner: zinc_build::Contract, keeper: Box<dyn IKeeper>) -> Self {
        Self { inner, keeper }
    }

    pub fn run<E: IEngine>(self, input: ContractInput) -> Result<ContractOutput, Error> {
        let mut cs = ConstantCS {};

        let method = self
            .inner
            .methods
            .get(input.method_name.as_str())
            .cloned()
            .ok_or(Error::MethodNotFound {
                found: input.method_name.clone(),
            })?;

        let arguments_flat = input.arguments.into_flat_values();
        let output_type = if method.is_mutable {
            method.output.into_mutable_method_output()
        } else {
            method.output
        };

        let mut storages = HashMap::with_capacity(1);
        if method.name.as_str() != zinc_const::contract::CONSTRUCTOR_NAME {
            let storage =
                DatabaseStorage::<Bn256>::from_build(self.inner.storage.clone(), input.storage)?;
            let storage_gadget =
                StorageGadget::<_, _, Sha256Hasher>::new(cs.namespace(|| "storage"), storage)?;
            storages.insert(arguments_flat[0].to_owned(), storage_gadget);
        }

        let mut state = ContractState::new(
            cs,
            storages,
            self.keeper,
            input.method_name,
            input.transaction,
        );

        let mut num_constraints = 0;
        let result = state.run(
            self.inner,
            method.input,
            Some(&arguments_flat),
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
            method.address,
        )?;

        let cs = state.constraint_system();
        if !cs.is_satisfied() {
            return Err(Error::UnsatisfiedConstraint);
        }

        let output_value: Vec<BigInt> = result.into_iter().filter_map(|value| value).collect();
        let output_value = zinc_build::Value::from_flat_values(output_type, &output_value);

        let storages = state
            .storages
            .into_iter()
            .map(|(address, storage)| (address, storage.into_build()))
            .collect();

        let transfers = state.execution_state.transfers;

        Ok(ContractOutput::new(output_value, storages, transfers))
    }

    pub fn test<E: IEngine>(self) -> Result<UnitTestExitCode, Error> {
        let mut exit_code = UnitTestExitCode::Passed;

        for (name, unit_test) in self.inner.unit_tests.clone().into_iter() {
            if unit_test.is_ignored {
                println!("test {} ... {}", name, "ignore".yellow());
                return Ok(UnitTestExitCode::Ignored);
            }

            let mut cs = TestConstraintSystem::<Bn256>::new();

            let storage = SetupStorage::from_build(
                self.inner.storage.clone(),
                zinc_build::Value::Contract(vec![]),
            )?;
            let storage_gadget =
                StorageGadget::<_, _, Sha256Hasher>::new(cs.namespace(|| "storage"), storage)?;

            let mut storages = HashMap::with_capacity(1);
            storages.insert(BigInt::zero(), storage_gadget);

            let mut state = ContractState::new(
                cs,
                storages,
                Box::new(DummyKeeper::default()),
                name.clone(),
                TransactionMsg::default(),
            );

            let result = state.run(
                self.inner.clone(),
                zinc_build::Type::new_empty_structure(),
                Some(&[]),
                |_| {},
                |_| Ok(()),
                unit_test.address,
            );

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

    pub fn setup<E: IEngine>(self, method_name: String) -> Result<Parameters<E>, Error> {
        let rng = &mut rand::thread_rng();
        let mut result = None;

        let method = self
            .inner
            .methods
            .get(method_name.as_str())
            .cloned()
            .ok_or(Error::MethodNotFound {
                found: method_name.clone(),
            })?;

        let storage = SetupStorage::from_build(
            self.inner.storage.clone(),
            zinc_build::Value::Contract(vec![]),
        )?;

        let synthesizable = ContractSynthesizer {
            inputs: None,
            output: &mut result,
            bytecode: self.inner,
            method,
            storage,
            keeper: self.keeper,
            transaction: TransactionMsg::default(),

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
        input: ContractInput,
    ) -> Result<(zinc_build::Value, Proof<E>), Error> {
        let method = self
            .inner
            .methods
            .get(input.method_name.as_str())
            .cloned()
            .ok_or(Error::MethodNotFound {
                found: input.method_name.clone(),
            })?;

        let mut result = None;
        let rng = &mut rand::thread_rng();

        let arguments_flat = input.arguments.into_flat_values();
        let output_type = if method.is_mutable {
            method.output.clone().into_mutable_method_output()
        } else {
            method.output.clone()
        };

        let storage = DatabaseStorage::from_build(self.inner.storage.clone(), input.storage)?;

        let synthesizable = ContractSynthesizer {
            inputs: Some(arguments_flat),
            output: &mut result,
            bytecode: self.inner,
            method,
            storage,
            keeper: self.keeper,
            transaction: input.transaction,

            _pd: PhantomData,
        };

        let proof = groth16::create_random_proof(synthesizable, &params, rng)
            .map_err(Error::SynthesisError)?;

        match result {
            None => Err(Error::InternalError(
                "contract hasn't generate outputs".into(),
            )),
            Some(result) => match result {
                Ok(result) => {
                    let output_flat: Vec<BigInt> =
                        result.into_iter().filter_map(|value| value).collect();
                    let output_value =
                        zinc_build::Value::from_flat_values(output_type, &output_flat);

                    Ok((output_value, proof))
                }
                Err(error) => Err(error),
            },
        }
    }
}
