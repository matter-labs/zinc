//!
//! The Zinc compiler binary.
//!

mod arguments;
mod error;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::process;
use std::thread;

use zinc_build::Bytes;
use zinc_compiler::Source;
use zinc_compiler::State;

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

    zinc_utils::initialize_logger(zinc_const::app_name::ZINC_COMPILER, args.verbosity);

    let source_directory_path = args.source_directory_path;
    let project_name = args.name;
    let optimize_dead_function_elimination = args.optimize_dead_function_elimination;
    let bytes = thread::Builder::new()
        .stack_size(zinc_const::limit::COMPILER_STACK_SIZE)
        .spawn(move || -> Result<Bytes, Error> {
            let source = Source::try_from_entry(&source_directory_path)?;
            let state = source.compile(project_name)?;
            let application =
                State::unwrap_rc(state).into_application(optimize_dead_function_elimination);
            Ok(application.into_bytes())
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

    match bytes {
        Bytes::Circuit {
            bytecode,
            input_template,
            output_template,
        } => {
            let mut witness_template_path = data_directory_path.clone();
            witness_template_path.push(format!(
                "{}.{}",
                zinc_const::file_name::WITNESS,
                zinc_const::extension::JSON
            ));
            if !witness_template_path.exists() {
                File::create(&witness_template_path)
                    .map_err(OutputError::Creating)
                    .map_err(|error| {
                        Error::WitnessTemplateOutput(
                            witness_template_path.as_os_str().to_owned(),
                            error,
                        )
                    })?
                    .write_all(input_template.as_slice())
                    .map_err(OutputError::Writing)
                    .map_err(|error| {
                        Error::WitnessTemplateOutput(
                            witness_template_path.as_os_str().to_owned(),
                            error,
                        )
                    })?;
                log::info!("Witness template written to {:?}", witness_template_path);
            } else {
                log::info!(
                    "Witness template {:?} already exists. Skipping",
                    witness_template_path
                );
            }

            let mut public_data_template_path = data_directory_path;
            public_data_template_path.push(format!(
                "{}.{}",
                zinc_const::file_name::PUBLIC_DATA,
                zinc_const::extension::JSON
            ));
            File::create(&public_data_template_path)
                .map_err(OutputError::Creating)
                .map_err(|error| {
                    Error::PublicDataTemplateOutput(
                        public_data_template_path.as_os_str().to_owned(),
                        error,
                    )
                })?
                .write_all(output_template.as_slice())
                .map_err(OutputError::Writing)
                .map_err(|error| {
                    Error::PublicDataTemplateOutput(
                        public_data_template_path.as_os_str().to_owned(),
                        error,
                    )
                })?;
            log::info!(
                "Public data template written to {:?}",
                public_data_template_path
            );

            let binary_path = args.binary_path;
            if binary_path.exists() {
                fs::remove_file(&binary_path)
                    .map_err(OutputError::Removing)
                    .map_err(|error| {
                        Error::BytecodeOutput(binary_path.as_os_str().to_owned(), error)
                    })?;
            }
            File::create(&binary_path)
                .map_err(OutputError::Creating)
                .map_err(|error| Error::BytecodeOutput(binary_path.as_os_str().to_owned(), error))?
                .write_all(bytecode.as_slice())
                .map_err(OutputError::Writing)
                .map_err(|error| {
                    Error::BytecodeOutput(binary_path.as_os_str().to_owned(), error)
                })?;
            log::info!("Compiled to {:?}", binary_path);
        }
        Bytes::Contract {
            bytecode,
            storage,
            input_templates,
            output_templates,
        } => {
            for (name, bytes) in input_templates.into_iter() {
                let mut witness_template_path = data_directory_path.clone();
                witness_template_path.push(format!(
                    "{}_{}.{}",
                    zinc_const::file_name::WITNESS,
                    name,
                    zinc_const::extension::JSON
                ));

                if !witness_template_path.exists() {
                    File::create(&witness_template_path)
                        .map_err(OutputError::Creating)
                        .map_err(|error| {
                            Error::WitnessTemplateOutput(
                                witness_template_path.as_os_str().to_owned(),
                                error,
                            )
                        })?
                        .write_all(bytes.as_slice())
                        .map_err(OutputError::Writing)
                        .map_err(|error| {
                            Error::WitnessTemplateOutput(
                                witness_template_path.as_os_str().to_owned(),
                                error,
                            )
                        })?;
                    log::info!("Witness template written to {:?}", witness_template_path);
                } else {
                    log::info!(
                        "Witness template file {:?} already exists. Skipping",
                        witness_template_path
                    );
                }
            }

            for (name, bytes) in output_templates.into_iter() {
                let mut public_data_template_path = data_directory_path.clone();
                public_data_template_path.push(format!(
                    "{}_{}.{}",
                    zinc_const::file_name::PUBLIC_DATA,
                    name,
                    zinc_const::extension::JSON
                ));

                File::create(&public_data_template_path)
                    .map_err(OutputError::Creating)
                    .map_err(|error| {
                        Error::PublicDataTemplateOutput(
                            public_data_template_path.as_os_str().to_owned(),
                            error,
                        )
                    })?
                    .write_all(bytes.as_slice())
                    .map_err(OutputError::Writing)
                    .map_err(|error| {
                        Error::PublicDataTemplateOutput(
                            public_data_template_path.as_os_str().to_owned(),
                            error,
                        )
                    })?;
                log::info!(
                    "Public data template written to {:?}",
                    public_data_template_path
                );
            }

            let mut storage_path = data_directory_path;
            storage_path.push(format!(
                "{}.{}",
                zinc_const::file_name::STORAGE,
                zinc_const::extension::JSON,
            ));
            if !storage_path.exists() {
                File::create(&storage_path)
                    .map_err(OutputError::Creating)
                    .map_err(|error| {
                        Error::BytecodeOutput(storage_path.as_os_str().to_owned(), error)
                    })?
                    .write_all(storage.as_slice())
                    .map_err(OutputError::Writing)
                    .map_err(|error| {
                        Error::StorageOutput(storage_path.as_os_str().to_owned(), error)
                    })?;
                log::info!("Storage written to {:?}", storage_path);
            } else {
                log::info!("Storage file {:?} already exists. Skipping", storage_path);
            }

            let binary_path = args.binary_path;
            if binary_path.exists() {
                fs::remove_file(&binary_path)
                    .map_err(OutputError::Removing)
                    .map_err(|error| {
                        Error::BytecodeOutput(binary_path.as_os_str().to_owned(), error)
                    })?;
            }
            File::create(&binary_path)
                .map_err(OutputError::Creating)
                .map_err(|error| Error::BytecodeOutput(binary_path.as_os_str().to_owned(), error))?
                .write_all(bytecode.as_slice())
                .map_err(OutputError::Writing)
                .map_err(|error| {
                    Error::BytecodeOutput(binary_path.as_os_str().to_owned(), error)
                })?;
            log::info!("Compiled to {:?}", binary_path);
        }
    }

    Ok(())
}
