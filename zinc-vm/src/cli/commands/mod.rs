mod exec;
mod setup;
mod prove;
mod verify;

use structopt::StructOpt;
use self::exec::ExecCommand;
use self::prove::ProveCommand;
use self::setup::SetupCommand;
use crate::commands::verify::VerifyCommand;

#[derive(Debug, StructOpt)]
#[structopt(name = "zinc", about = "Zinc Virtual Machine")]
pub struct Arguments {
    #[structopt(short = "v", long = "verbose", about = "Shows verbose logs")]
    pub verbose: bool,

    #[structopt(subcommand)]
    pub command: Command
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Exec(ExecCommand),
    Setup(SetupCommand),
    Prove(ProveCommand),
    Verify(VerifyCommand),
}
