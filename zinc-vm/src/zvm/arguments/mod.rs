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
    /// The logging level value, which helps the logger to set the logging level.
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
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
