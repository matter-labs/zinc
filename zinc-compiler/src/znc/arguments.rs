//!
//! The Zinc compiler arguments.
//!

use std::path::PathBuf;

use structopt::StructOpt;

///
/// The Zinc compiler arguments.
///
#[derive(Debug, StructOpt)]
#[structopt(name = zinc_const::app_name::ZINC_COMPILER, about = "The Zinc compiler")]
pub struct Arguments {
    /// The logging level value, which helps the logger to set the logging level.
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Show verbose logs, use multiple times for more verbosity"
    )]
    pub verbosity: usize,

    /// The path to the keys, template, and other auxiliary data directory.
    #[structopt(
        long = "data",
        parse(from_os_str),
        help = "The witness and public data directory path"
    )]
    pub data_path: PathBuf,

    /// The path to the build directory.
    #[structopt(long = "build", parse(from_os_str), help = "The build directory path")]
    pub build_path: PathBuf,

    /// If set, compiles only unit tests.
    #[structopt(long = "test-only", help = "Build the unit tests only")]
    pub test_only: bool,

    /// The path to the source code directory.
    #[structopt(parse(from_os_str), help = "The source file or `src` directory path")]
    pub source_path: PathBuf,
}

impl Arguments {
    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self::from_args()
    }
}
