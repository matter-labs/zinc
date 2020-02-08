mod prove;
mod run;
mod setup;
mod verify;

use self::prove::ProveCommand;
use self::run::RunCommand;
use self::setup::SetupCommand;
use crate::commands::verify::VerifyCommand;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "zvm", about = "Zinc Virtual Machine")]
pub struct Arguments {
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    pub verbose: usize,

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

fn read_hex<R: std::io::Read>(reader: &mut R) -> std::io::Result<Vec<u8>> {
    let mut hex_bytes = Vec::new();
    reader
        .read_to_end(&mut hex_bytes)
        .expect("failed to read from stdin");
    let proof_hex = String::from_utf8(hex_bytes).expect("invalid utf-8");
    Ok(hex::decode(proof_hex.trim()).expect("failed to decode hex proof"))
}
