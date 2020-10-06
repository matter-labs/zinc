//!
//! The Zinc Schnorr signature tool subcommand.
//!

pub mod gen_key;
pub mod pub_key;
pub mod sign;

use structopt::StructOpt;

use crate::error::Error;

use self::gen_key::Command as GenKeyCommand;
use self::pub_key::Command as PubKeyCommand;
use self::sign::Command as SignCommand;

///
/// The generic trait used for commands.
///
pub trait IExecutable {
    /// The generic subcommand error type.
    type Error;

    ///
    /// Executes the instance.
    ///
    fn execute(self) -> Result<(), Self::Error>;
}

///
/// The Zinc Schnorr signature tool subcommand.
///
#[derive(StructOpt)]
#[structopt(about = "Schnorr signature tool: creates keys, signs and verifies messages")]
pub enum Command {
    /// Generates a random private key.
    GenKey(GenKeyCommand),
    /// Recovers the public key from the private key.
    PubKey(PubKeyCommand),
    /// Generates a signature.
    Sign(SignCommand),
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<(), Self::Error> {
        match self {
            Self::GenKey(inner) => inner.execute(),
            Self::PubKey(inner) => inner.execute(),
            Self::Sign(inner) => inner.execute(),
        }
    }
}
