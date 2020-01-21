mod gadgets;
mod instructions;
mod vm;

mod facade;
pub use facade::*;

use franklin_crypto::babyjubjub::{JubjubEngine, JubjubBn256};
use std::fmt::Debug;
use pairing::bn256::Bn256;
use lazy_static::lazy_static;

pub trait ZincEngine: JubjubEngine + Debug {
    fn jubjub_params<'a>() -> &'a Self::Params;
}
lazy_static! {
    static ref JUBJUB_BN256_PARAMS: JubjubBn256 = JubjubBn256::new();
}

impl ZincEngine for Bn256 {
    fn jubjub_params<'a>() -> &'a Self::Params {
        &JUBJUB_BN256_PARAMS
    }
}
