use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "zinc-tester",
    about = "integration test runner for zinc framework"
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

    #[structopt(short = "q", long = "quiet", help = "Doesn't show successful tests.")]
    pub quiet: bool,
}
