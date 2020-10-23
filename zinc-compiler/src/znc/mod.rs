//!
//! The Zinc compiler binary.
//!

mod arguments;
mod error;

use std::convert::TryFrom;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process;
use std::thread;

use zinc_build::Build;
use zinc_compiler::Source;
use zinc_compiler::State;
use zinc_manifest::Manifest;

use self::arguments::Arguments;
use self::error::Error;
use self::error::OutputError;

///
/// The application entry point.
///
fn main() {
    process::exit(match main_inner() {
        Ok(()) => zinc_const::exit_code::SUCCESS,
        Err(error) => {
            eprintln!("{}", error);
            zinc_const::exit_code::FAILURE
        }
    })
}

///
/// The auxiliary `main` function to facilitate the `?` error conversion operator.
///
fn main_inner() -> Result<(), Error> {
    let args = Arguments::new();

    zinc_logger::initialize(zinc_const::app_name::COMPILER, args.verbosity);

    let manifest = Manifest::try_from(&args.manifest_path).map_err(Error::Manifest)?;

    let source_directory_path = args.source_directory_path;
    let optimize_dead_function_elimination = args.optimize_dead_function_elimination;
    let build = thread::Builder::new()
        .stack_size(zinc_const::limit::COMPILER_STACK_SIZE)
        .spawn(move || -> Result<Build, Error> {
            let source = Source::try_from_entry(&source_directory_path)?;
            let state = source.compile(manifest)?;
            let application =
                State::unwrap_rc(state).into_application(optimize_dead_function_elimination);
            Ok(application.into_build())
        })
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .join()
        .expect(zinc_const::panic::SYNCHRONIZATION)?;

    let mut build_directory_path = args.binary_path.clone();
    build_directory_path.pop();
    fs::create_dir_all(&build_directory_path).map_err(|error| {
        Error::DirectoryCreating(build_directory_path.as_os_str().to_owned(), error)
    })?;

    let data_directory_path = args.data_directory_path;
    fs::create_dir_all(&data_directory_path).map_err(|error| {
        Error::DirectoryCreating(data_directory_path.as_os_str().to_owned(), error)
    })?;

    let mut input_template_path = data_directory_path;
    input_template_path.push(format!(
        "{}.{}",
        zinc_const::file_name::INPUT,
        zinc_const::extension::JSON
    ));
    let input_template_data =
        serde_json::to_vec_pretty(&build.input).expect(zinc_const::panic::DATA_CONVERSION);
    if !input_template_path.exists() {
        File::create(&input_template_path)
            .map_err(OutputError::Creating)
            .map_err(|error| {
                Error::InputTemplateWriting(input_template_path.as_os_str().to_owned(), error)
            })?
            .write_all(input_template_data.as_slice())
            .map_err(OutputError::Writing)
            .map_err(|error| {
                Error::InputTemplateWriting(input_template_path.as_os_str().to_owned(), error)
            })?;
        log::info!("Input template written to {:?}", input_template_path);
    } else {
        log::info!(
            "Input template file {:?} already exists. Skipping",
            input_template_path
        );
    }

    let binary_path = args.binary_path;
    if binary_path.exists() {
        fs::remove_file(&binary_path)
            .map_err(OutputError::Removing)
            .map_err(|error| Error::BytecodeWriting(binary_path.as_os_str().to_owned(), error))?;
    }
    File::create(&binary_path)
        .map_err(OutputError::Creating)
        .map_err(|error| Error::BytecodeWriting(binary_path.as_os_str().to_owned(), error))?
        .write_all(build.bytecode.as_slice())
        .map_err(OutputError::Writing)
        .map_err(|error| Error::BytecodeWriting(binary_path.as_os_str().to_owned(), error))?;
    log::info!("Compiled to {:?}", binary_path);

    Ok(())
}
