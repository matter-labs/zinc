//!
//! The Zinc compiler arguments.
//!

use std::path::PathBuf;

use structopt::StructOpt;

///
/// The Zinc compiler arguments.
///
#[derive(Debug, StructOpt)]
#[structopt(name = zinc_const::app_name::COMPILER, about = "The Zinc compiler")]
pub struct Arguments {
    /// Prints more logs, if passed several times.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbosity: usize,

    /// The path to the Zinc project manifest file.
    #[structopt(
        long = "manifest-path",
        parse(from_os_str),
        default_value = "./Zargo.toml"
    )]
    pub manifest_path: PathBuf,

    /// The path to the source code directory.
    #[structopt(parse(from_os_str), default_value = "./src/")]
    pub source_directory_path: PathBuf,

    /// The path to the keys, template, and other auxiliary data directory.
    #[structopt(long = "data", parse(from_os_str), default_value = "./data/")]
    pub data_directory_path: PathBuf,

    /// The path to the bytecode file.
    #[structopt(
        long = "binary",
        parse(from_os_str),
        default_value = "./build/main.znb"
    )]
    pub binary_path: PathBuf,

    /// Builds only the unit tests.
    #[structopt(long = "test-only")]
    pub test_only: bool,

    /// Enables the dead function code elimination optimization.
    #[structopt(long = "opt-dfe")]
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
