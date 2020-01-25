//!
//! The Zargo command.
//!

mod build;
mod clean;
mod error;
mod exec;
mod init;
mod new;
mod prove;
mod setup;
mod verify;

pub use self::build::Command as BuildCommand;
pub use self::build::Error as BuildCommandError;
pub use self::clean::Command as CleanCommand;
pub use self::clean::Error as CleanCommandError;
pub use self::error::Error;
pub use self::exec::Command as ExecCommand;
pub use self::exec::Error as ExecCommandError;
pub use self::init::Command as InitCommand;
pub use self::init::Error as InitCommandError;
pub use self::new::Command as NewCommand;
pub use self::new::Error as NewCommandError;
pub use self::prove::Command as ProveCommand;
pub use self::prove::Error as ProveCommandError;
pub use self::setup::Command as SetupCommand;
pub use self::setup::Error as SetupCommandError;
pub use self::verify::Command as VerifyCommand;
pub use self::verify::Error as VerifyCommandError;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    New(NewCommand),
    Init(InitCommand),
    Build(BuildCommand),
    Clean(CleanCommand),
    Run(ExecCommand),
    Setup(SetupCommand),
    Prove(ProveCommand),
    Verify(VerifyCommand),
}

impl Command {
    pub fn execute(self) -> Result<(), Error> {
        match self {
            Self::New(command) => command.execute()?,
            Self::Init(command) => command.execute()?,
            Self::Build(command) => command.execute()?,
            Self::Clean(command) => command.execute()?,
            Self::Run(command) => command.execute()?,
            Self::Setup(command) => command.execute()?,
            Self::Prove(command) => command.execute()?,
            Self::Verify(command) => command.execute()?,
        }
        Ok(())
    }
}
