//!
//! The Zargo command.
//!

mod new;
mod error;

pub use self::error::Error;
pub use self::new::Command as NewCommand;
pub use self::new::Error as NewCommandError;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    New(NewCommand),
}

impl Command {
    pub fn execute(self) -> Result<(), Error> {
        Ok(match self {
            Self::New(command) => command.execute()?,
        })
    }
}
