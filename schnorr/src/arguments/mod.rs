pub mod gen_key;
pub mod pub_key;
pub mod sign;

use failure::Fail;
use structopt::StructOpt;

use franklin_crypto::bellman::pairing::ff::PrimeField;
use franklin_crypto::bellman::pairing::ff::PrimeFieldRepr;

use self::gen_key::GenKeyCommand;
use self::pub_key::PubKeyCommand;
use self::sign::SignCommand;

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
}

pub fn fr_into_hex<Fr: PrimeField>(fr: Fr) -> String {
    let mut buffer = Vec::<u8>::new();

    fr.into_repr()
        .write_be(&mut buffer)
        .expect("failed to write into Vec<u8>");

    let num = num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &buffer);
    format!("0x{}", num.to_str_radix(16))
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "input-output error: {}", _0)]
    IO(std::io::Error),

    #[fail(display = "hex decoding error: {}", _0)]
    Hex(hex::FromHexError),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IO(error)
    }
}

impl From<hex::FromHexError> for Error {
    fn from(error: hex::FromHexError) -> Self {
        Self::Hex(error)
    }
}
