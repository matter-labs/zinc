mod exec;
mod prove;
mod setup;
mod verify;

use self::exec::ExecCommand;
use self::prove::ProveCommand;
use self::setup::SetupCommand;
use crate::commands::verify::VerifyCommand;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "zinc", about = "Zinc Virtual Machine")]
pub struct Arguments {
    #[structopt(short = "v", long = "verbose", about = "Shows verbose logs")]
    pub verbose: bool,

    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Exec(ExecCommand),
    Setup(SetupCommand),
    Prove(ProveCommand),
    Verify(VerifyCommand),
}

fn read_hex<R: std::io::Read>(reader: &mut R) -> std::io::Result<Vec<u8>> {
    let mut hex_bytes = Vec::new();
    reader.read_to_end(&mut hex_bytes).expect("failed to read from stdin");
    let proof_hex = String::from_utf8(hex_bytes).expect("invalid utf-8");
    Ok(hex::decode(proof_hex.trim()).expect("failed to decode hex proof"))
}
