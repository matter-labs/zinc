//!
//! The Zinc tester arguments.
//!

use structopt::StructOpt;

///
/// The Zinc tester arguments.
///
#[derive(Debug, StructOpt)]
#[structopt(
    name = zinc_const::app_name::ZINC_TESTER,
    about = "The integration test runner for the Zinc framework"
)]
pub struct Arguments {
    /// The logging level value, which helps the logger to set the logging level.
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity."
    )]
    pub verbosity: usize,
    /// If set, runs the full testing with trusted setup and proof verification.
    #[structopt(
        short = "p",
        long = "proof-check",
        help = "Performs proof-check for every test case"
    )]
    pub proof_check: bool,
    /// If set, runs only tests whose name contains the specified string.
    #[structopt(
        short = "f",
        long = "filter",
        help = "Runs cases only if they contain the filter string"
    )]
    pub filter: Option<String>,
}

impl Arguments {
    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self::from_args()
    }
}
