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
