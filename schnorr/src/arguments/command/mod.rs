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
pub enum Command {
    /// The `generate key` subcommand.
    GenKey(GenKeyCommand),
    /// The `public key` subcommand.
    PubKey(PubKeyCommand),
    /// The `sign` subcommand.
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
