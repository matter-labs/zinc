//!
//! The Zinc virtual machine library.
//!

#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]

#[cfg(test)]
mod tests;

pub(crate) mod constraint_systems;
pub(crate) mod core;
pub(crate) mod error;
pub(crate) mod gadgets;
pub(crate) mod instructions;

pub use franklin_crypto::bellman::pairing::bn256::Bn256;

pub use self::core::circuit::facade::Facade as CircuitFacade;
pub use self::core::circuit::output::Output as CircuitOutput;
pub use self::core::contract::facade::Facade as ContractFacade;
pub use self::core::contract::input::Input as ContractInput;
pub use self::core::contract::output::initializer::Initializer as ContractOutputInitializer;
pub use self::core::contract::output::Output as ContractOutput;
pub use self::core::contract::storage::keeper::IKeeper as IContractStorageKeeper;
pub use self::core::facade::Facade;
pub use self::core::library::facade::Facade as LibraryFacade;
pub use self::error::Error;
pub use self::error::VerificationError;

use std::fmt;

use lazy_static::lazy_static;

use franklin_crypto::alt_babyjubjub::AltJubjubBn256;
use franklin_crypto::alt_babyjubjub::JubjubEngine;

pub trait IEngine: fmt::Debug + JubjubEngine {
    fn jubjub_params<'a>() -> &'a Self::Params;
}

lazy_static! {
    static ref JUBJUB_BN256_PARAMS: AltJubjubBn256 = AltJubjubBn256::new();
}

impl IEngine for Bn256 {
    fn jubjub_params<'a>() -> &'a Self::Params {
        &JUBJUB_BN256_PARAMS
    }
}
