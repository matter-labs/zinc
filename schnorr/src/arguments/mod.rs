//!
//! The Zinc Schnorr signature tool arguments.
//!

pub mod command;

use structopt::StructOpt;

use self::command::Command;

///
/// The Zinc Schnorr signature tool arguments.
///
#[derive(StructOpt)]
#[structopt(
    name = zinc_const::app_name::SCHNORR,
    about = "Schnorr signature tool: creates keys, signs and verifies messages",
)]
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
