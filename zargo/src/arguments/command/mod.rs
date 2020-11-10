//!
//! The Zargo package manager subcommand.
//!

pub mod build;
pub mod call;
pub mod clean;
pub mod init;
pub mod new;
pub mod proof_check;
pub mod prove;
pub mod publish;
pub mod query;
pub mod run;
pub mod setup;
pub mod test;
pub mod verify;

use structopt::StructOpt;

use self::build::Command as BuildCommand;
use self::call::Command as CallCommand;
use self::clean::Command as CleanCommand;
use self::init::Command as InitCommand;
use self::new::Command as NewCommand;
use self::proof_check::Command as ProofCheckCommand;
use self::prove::Command as ProveCommand;
use self::publish::Command as PublishCommand;
use self::query::Command as QueryCommand;
use self::run::Command as RunCommand;
use self::setup::Command as SetupCommand;
use self::test::Command as TestCommand;
use self::verify::Command as VerifyCommand;

///
/// The Zargo package manager subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "The Zinc package manager")]
pub enum Command {
    /// Creates a new project in the specified directory.
    New(NewCommand),
    /// Initializes a new project in the specified directory.
    Init(InitCommand),
    /// Builds the project at the given path.
    Build(BuildCommand),
    /// Removes the project build artifacts.
    Clean(CleanCommand),
    /// Runs the project and prints its output.
    Run(RunCommand),
    /// Runs the project unit tests.
    Test(TestCommand),
    /// Generates a pair of proving and verifying keys.
    Setup(SetupCommand),
    /// Generates the zero-knowledge proof for given input data.
    Prove(ProveCommand),
    /// Verifies the zero-knowledge proof.
    Verify(VerifyCommand),
    /// Runs the full project building, running, trusted setup, proving & verifying sequence.
    ProofCheck(ProofCheckCommand),
    /// Uploads the smart contract to the specified network.
    Publish(PublishCommand),
    /// Queries a contract storage or calls an immutable method.
    Query(QueryCommand),
    /// Calls a mutable smart contract method.
    Call(CallCommand),
}

impl Command {
    ///
    /// Executes the command.
    ///
    pub async fn execute(self) -> anyhow::Result<()> {
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
            Self::Publish(inner) => inner.execute().await?,
            Self::Query(inner) => inner.execute().await?,
            Self::Call(inner) => inner.execute().await?,
        }

        Ok(())
    }
}
