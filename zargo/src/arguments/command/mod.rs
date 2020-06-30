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

///
/// The generic trait used for commands.
///
pub trait IExecutable {
    /// The generic subcommand error type.
    type Error;

    ///
    /// Executes the instance.
    ///
    fn execute(self) -> Result<(), Self::Error>;
}

///
/// The Zargo project manager subcommand.
///
#[derive(Debug, StructOpt)]
pub enum Command {
    /// The `new` subcommand.
    New(NewCommand),
    /// The `init` subcommand.
    Init(InitCommand),
    /// The `build` subcommand.
    Build(BuildCommand),
    /// The `clean` subcommand.
    Clean(CleanCommand),
    /// The `run` subcommand.
    Run(RunCommand),
    /// The `test` subcommand.
    Test(TestCommand),
    /// The `setup` subcommand.
    Setup(SetupCommand),
    /// The `prove` subcommand.
    Prove(ProveCommand),
    /// The `verify` subcommand.
    Verify(VerifyCommand),
    /// The `proof-check` subcommand.
    ProofCheck(ProofCheckCommand),
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<(), Self::Error> {
        match self {
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
        }

        Ok(())
    }
}
