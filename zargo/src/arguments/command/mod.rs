//!
//! The Zargo project manager subcommand.
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

pub trait IExecutable {
    type Error;

    ///
    /// Executes the instance.
    ///
    fn execute(self) -> Result<(), Self::Error>;
}

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

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<(), Self::Error> {
        Ok(match self {
            Self::New(inner) => inner.execute()?,
            Self::Init(inner) => inner.execute()?,
            Self::Build(inner) => inner.execute()?,
            Self::Clean(inner) => inner.execute()?,
            Self::Run(inner) => inner.execute()?,
            Self::Test(inner) => inner.execute()?,
            Self::Setup(inner) => inner.execute()?,
            Self::Prove(inner) => inner.execute()?,
            Self::Verify(inner) => inner.execute()?,
            Self::ProofCheck(inner) => inner.execute()?,
        })
    }
}
