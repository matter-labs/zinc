//!
//! The Zinc compiler binary.
//!

use std::convert::TryFrom;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process;

use structopt::StructOpt;

use zinc_compiler::Source;
use zinc_compiler::SourceError;

const EXIT_CODE_SUCCESS: i32 = 0;
const EXIT_CODE_FAILURE: i32 = 1;

#[derive(Debug, StructOpt)]
#[structopt(name = "znc", about = "The Zinc compiler")]
struct Arguments {
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    verbosity: usize,
    #[structopt(
        long = "witness",
        parse(from_os_str),
        help = "The witness template output path"
    )]
    witness_template_path: PathBuf,
    #[structopt(
        long = "public-data",
        parse(from_os_str),
        help = "The public data template output path"
    )]
    public_data_template_path: PathBuf,
    #[structopt(
        short = "o",
        long = "output",
        parse(from_os_str),
        help = "The *.znb bytecode output path"
    )]
    bytecode_output_path: PathBuf,
    #[structopt(parse(from_os_str), help = "The *.zn source file names")]
    source_files: Vec<PathBuf>,
}

fn main() {
    let args: Arguments = Arguments::from_args();

    process::exit(match main_inner(args) {
        Ok(()) => EXIT_CODE_SUCCESS,
        Err(error) => {
            eprintln!("{}", error);
            EXIT_CODE_FAILURE
        }
    })
}

fn main_inner(args: Arguments) -> Result<(), Error> {
    zinc_utils::logger::init_logger("znc", args.verbosity);

    let bytecode = Source::try_from(args.source_files)
        .map_err(Error::Source)?
        .compile()
        .map_err(Error::Compiling)?;

    if !args.witness_template_path.exists() {
        File::create(&args.witness_template_path)
            .map_err(OutputError::Creating)
            .map_err(Error::WitnessTemplateOutput)?
            .write_all(bytecode.input_template_bytes().as_slice())
            .map_err(OutputError::Writing)
            .map_err(Error::WitnessTemplateOutput)?;
        log::info!(
            "Witness template written to {:?}",
            args.witness_template_path
        );
    }

    File::create(&args.public_data_template_path)
        .map_err(OutputError::Creating)
        .map_err(Error::PublicDataTemplateOutput)?
        .write_all(bytecode.output_template_bytes().as_slice())
        .map_err(OutputError::Writing)
        .map_err(Error::PublicDataTemplateOutput)?;
    log::info!(
        "Public data template written to {:?}",
        args.public_data_template_path
    );

    File::create(&args.bytecode_output_path)
        .map_err(OutputError::Creating)
        .map_err(Error::BytecodeOutput)?
        .write_all(bytecode.into_bytes().as_slice())
        .map_err(OutputError::Writing)
        .map_err(Error::BytecodeOutput)?;
    log::info!("Compiled to {:?}", args.bytecode_output_path);

    Ok(())
}

enum Error {
    Source(SourceError),
    Compiling(String),
    WitnessTemplateOutput(OutputError),
    PublicDataTemplateOutput(OutputError),
    BytecodeOutput(OutputError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Source(inner) => write!(f, "{}", inner),
            Self::Compiling(inner) => write!(f, "{}", inner),
            Self::WitnessTemplateOutput(inner) => write!(f, "witness template output {}", inner),
            Self::PublicDataTemplateOutput(inner) => {
                write!(f, "public data template output {}", inner)
            }
            Self::BytecodeOutput(inner) => write!(f, "bytecode output {}", inner),
        }
    }
}

enum OutputError {
    Creating(std::io::Error),
    Writing(std::io::Error),
}

impl fmt::Display for OutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Creating(inner) => write!(f, "creating {}", inner),
            Self::Writing(inner) => write!(f, "writing {}", inner),
        }
    }
}
