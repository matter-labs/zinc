//!
//! The Zinc virtual machine arguments.
//!

pub mod command;

use structopt::StructOpt;

use self::command::Command;

///
/// The Zinc virtual machine arguments.
///
#[derive(Debug, StructOpt)]
#[structopt(name = zinc_const::app_name::ZINC_VIRTUAL_MACHINE, about = "The Zinc virtual machine")]
pub struct Arguments {
    /// Prints more logs, if passed several times.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbosity: usize,

    /// The subcommand variant.
    #[structopt(subcommand)]
    pub command: Command,
}

impl Arguments {
    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self::from_args()
    }
}
