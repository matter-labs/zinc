//!
//! The Zinc Schnorr signature tool arguments.
//!

pub mod gen_key;
pub mod pub_key;
pub mod sign;

use structopt::StructOpt;

use crate::error::Error;

use self::gen_key::Command as GenKeyCommand;
use self::pub_key::Command as PubKeyCommand;
use self::sign::Command as SignCommand;

pub trait IExecutable {
    type Error;

    ///
    /// Executes the instance.
    ///
    fn execute(self) -> Result<(), Self::Error>;
}

#[derive(StructOpt)]
pub enum Command {
    GenKey(GenKeyCommand),
    PubKey(PubKeyCommand),
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
