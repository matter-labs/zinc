//!
//! The virtual machine contract facade.
//!

use std::collections::HashMap;

use colored::Colorize;
use num::BigInt;

use franklin_crypto::bellman::pairing::bn256::Bn256;
use franklin_crypto::bellman::ConstraintSystem;

use zinc_const::UnitTestExitCode;

use crate::constraint_systems::constant::Constant as ConstantCS;
use crate::constraint_systems::main::Main as MainCS;
use crate::core::contract::input::Input as ContractInput;
use crate::core::contract::output::Output as ContractOutput;
use crate::core::contract::storage::database::Storage as DatabaseStorage;
use crate::core::contract::storage::keeper::DummyKeeper;
use crate::core::contract::storage::keeper::IKeeper;
use crate::core::contract::State as ContractState;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::gadgets::contract::merkle_tree::hasher::sha256::Hasher as Sha256Hasher;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::gadgets::contract::storage::StorageGadget;
use crate::IEngine;

pub struct Facade {
    inner: zinc_types::Contract,
    keeper: Box<dyn IKeeper>,
}

impl Facade {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(inner: zinc_types::Contract) -> Self {
        Self {
            inner,
            keeper: Box::new(DummyKeeper::default()),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_with_keeper(inner: zinc_types::Contract, keeper: Box<dyn IKeeper>) -> Self {
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
        } else if method.name.as_str() == zinc_const::contract::CONSTRUCTOR_IDENTIFIER {
            zinc_types::Type::eth_address()
        } else {
            method.output
        };

        let mut storages = HashMap::with_capacity(1);
        if method.name.as_str() != zinc_const::contract::CONSTRUCTOR_IDENTIFIER {
            for (address, storage) in input.storages.into_iter() {
                let address = BigInt::from_bytes_be(num::bigint::Sign::Plus, address.as_bytes());
                let storage =
                    DatabaseStorage::<Bn256>::from_build(self.inner.storage.clone(), storage)?;
                let storage_gadget =
                    StorageGadget::<_, _, Sha256Hasher>::new(cs.namespace(|| "storage"), storage)?;
                storages.insert(address, storage_gadget);
            }
        }

        let mut state = ContractState::new(cs, storages, self.keeper, input.transaction);

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
        let output_value = zinc_types::Value::from_flat_values(output_type, &output_value);

        let storages = state
            .storages
            .into_iter()
            .map(|(address, storage)| (address, storage.into_build()))
            .collect();

        let transfers = state.execution_state.transfers;
        let initializers = state.execution_state.initializers;

        Ok(ContractOutput::new(
            output_value,
            storages,
            transfers,
            initializers,
        ))
    }

    pub fn test<E: IEngine>(self) -> Result<UnitTestExitCode, Error> {
        let mut exit_code = UnitTestExitCode::Passed;

        for (name, unit_test) in self.inner.unit_tests.clone().into_iter() {
            if unit_test.is_ignored {
                log::info!("test {} ... {}", name, "ignore".yellow());
                return Ok(UnitTestExitCode::Ignored);
            }

            let cs = MainCS::<Bn256>::new();

            let mut state = ContractState::<_, _, DatabaseStorage<_>, Sha256Hasher>::new(
                cs,
                HashMap::with_capacity(1),
                Box::new(DummyKeeper::default()),
                unit_test.zksync_msg.unwrap_or_default(),
            );

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
