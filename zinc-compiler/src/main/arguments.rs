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
        long = "witness",
        parse(from_os_str),
        help = "The witness template output path"
    )]
    pub witness_template_path: PathBuf,
    #[structopt(
        long = "public-data",
        parse(from_os_str),
        help = "The public data template output path"
    )]
    pub public_data_template_path: PathBuf,
    #[structopt(
        short = "o",
        long = "output",
        parse(from_os_str),
        help = "The *.znb bytecode output path"
    )]
    pub bytecode_output_path: PathBuf,
    #[structopt(parse(from_os_str), help = "The *.zn source file names")]
    pub source_files: Vec<PathBuf>,
}

impl Arguments {
    pub fn new() -> Self {
        Self::from_args()
    }
}
