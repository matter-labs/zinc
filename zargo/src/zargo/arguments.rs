//!
//! The Zargo package manager arguments.
//!

use structopt::StructOpt;

use zargo::Command;

///
/// The Zargo package manager arguments.
///
#[derive(Debug, StructOpt)]
#[structopt(
    name = zinc_const::app_name::ZARGO,
    about = "The Zinc package manager",
)]
pub struct Arguments {
    /// Prints more logs, if passed several times.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbosity: usize,

    /// Suppresses output, if set.
    #[structopt(short = "q", long = "quiet")]
    pub quiet: bool,

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
