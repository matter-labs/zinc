//!
//! The command error.
//!

use failure::Fail;

use crate::command::build::Error as BuildCommandError;
use crate::command::clean::Error as CleanCommandError;
use crate::command::init::Error as InitCommandError;
use crate::command::new::Error as NewCommandError;
use crate::command::proof_check::Error as ProofCheckCommandError;
use crate::command::prove::Error as ProveCommandError;
use crate::command::run::Error as RunCommandError;
use crate::command::setup::Error as SetupCommandError;
use crate::command::verify::Error as VerifyCommandError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{}", _0)]
    New(NewCommandError),
    #[fail(display = "{}", _0)]
    Init(InitCommandError),
    #[fail(display = "{}", _0)]
    Build(BuildCommandError),
    #[fail(display = "{}", _0)]
    Clean(CleanCommandError),
    #[fail(display = "{}", _0)]
    Run(RunCommandError),
    #[fail(display = "{}", _0)]
    Setup(SetupCommandError),
    #[fail(display = "{}", _0)]
    Prove(ProveCommandError),
    #[fail(display = "{}", _0)]
    Verify(VerifyCommandError),
    #[fail(display = "{}", _0)]
    ProofCheck(ProofCheckCommandError),
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
