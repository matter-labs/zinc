//!
//! The Zinc compiler binary.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process;
use std::rc::Rc;

use failure::Fail;
use structopt::StructOpt;

use zinc_compiler::Bytecode;
use zinc_compiler::Scope;

static ZINC_SOURCE_FILE_EXTENSION: &str = "zn";

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

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "source file: {}", _0)]
    SourceFile(FileError),
    #[fail(display = "{}", _0)]
    Compiler(String),
    #[fail(display = "witness template output: {}", _0)]
    WitnessTemplateOutput(OutputError),
    #[fail(display = "public data template output: {}", _0)]
    PublicDataTemplateOutput(OutputError),
    #[fail(display = "bytecode output: {}", _0)]
    BytecodeOutput(OutputError),
    #[fail(display = "the 'main.zn' source file is missing")]
    EntrySourceFileNotFound,
}

#[derive(Debug, Fail)]
enum FileError {
    #[fail(display = "file extension not found")]
    ExtensionNotFound,
    #[fail(display = "file extension is invalid")]
    ExtensionInvalid(OsString),
    #[fail(display = "file name not found")]
    StemNotFound,
}

#[derive(Debug, Fail)]
enum OutputError {
    #[fail(display = "creating: {}", _0)]
    Creating(std::io::Error),
    #[fail(display = "writing: {}", _0)]
    Writing(std::io::Error),
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
    zinc_bytecode::logger::init_logger("znc", args.verbosity);

    let bytecode = Rc::new(RefCell::new(Bytecode::new()));

    let mut modules = HashMap::<String, Rc<RefCell<Scope>>>::new();
    let mut entry_file_path = None;

    for source_file_path in args.source_files.into_iter() {
        let source_file_extension = source_file_path
            .extension()
            .ok_or(FileError::ExtensionNotFound)
            .map_err(Error::SourceFile)?;
        if source_file_extension != ZINC_SOURCE_FILE_EXTENSION {
            return Err(FileError::ExtensionInvalid(
                source_file_extension.to_owned(),
            ))
            .map_err(Error::SourceFile);
        }

        let source_file_stem = source_file_path
            .file_stem()
            .ok_or(FileError::StemNotFound)
            .map_err(Error::SourceFile)?;
        if source_file_stem == "main" {
            entry_file_path = Some(source_file_path);
            continue;
        }

        let module_name = source_file_stem.to_string_lossy().to_string();
        bytecode
            .borrow_mut()
            .start_new_file(source_file_path.to_string_lossy().as_ref());
        log::info!("Compiling {:?}", source_file_path);
        let (module, intermediate) =
            zinc_compiler::compile_module(source_file_path).map_err(Error::Compiler)?;

        modules.insert(module_name, module);
    }

    match entry_file_path.take() {
        Some(entry_file_path) => {
            bytecode
                .borrow_mut()
                .start_new_file(entry_file_path.to_string_lossy().as_ref());
            log::info!("Compiling {:?}", entry_file_path);
            zinc_compiler::compile_entry(entry_file_path, modules).map_err(Error::Compiler)?;
        }
        None => return Err(Error::EntrySourceFileNotFound),
    }

    if !args.witness_template_path.exists() {
        File::create(&args.witness_template_path)
            .map_err(OutputError::Creating)
            .map_err(Error::WitnessTemplateOutput)?
            .write_all(bytecode.borrow().input_template_bytes().as_slice())
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
        .write_all(bytecode.borrow().output_template_bytes().as_slice())
        .map_err(OutputError::Writing)
        .map_err(Error::PublicDataTemplateOutput)?;
    log::info!(
        "Public data template written to {:?}",
        args.public_data_template_path
    );

    let bytecode: Vec<u8> = Rc::try_unwrap(bytecode)
        .expect(zinc_compiler::PANIC_LAST_SHARED_REFERENCE)
        .into_inner()
        .into();

    File::create(&args.bytecode_output_path)
        .map_err(OutputError::Creating)
        .map_err(Error::BytecodeOutput)?
        .write_all(bytecode.as_slice())
        .map_err(OutputError::Writing)
        .map_err(Error::BytecodeOutput)?;
    log::info!("Compiled to {:?}", args.bytecode_output_path);

    Ok(())
}
