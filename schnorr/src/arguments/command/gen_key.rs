//!
//! The `generate key` command arguments.
//!

use structopt::StructOpt;

use franklin_crypto::bellman::pairing::bn256::Bn256;

use crate::arguments::command::IExecutable;
use crate::error::Error;

///
/// The `generate key` command arguments.
///
#[derive(StructOpt)]
#[structopt(name = "gen-key", about = "Generates a random private key")]
pub struct Command {}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<(), Self::Error> {
        let key = schnorr::generate_private_key::<Bn256>();
        let mut bytes = Vec::new();
        key.write(&mut bytes).expect("writing to Vec");
        let key_hex = hex::encode(bytes);
        println!("{}", key_hex);

        Ok(())
    }
}
