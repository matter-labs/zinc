//!
//! The parser binary.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;

use failure::Fail;
use structopt::StructOpt;

use zinc_compiler::BinaryAnalyzer;
use zinc_compiler::Bytecode;
use zinc_compiler::LibraryAnalyzer;
use zinc_compiler::Parser;
use zinc_compiler::Scope;
use zinc_compiler::SyntaxTree;

#[derive(Debug, StructOpt)]
#[structopt(name = "znc", about = "The Zinc compiler")]
struct Arguments {
    #[structopt(
        short = "i",
        long = "input",
        name = "INPUT",
        parse(from_os_str),
        help = "Specifies the project directory path"
    )]
    input_path: PathBuf,
    #[structopt(
        short = "o",
        long = "output",
        name = "OUTPUT",
        parse(from_os_str),
        help = "Specifies the *.znb output file name"
    )]
    output: PathBuf,
}

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "Input: {}", _0)]
    Input(InputError),
    #[fail(display = "Parser: {}", _0)]
    Compiler(zinc_compiler::Error),
    #[fail(display = "Output: {}", _0)]
    Output(OutputError),
    #[fail(display = "Binary file not found")]
    BinaryNotFound,
}

#[derive(Debug, Fail)]
enum InputError {
    #[fail(display = "Directory: {}", _0)]
    Directory(std::io::Error),
    #[fail(display = "Directory entry: {}", _0)]
    DirectoryEntry(std::io::Error),
    #[fail(display = "File type: {}", _0)]
    FileType(std::io::Error),
    #[fail(display = "File extension not found")]
    FileExtension,
    #[fail(display = "File name not found")]
    FileStem,
    #[fail(display = "Opening: {}", _0)]
    Opening(std::io::Error),
    #[fail(display = "Metadata: {}", _0)]
    Metadata(std::io::Error),
    #[fail(display = "Reading: {}", _0)]
    Reading(std::io::Error),
}

#[derive(Debug, Fail)]
enum OutputError {
    #[fail(display = "Creating: {}", _0)]
    Creating(std::io::Error),
    #[fail(display = "Writing: {}", _0)]
    Writing(std::io::Error),
}

static ZINC_SOURCE_FILE_EXTENSION: &str = "zn";

static PANIC_LAST_SHARED_REFERENCE: &str = "There are no other references at this point";

fn main() -> Result<(), Error> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .init();

    let args: Arguments = Arguments::from_args();

    let bytecode = Rc::new(RefCell::new(Bytecode::new()));
    let directory = fs::read_dir(args.input_path)
        .map_err(InputError::Directory)
        .map_err(Error::Input)?;

    let mut modules = HashMap::<String, Rc<RefCell<Scope>>>::new();
    let mut binary_path = None;

    for entry in directory.into_iter() {
        let entry = entry
            .map_err(InputError::DirectoryEntry)
            .map_err(Error::Input)?;
        let file_type = entry
            .file_type()
            .map_err(InputError::FileType)
            .map_err(Error::Input)?;
        if !file_type.is_file() {
            continue;
        }

        let file_path = entry.path();
        let file_extension = file_path
            .extension()
            .ok_or(InputError::FileExtension)
            .map_err(Error::Input)?;
        if file_extension != ZINC_SOURCE_FILE_EXTENSION {
            continue;
        }

        let file_stem = file_path
            .file_stem()
            .ok_or(InputError::FileStem)
            .map_err(Error::Input)?;
        if file_stem == "main" {
            binary_path = Some(file_path);
            continue;
        }

        let module_name = file_stem.to_string_lossy().to_string();
        let module = LibraryAnalyzer::new(bytecode.clone())
            .compile(syntax_tree(file_path)?)
            .map_err(|error| {
                log::error!("{}", error);
                Error::Compiler(error)
            })?;

        modules.insert(module_name, module);
    }

    match binary_path.take() {
        Some(binary_path) => BinaryAnalyzer::new(bytecode.clone())
            .compile(syntax_tree(binary_path)?, modules)
            .map_err(|error| {
                log::error!("{}", error);
                Error::Compiler(error)
            })?,
        None => return Err(Error::BinaryNotFound),
    }

    log::info!("Output: {:?}", args.output);
    File::create(&args.output)
        .map_err(OutputError::Creating)
        .map_err(Error::Output)?
        .write_all(
            Rc::try_unwrap(bytecode)
                .expect(PANIC_LAST_SHARED_REFERENCE)
                .into_inner()
                .into_bytes()
                .as_slice(),
        )
        .map_err(OutputError::Writing)
        .map_err(Error::Output)?;

    Ok(())
}

fn syntax_tree(path: PathBuf) -> Result<SyntaxTree, Error> {
    log::info!("Input: {:?}", path);
    let mut file = File::open(path)
        .map_err(InputError::Opening)
        .map_err(Error::Input)?;
    let size = file
        .metadata()
        .map_err(InputError::Metadata)
        .map_err(Error::Input)?
        .len() as usize;
    let mut input = String::with_capacity(size);
    file.read_to_string(&mut input)
        .map_err(InputError::Reading)
        .map_err(Error::Input)?;

    Parser::default().parse(input).map_err(|error| {
        log::error!("{}", error);
        Error::Compiler(error)
    })
}
