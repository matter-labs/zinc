//!
//! The Zinc compiler binary arguments.
//!

use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "znc", about = "The Zinc compiler")]
pub struct Arguments {
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Show verbose logs, use multiple times for more verbosity"
    )]
    pub verbosity: usize,

    #[structopt(
        long = "data",
        parse(from_os_str),
        help = "The witness and public data directory path"
    )]
    pub data_path: PathBuf,
    #[structopt(long = "build", parse(from_os_str), help = "The build directory path")]
    pub build_path: PathBuf,

    #[structopt(long = "test-only", help = "Build the unit tests only")]
    pub test_only: bool,

    #[structopt(parse(from_os_str), help = "The source file or `src` directory path")]
    pub source_path: PathBuf,
}

impl Arguments {
    pub fn new() -> Self {
        Self::from_args()
    }
}
