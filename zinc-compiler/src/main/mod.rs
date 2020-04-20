//!
//! The Zinc compiler binary.
//!

#![recursion_limit = "1024"]

mod arguments;
mod error;

use std::convert::TryFrom;
use std::fs::File;
use std::io::Write;
use std::process;

use zinc_compiler::Source;

use self::arguments::Arguments;
use self::error::Error;
use self::error::OutputError;

const EXIT_CODE_SUCCESS: i32 = 0;
const EXIT_CODE_FAILURE: i32 = 1;

fn main() {
    process::exit(match main_inner() {
        Ok(()) => EXIT_CODE_SUCCESS,
        Err(error) => {
            eprintln!("{}", error);
            EXIT_CODE_FAILURE
        }
    })
}

fn main_inner() -> Result<(), Error> {
    let args = Arguments::new();

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
