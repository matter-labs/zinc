mod gen_key;
mod pub_key;
mod sign;

pub use gen_key::*;
pub use pub_key::*;
pub use sign::*;

use structopt::StructOpt;

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
