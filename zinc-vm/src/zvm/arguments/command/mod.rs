//!
//! The Zinc virtual machine subcommand.
//!

pub mod run;
pub mod test;

use structopt::StructOpt;

use crate::error::Error;

use self::run::Command as RunCommand;
use self::test::Command as TestCommand;

///
/// The generic trait used for commands.
///
pub trait IExecutable {
    /// The generic subcommand error type.
    type Error;

    ///
    /// Executes the instance.
    ///
    fn execute(self) -> Result<i32, Self::Error>;
}

///
/// The Zinc virtual machine subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "The Zinc virtual machine")]
pub enum Command {
    /// Executes the bytecode and prints its output.
    Run(RunCommand),
    /// Executes a unit test.
    Test(TestCommand),
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<i32, Self::Error> {
        match self {
            Command::Run(inner) => inner.execute(),
            Command::Test(inner) => inner.execute(),
        }
    }
}
