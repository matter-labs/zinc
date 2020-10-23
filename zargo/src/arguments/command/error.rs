//!
//! The Zargo package manager subcommand error.
//!

use failure::Fail;

use crate::arguments::command::build::error::Error as BuildCommandError;
use crate::arguments::command::call::error::Error as CallCommandError;
use crate::arguments::command::clean::error::Error as CleanCommandError;
use crate::arguments::command::init::error::Error as InitCommandError;
use crate::arguments::command::new::error::Error as NewCommandError;
use crate::arguments::command::proof_check::error::Error as ProofCheckCommandError;
use crate::arguments::command::prove::error::Error as ProveCommandError;
use crate::arguments::command::publish::error::Error as PublishCommandError;
use crate::arguments::command::query::error::Error as QueryCommandError;
use crate::arguments::command::run::error::Error as RunCommandError;
use crate::arguments::command::setup::error::Error as SetupCommandError;
use crate::arguments::command::test::error::Error as TestCommandError;
use crate::arguments::command::verify::error::Error as VerifyCommandError;

///
/// The Zargo package manager error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The `new` command error.
    #[fail(display = "{}", _0)]
    New(NewCommandError),
    /// The `init` command error.
    #[fail(display = "{}", _0)]
    Init(InitCommandError),
    /// The `build` command error.
    #[fail(display = "{}", _0)]
    Build(BuildCommandError),
    /// The `clean` command error.
    #[fail(display = "{}", _0)]
    Clean(CleanCommandError),
    /// The `run` command error.
    #[fail(display = "{}", _0)]
    Run(RunCommandError),
    /// The `test` command error.
    #[fail(display = "{}", _0)]
    Test(TestCommandError),
    /// The `setup` command error.
    #[fail(display = "{}", _0)]
    Setup(SetupCommandError),
    /// The `prove` command error.
    #[fail(display = "{}", _0)]
    Prove(ProveCommandError),
    /// The `verify` command error.
    #[fail(display = "{}", _0)]
    Verify(VerifyCommandError),
    /// The `proof-check` command error.
    #[fail(display = "{}", _0)]
    ProofCheck(ProofCheckCommandError),
    /// The `publish` command error.
    #[fail(display = "{}", _0)]
    Publish(PublishCommandError),
    /// The `query` command error.
    #[fail(display = "{}", _0)]
    Query(QueryCommandError),
    /// The `call` command error.
    #[fail(display = "{}", _0)]
    Call(CallCommandError),
}

impl From<NewCommandError> for Error {
    fn from(inner: NewCommandError) -> Self {
        Self::New(inner)
    }
}

impl From<InitCommandError> for Error {
    fn from(inner: InitCommandError) -> Self {
        Self::Init(inner)
    }
}

impl From<BuildCommandError> for Error {
    fn from(inner: BuildCommandError) -> Self {
        Self::Build(inner)
    }
}

impl From<CleanCommandError> for Error {
    fn from(inner: CleanCommandError) -> Self {
        Self::Clean(inner)
    }
}

impl From<RunCommandError> for Error {
    fn from(inner: RunCommandError) -> Self {
        Self::Run(inner)
    }
}

impl From<TestCommandError> for Error {
    fn from(inner: TestCommandError) -> Self {
        Self::Test(inner)
    }
}

impl From<SetupCommandError> for Error {
    fn from(inner: SetupCommandError) -> Self {
        Self::Setup(inner)
    }
}

impl From<ProveCommandError> for Error {
    fn from(inner: ProveCommandError) -> Self {
        Self::Prove(inner)
    }
}

impl From<VerifyCommandError> for Error {
    fn from(inner: VerifyCommandError) -> Self {
        Self::Verify(inner)
    }
}

impl From<ProofCheckCommandError> for Error {
    fn from(inner: ProofCheckCommandError) -> Self {
        Self::ProofCheck(inner)
    }
}

impl From<PublishCommandError> for Error {
    fn from(inner: PublishCommandError) -> Self {
        Self::Publish(inner)
    }
}

impl From<QueryCommandError> for Error {
    fn from(inner: QueryCommandError) -> Self {
        Self::Query(inner)
    }
}

impl From<CallCommandError> for Error {
    fn from(inner: CallCommandError) -> Self {
        Self::Call(inner)
    }
}
