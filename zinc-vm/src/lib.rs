mod core;
mod errors;
mod gadgets;
mod instructions;

#[cfg(test)]
mod tests;

mod facade;
pub use facade::*;

use franklin_crypto::babyjubjub::{JubjubBn256, JubjubEngine};
use lazy_static::lazy_static;
use pairing::bn256::Bn256;
use std::fmt::Debug;

pub trait Engine: JubjubEngine + Debug {
    fn jubjub_params<'a>() -> &'a Self::Params;
}
lazy_static! {
    static ref JUBJUB_BN256_PARAMS: JubjubBn256 = JubjubBn256::new();
}

impl Engine for Bn256 {
    fn jubjub_params<'a>() -> &'a Self::Params {
        &JUBJUB_BN256_PARAMS
    }
}
