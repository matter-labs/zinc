//!
//! The Zinc compiler binary.
//!

pub(crate) mod arguments;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process;
use std::thread;

use anyhow::Context;

use zinc_build::Build;
use zinc_compiler::Source;
use zinc_compiler::State;
use zinc_manifest::Manifest;

use self::arguments::Arguments;

///
/// The application entry point.
///
fn main() {
    process::exit(match main_inner() {
        Ok(()) => zinc_const::exit_code::SUCCESS,
        Err(error) => {
            log::error!("{:?}", error);
            zinc_const::exit_code::FAILURE
        }
    })
}

///
/// The auxiliary `main` function to facilitate the `?` error conversion operator.
///
fn main_inner() -> anyhow::Result<()> {
    let args = Arguments::new();

    zinc_logger::initialize(zinc_const::app_name::COMPILER, args.verbosity);

    let optimize_dead_function_elimination = args.optimize_dead_function_elimination;

    let manifest = Manifest::try_from(&args.manifest_path)
        .with_context(|| args.manifest_path.to_string_lossy().to_string())?;

    let mut data_directory_path = args.manifest_path.clone();
    data_directory_path.push(zinc_const::directory::DATA);
    fs::create_dir_all(&data_directory_path)
        .with_context(|| data_directory_path.to_string_lossy().to_string())?;

    let mut target_directory_path = args.manifest_path.clone();
    target_directory_path.push(if args.optimize_dead_function_elimination {
        zinc_const::directory::TARGET_RELEASE
    } else {
        zinc_const::directory::TARGET_DEBUG
    });
    fs::create_dir_all(&target_directory_path)
        .with_context(|| target_directory_path.to_string_lossy().to_string())?;

    let mut dependencies_directory_path = args.manifest_path.clone();
    dependencies_directory_path.push(zinc_const::directory::TARGET_DEPS);
    fs::create_dir_all(&dependencies_directory_path)
        .with_context(|| dependencies_directory_path.to_string_lossy().to_string())?;

    let mut source_directory_path = args.manifest_path;
    source_directory_path.push(zinc_const::directory::SOURCE);
    let build = thread::Builder::new()
        .stack_size(zinc_const::limit::COMPILER_STACK_SIZE)
        .spawn(move || -> anyhow::Result<Build> {
            let dependencies_directory = fs::read_dir(dependencies_directory_path)?;
            let mut dependencies = HashMap::with_capacity(
                manifest
                    .dependencies
                    .as_ref()
                    .map(HashMap::len)
                    .unwrap_or_default(),
            );
            for entry in dependencies_directory.into_iter() {
                let entry = entry?;
                let path = entry.path();
                let entry_type = entry
                    .file_type()
                    .with_context(|| path.to_string_lossy().to_string())?;
                if !entry_type.is_dir() {
                    continue;
                }

                let manifest = Manifest::try_from(&path)
                    .with_context(|| path.to_string_lossy().to_string())?;
                let name = manifest.project.name.clone();

                let mut source_directory_path = path.clone();
                source_directory_path.push(zinc_const::directory::SOURCE);
                let source = Source::try_from_entry(&source_directory_path)?;
                let scope = source.modularize()?;

                dependencies.insert(name, scope);
            }

            let source = Source::try_from_entry(&source_directory_path)?;
            let state = source.compile(manifest, dependencies)?;
            let application =
                State::unwrap_rc(state).into_application(optimize_dead_function_elimination);
            Ok(application.into_build())
        })
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .join()
        .expect(zinc_const::panic::SYNCHRONIZATION)?;

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
            .with_context(|| input_template_path.to_string_lossy().to_string())?
            .write_all(input_template_data.as_slice())
            .with_context(|| input_template_path.to_string_lossy().to_string())?;
        log::info!("Input template written to {:?}", input_template_path);
    } else {
        log::info!(
            "Input template file {:?} already exists. Skipping",
            input_template_path
        );
    }

    let mut binary_path = target_directory_path;
    binary_path.push(format!(
        "{}.{}",
        zinc_const::file_name::BINARY,
        zinc_const::extension::BINARY,
    ));
    if binary_path.exists() {
        fs::remove_file(&binary_path).with_context(|| binary_path.to_string_lossy().to_string())?;
    }
    File::create(&binary_path)
        .with_context(|| binary_path.to_string_lossy().to_string())?
        .write_all(build.bytecode.as_slice())
        .with_context(|| binary_path.to_string_lossy().to_string())?;
    log::info!("Compiled to {:?}", binary_path);

    Ok(())
}
