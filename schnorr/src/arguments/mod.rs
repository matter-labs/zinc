//!
//! The Zinc Schnorr signature tool arguments.
//!

pub mod gen_key;
pub mod pub_key;
pub mod sign;

use failure::Fail;
use structopt::StructOpt;

use self::gen_key::GenKeyCommand;
use self::pub_key::PubKeyCommand;
use self::sign::SignCommand;

///
/// The Zinc Schnorr signature tool arguments.
///
#[derive(StructOpt)]
#[structopt(
    name = "schnorr",
    about = "schnorr signature: create keys, sign and verify"
)]
pub struct Arguments {
    #[structopt(subcommand)]
    pub command: Command,
}

impl Arguments {
    pub fn new() -> Self {
        Self::from_args()
    }
}

#[derive(StructOpt)]
pub enum Command {
    GenKey(GenKeyCommand),
    PubKey(PubKeyCommand),
    Sign(SignCommand),
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
