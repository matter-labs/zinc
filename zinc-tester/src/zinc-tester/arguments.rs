//!
//! The Zinc tester arguments.
//!

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "zinc-tester",
    about = "The integration test runner for the Zinc framework"
)]
pub struct Arguments {
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity."
    )]
    pub verbosity: usize,
    #[structopt(
        short = "p",
        long = "proof-check",
        help = "Performs proof-check for every test case"
    )]
    pub proof_check: bool,
    #[structopt(
        short = "f",
        long = "filter",
        help = "Runs cases only if they contain the filter string"
    )]
    pub filter: Option<String>,
}

impl Arguments {
    pub fn new() -> Self {
        Self::from_args()
    }
}
