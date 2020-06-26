//!
//! The `generate key` command arguments.
//!

use structopt::StructOpt;

use franklin_crypto::bellman::pairing::bn256::Bn256;

use crate::arguments::Error;

///
/// The `generate key` command arguments.
///
#[derive(StructOpt)]
#[structopt(name = "gen-key", about = "generate a random private key")]
pub struct GenKeyCommand {}

impl GenKeyCommand {
    pub fn execute(&self) -> Result<(), Error> {
        let key = schnorr::generate_private_key::<Bn256>();
        let mut bytes = Vec::new();
        key.write(&mut bytes).expect("writing to Vec");
        let key_hex = hex::encode(bytes);
        println!("{}", key_hex);

        Ok(())
    }
}
