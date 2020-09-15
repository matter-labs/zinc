//!
//! The Zinc virtual machine library.
//!

//#![warn(missing_docs)]
//#![warn(clippy::missing_docs_in_private_items)]

mod tests;

pub(crate) mod constraint_systems;
pub(crate) mod core;
pub(crate) mod error;
pub mod gadgets;
pub(crate) mod instructions;

pub use franklin_crypto::bellman::pairing::bn256::Bn256;

pub use self::core::circuit::facade::Facade as CircuitFacade;
pub use self::core::circuit::output::Output as CircuitOutput;
pub use self::core::contract::facade::Facade as ContractFacade;
pub use self::core::contract::output::Output as ContractOutput;
pub use self::core::facade::Facade;
pub use self::error::RuntimeError;
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
