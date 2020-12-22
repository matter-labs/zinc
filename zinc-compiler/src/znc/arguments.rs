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

    /// Suppresses output, if set.
    #[structopt(short = "q", long = "quiet")]
    pub quiet: bool,

    /// The path to the Zinc project manifest file.
    #[structopt(
        long = "manifest-path",
        parse(from_os_str),
        default_value = "./Zargo.toml"
    )]
    pub manifest_path: PathBuf,

    /// The paths to the dependency manifest files.
    #[structopt(long = "dependencies", parse(from_os_str))]
    pub dependency_paths: Vec<PathBuf>,

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
