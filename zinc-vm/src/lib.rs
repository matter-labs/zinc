//!
//! The Zinc virtual machine library.
//!

#[cfg(test)]
mod tests;

pub(crate) mod constraint_systems;
pub(crate) mod core;
pub(crate) mod error;
pub mod facade;
pub mod gadgets;
pub(crate) mod instructions;
pub(crate) mod stdlib;

use std::fmt;

use lazy_static::lazy_static;

use franklin_crypto::alt_babyjubjub::AltJubjubBn256;
use franklin_crypto::alt_babyjubjub::JubjubEngine;
use pairing::bn256::Bn256;

pub use self::error::RuntimeError;
pub use self::facade::debug;
pub use self::facade::prove;
pub use self::facade::run;
pub use self::facade::setup;
pub use self::facade::verify;
pub use self::facade::VerificationError;

pub trait Engine: fmt::Debug + JubjubEngine {
    fn jubjub_params<'a>() -> &'a Self::Params;
}

lazy_static! {
    static ref JUBJUB_BN256_PARAMS: AltJubjubBn256 = AltJubjubBn256::new();
}

impl Engine for Bn256 {
    fn jubjub_params<'a>() -> &'a Self::Params {
        &JUBJUB_BN256_PARAMS
    }
}
