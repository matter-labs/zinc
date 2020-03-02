mod prove;
mod run;
mod setup;
mod verify;
mod debug;

use self::prove::ProveCommand;
use self::run::RunCommand;
use self::debug::DebugCommand;
use self::setup::SetupCommand;
use self::verify::VerifyCommand;
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
    Debug(DebugCommand),
    Setup(SetupCommand),
    Prove(ProveCommand),
    Verify(VerifyCommand),
}
