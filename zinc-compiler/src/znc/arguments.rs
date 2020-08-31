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

    /// The project name, which is specified in the project manifest file.
    #[structopt(
        long = "name",
        help = "The project name specified in the manifest file"
    )]
    pub name: String,

    /// The path to the source code directory.
    #[structopt(
        parse(from_os_str),
        help = "The source file or `src` directory path",
        default_value = zinc_const::path::SOURCE,
    )]
    pub source_directory_path: PathBuf,

    /// The path to the keys, template, and other auxiliary data directory.
    #[structopt(
        long = "data",
        parse(from_os_str),
        help = "The witness and public data directory path",
        default_value = zinc_const::path::DATA,
    )]
    pub data_directory_path: PathBuf,

    /// The path to the bytecode file.
    #[structopt(
        long = "binary",
        parse(from_os_str),
        help = "The path to the bytecode file",
        default_value = zinc_const::path::BINARY,
    )]
    pub binary_path: PathBuf,

    /// If set, compiles only unit tests.
    #[structopt(long = "test-only", help = "Build the unit tests only")]
    pub test_only: bool,

    /// Whether to apply the dead function elimination optimization.
    #[structopt(long = "opt-dfe", help = "Eliminate the dead function code")]
    pub optimize_dead_function_elimination: bool,
}

impl Arguments {
    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self::from_args()
    }
}
