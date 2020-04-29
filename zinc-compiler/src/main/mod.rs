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

static BINARY_NAME: &str = "znc";

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

    zinc_utils::logger::init_logger(BINARY_NAME, args.verbosity);

    let mut bytecode = Source::try_from(args.source_files)
        .map_err(Error::Source)?
        .compile()
        .map_err(Error::Compiling)?;

    for entry_id in bytecode.entries() {
        let entry_name = bytecode.entry_name(entry_id);

        let mut bytecode_path = args.build_path.clone();
        bytecode_path.push(format!("{}.znb", entry_name));

        let mut witness_template_path = args.data_path.clone();
        witness_template_path.push(format!("{}_witness.json", entry_name));

        let mut public_data_template_path = args.data_path.clone();
        public_data_template_path.push(format!("{}_public_data.json", entry_name));

        if !witness_template_path.exists() {
            File::create(&witness_template_path)
                .map_err(OutputError::Creating)
                .map_err(Error::WitnessTemplateOutput)?
                .write_all(bytecode.input_template_bytes(entry_id).as_slice())
                .map_err(OutputError::Writing)
                .map_err(Error::WitnessTemplateOutput)?;
            log::info!("Witness template written to {:?}", witness_template_path);
        } else {
            log::info!(
                "Witness template {:?} already exists. Skipping",
                witness_template_path
            );
        }

        File::create(&public_data_template_path)
            .map_err(OutputError::Creating)
            .map_err(Error::PublicDataTemplateOutput)?
            .write_all(bytecode.output_template_bytes(entry_id).as_slice())
            .map_err(OutputError::Writing)
            .map_err(Error::PublicDataTemplateOutput)?;
        log::info!(
            "Public data template written to {:?}",
            public_data_template_path
        );

        File::create(&bytecode_path)
            .map_err(OutputError::Creating)
            .map_err(Error::BytecodeOutput)?
            .write_all(bytecode.entry_to_bytes(entry_id).as_slice())
            .map_err(OutputError::Writing)
            .map_err(Error::BytecodeOutput)?;
        log::info!("Compiled to {:?}", bytecode_path);
    }

    Ok(())
}
