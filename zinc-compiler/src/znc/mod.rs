//!
//! The Zinc compiler binary.
//!

mod arguments;
mod error;

use std::fs::File;
use std::io::Write;
use std::process;

use zinc_compiler::Bytecode;
use zinc_compiler::Source;

use self::arguments::Arguments;
use self::error::Error;
use self::error::OutputError;

static BINARY_NAME: &str = "znc";
static WITNESS_TEMPLATE_SUFFIX: &str = "_witness";
static PUBLIC_DATA_TEMPLATE_SUFFIX: &str = "_public_data";

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

    let bytecode = Source::try_from_path(&args.source_path, true)?.compile()?;
    let compiled_entries = Bytecode::unwrap_rc(bytecode).into_entries();

    for (entry_name, entry_data) in compiled_entries.into_iter() {
        let mut bytecode_path = args.build_path.clone();
        bytecode_path.push(format!("{}.znb", entry_name));

        let mut witness_template_path = args.data_path.clone();
        witness_template_path.push(format!("{}{}.json", entry_name, WITNESS_TEMPLATE_SUFFIX));

        let mut public_data_template_path = args.data_path.clone();
        public_data_template_path.push(format!(
            "{}{}.json",
            entry_name, PUBLIC_DATA_TEMPLATE_SUFFIX
        ));

        if !witness_template_path.exists() {
            File::create(&witness_template_path)
                .map_err(OutputError::Creating)
                .map_err(Error::WitnessTemplateOutput)?
                .write_all(entry_data.witness_template.as_slice())
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
            .write_all(entry_data.public_data_template.as_slice())
            .map_err(OutputError::Writing)
            .map_err(Error::PublicDataTemplateOutput)?;
        log::info!(
            "Public data template written to {:?}",
            public_data_template_path
        );

        File::create(&bytecode_path)
            .map_err(OutputError::Creating)
            .map_err(Error::BytecodeOutput)?
            .write_all(entry_data.bytecode.as_slice())
            .map_err(OutputError::Writing)
            .map_err(Error::BytecodeOutput)?;
        log::info!("Compiled to {:?}", bytecode_path);
    }

    Ok(())
}
