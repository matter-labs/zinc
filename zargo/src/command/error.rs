//!
//! The Zargo command error.
//!

use failure::Fail;

use crate::command::BuildCommandError;
use crate::command::CleanCommandError;
use crate::command::ExecCommandError;
use crate::command::InitCommandError;
use crate::command::NewCommandError;
use crate::command::ProveCommandError;
use crate::command::SetupCommandError;
use crate::command::VerifyCommandError;

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
    Exec(ExecCommandError),
    #[fail(display = "{}", _0)]
    Setup(SetupCommandError),
    #[fail(display = "{}", _0)]
    Prove(ProveCommandError),
    #[fail(display = "{}", _0)]
    Verify(VerifyCommandError),
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

impl From<ExecCommandError> for Error {
    fn from(inner: ExecCommandError) -> Self {
        Self::Exec(inner)
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
