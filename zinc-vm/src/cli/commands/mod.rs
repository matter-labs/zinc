mod prove;
mod run;
mod setup;
mod verify;

use self::prove::ProveCommand;
use self::run::RunCommand;
use self::setup::SetupCommand;
use crate::commands::verify::VerifyCommand;
use structopt::StructOpt;
use crate::{Error, IoToError};

#[derive(Debug, StructOpt)]
#[structopt(name = "zvm", about = "Zinc Virtual Machine")]
pub struct Arguments {
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    pub verbosity: usize,

    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Run(RunCommand),
    Setup(SetupCommand),
    Prove(ProveCommand),
    Verify(VerifyCommand),
}

fn read_proof<R: std::io::Read>(reader: &mut R) -> Result<Vec<u8>, Error> {
    let mut hex_bytes = Vec::new();
    reader
        .read_to_end(&mut hex_bytes)
        .error_with_path(|| "<stdin>")?;

    let proof_hex: String = String::from_utf8_lossy(&hex_bytes).into();

    let bytes = hex::decode(proof_hex.trim())
        .map_err(|error| {
            Error::DecodingProof(error)
        })?;

    Ok(bytes)
}
