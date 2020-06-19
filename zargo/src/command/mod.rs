//!
//! The command.
//!

pub mod build;
pub mod clean;
pub mod error;
pub mod init;
pub mod new;
pub mod proof_check;
pub mod prove;
pub mod run;
pub mod setup;
pub mod test;
pub mod verify;

use structopt::StructOpt;

use self::build::Command as BuildCommand;
use self::clean::Command as CleanCommand;
use self::error::Error;
use self::init::Command as InitCommand;
use self::new::Command as NewCommand;
use self::proof_check::Command as ProofCheckCommand;
use self::prove::Command as ProveCommand;
use self::run::Command as RunCommand;
use self::setup::Command as SetupCommand;
use self::test::Command as TestCommand;
use self::verify::Command as VerifyCommand;

#[derive(Debug, StructOpt)]
pub enum Command {
    New(NewCommand),
    Init(InitCommand),
    Build(BuildCommand),
    Clean(CleanCommand),
    Run(RunCommand),
    Test(TestCommand),
    Setup(SetupCommand),
    Prove(ProveCommand),
    Verify(VerifyCommand),
    ProofCheck(ProofCheckCommand),
}

impl Command {
    pub fn execute(self) -> Result<(), Error> {
        match self {
            Self::New(command) => command.execute()?,
            Self::Init(command) => command.execute()?,
            Self::Build(command) => command.execute()?,
            Self::Clean(command) => command.execute()?,
            Self::Run(command) => command.execute()?,
            Self::Test(command) => command.execute()?,
            Self::Setup(command) => command.execute()?,
            Self::Prove(command) => command.execute()?,
            Self::Verify(command) => command.execute()?,
            Self::ProofCheck(command) => command.execute()?,
        }
        Ok(())
    }
}
