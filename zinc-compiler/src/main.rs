//!
//! The Zinc compiler binary.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::process;
use std::rc::Rc;

use failure::Fail;
use log::LevelFilter;
use structopt::StructOpt;

use zinc_compiler::BinaryAnalyzer;
use zinc_compiler::Bytecode;
use zinc_compiler::LibraryAnalyzer;
use zinc_compiler::Parser;
use zinc_compiler::Scope;
use zinc_compiler::SyntaxTree;

static ZINC_SOURCE_FILE_EXTENSION: &str = "zn";
static PANIC_LAST_SHARED_REFERENCE: &str = "There are no other references at this point";

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
    verbose: usize,
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
    #[fail(display = "source input: {}", _0)]
    SourceInput(InputError),
    #[fail(display = "compiler: {}:{}", _0, _1)]
    Compiler(String, zinc_compiler::Error),
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
enum InputError {
    #[fail(display = "file extension not found")]
    FileExtensionNotFound,
    #[fail(display = "file extension is invalid")]
    FileExtensionInvalid(OsString),
    #[fail(display = "file name not found")]
    FileStemNotFound,
    #[fail(display = "opening: {}", _0)]
    Opening(std::io::Error),
    #[fail(display = "metadata: {}", _0)]
    Metadata(std::io::Error),
    #[fail(display = "reading: {}", _0)]
    Reading(std::io::Error),
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
            log::error!("{}", error);
            EXIT_CODE_FAILURE
        }
    })
}

fn main_inner(args: Arguments) -> Result<(), Error> {
    init_logger(args.verbose);

    let bytecode = Rc::new(RefCell::new(Bytecode::new()));

    let mut modules = HashMap::<String, Rc<RefCell<Scope>>>::new();
    let mut binary_path = None;

    for source_file_path in args.source_files.into_iter() {
        let source_file_extension = source_file_path
            .extension()
            .ok_or(InputError::FileExtensionNotFound)
            .map_err(Error::SourceInput)?;
        if source_file_extension != ZINC_SOURCE_FILE_EXTENSION {
            return Err(InputError::FileExtensionInvalid(
                source_file_extension.to_owned(),
            ))
            .map_err(Error::SourceInput);
        }

        let source_file_stem = source_file_path
            .file_stem()
            .ok_or(InputError::FileStemNotFound)
            .map_err(Error::SourceInput)?;
        if source_file_stem == "main" {
            binary_path = Some(source_file_path);
            continue;
        }

        let module_name = source_file_stem.to_string_lossy().to_string();
        let module_file_path = format!("src/{}.zn", module_name);
        bytecode.borrow_mut().start_new_file(&module_file_path);
        let module = LibraryAnalyzer::new(bytecode.clone())
            .compile(parse(source_file_path, &module_file_path)?)
            .map_err(|error| Error::Compiler(module_file_path, error))?;

        modules.insert(module_name, module);
    }

    let entry_file_path = "src/main.zn";
    bytecode.borrow_mut().start_new_file(entry_file_path);
    match binary_path.take() {
        Some(binary_path) => BinaryAnalyzer::new(bytecode.clone())
            .compile(parse(binary_path, entry_file_path)?, modules)
            .map_err(|error| Error::Compiler(entry_file_path.to_owned(), error))?,
        None => return Err(Error::EntrySourceFileNotFound),
    }

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
        .expect(PANIC_LAST_SHARED_REFERENCE)
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

fn parse(path: PathBuf, module_name: &str) -> Result<SyntaxTree, Error> {
    log::info!("Compiling   {:?}", path);
    let mut file = File::open(path)
        .map_err(InputError::Opening)
        .map_err(Error::SourceInput)?;
    let size = file
        .metadata()
        .map_err(InputError::Metadata)
        .map_err(Error::SourceInput)?
        .len() as usize;
    let mut input = String::with_capacity(size);
    file.read_to_string(&mut input)
        .map_err(InputError::Reading)
        .map_err(Error::SourceInput)?;

    Parser::default()
        .parse(input)
        .map_err(|error| Error::Compiler(module_name.to_owned(), error))
}

fn init_logger(verbosity: usize) {
    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .filter_level(match verbosity {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        })
        .init();
}
