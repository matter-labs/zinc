mod gen_key;
mod pub_key;
mod sign;

pub use gen_key::*;
pub use pub_key::*;
pub use sign::*;

use structopt::StructOpt;
use franklin_crypto::bellman::pairing::ff::{PrimeField, PrimeFieldRepr};

#[derive(StructOpt)]
#[structopt(
    name = "schnorr",
    about = "schnorr signature: create keys, sign and verify"
)]
pub struct Arguments {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt)]
pub enum Command {
    GenKey(GenKeyCommand),
    PubKey(PubKeyCommand),
    Sign(SignCommand),
    // Verify(commands::VerifyCommand),
}

pub fn fr_into_hex<Fr: PrimeField>(fr: Fr) -> String {
    let mut buffer = Vec::<u8>::new();

    fr.into_repr()
        .write_be(&mut buffer)
        .expect("failed to write into Vec<u8>");

    let num = num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &buffer);
    format!("0x{}", num.to_str_radix(16))
}
