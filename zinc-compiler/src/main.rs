//!
//! The Zinc compiler binary.
//!

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::convert::TryFrom;
use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process;
use std::rc::Rc;

use failure::Fail;
use log::debug;
use structopt::StructOpt;

use zinc_compiler::Bytecode;
use zinc_compiler::File as ZincFile;
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

// Topologically sort the module path into L

// L ← Empty list that will contain the sorted nodes
// while exists nodes without a permanent mark do
//     select an unmarked node n
//     visit(n)
//
// function visit(node n)
//     if n has a permanent mark then
//         return
//     if n has a temporary mark then
//         stop   (not a DAG)
//
//     mark n with a temporary mark
//
//     for each node m with an edge from n to m do
//         visit(m)
//
//     remove temporary mark from n
//     mark n with a permanent mark
//     add n to head of L
fn visit(n: PathBuf, L: &mut VecDeque<PathBuf>, temp_marks: &mut Vec<PathBuf>) -> Result<(), Error> {
    debug!("Visiting {}", n.display());
    // if n has a permanent mark then
    //         return
    if L.contains(&n) {
        debug!("Already sorted: {}", n.display());

        return Ok(());
    } // already in sorted list


    // if n has a temporary mark then
    //         stop   (not a DAG)
    debug!("TEMP MARK - CHECK : {}", n.display());
    if temp_marks.contains(&n) {
        debug!("TEMP MARK - CHECK : {}", "FOUND!");
        return Err(Error::Compiler("Cyclic module dependencies are not supported".to_string()));
    }

    // mark n with a temporary mark
    debug!("TEMP MARK - ADD   : {}", n.display());
    // temp_marks.insert(n.clone(), true);
    temp_marks.push(n.clone());


    //  for each node m with an edge from n to m do
    //         visit(m)
    let found_modules = ZincFile::try_from(n.clone())
        .map_err(Error::Compiler)?
        .find_modules()
        .map_err(Error::Compiler)?;

    debug!("Found # modules: {}", found_modules.len());

    found_modules.into_iter().try_for_each(|m| {
        // We assume that all modules are in the root path, next main.zn.
        // File name equals: <module name>.zn
        let module_path = n.with_file_name(m + ".zn");
        visit(module_path, L, temp_marks)
    }).expect("Compilation failed during module graph ordering");

    //     remove temporary mark from n
    debug!("TEMP MARK - REMOVE: {}", n.display());
    if let Some(pos) = temp_marks.iter().position(|x| *x == n) {
        temp_marks.remove(pos);
    }

    //     mark n with a permanent mark. In our case, that is same as adding to head of L
    //     add n to head of L
    debug!("Adding to sorted list: {}", n.display());
    L.push_back(PathBuf::from(&n));

    Ok(())
}

fn ordered_source_files(source_files: Vec<PathBuf>) -> Result<VecDeque<PathBuf>, Error> {
    let mut L = VecDeque::<PathBuf>::new();
    let mut temp_marks = Vec::<PathBuf>::new();

    for source_file_path in source_files.into_iter() {
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

        visit(source_file_path, &mut L, &mut temp_marks)
            .expect("Compilation failed during module graph ordering");
    }
    Ok(L)
}

fn main_inner(args: Arguments) -> Result<(), Error> {
    zinc_bytecode::logger::init_logger("znc", args.verbosity);

    let ordered_source_files = ordered_source_files(args.source_files)
        .map_err(|e| {
            Error::Compiler("Could not determine ordered source files: ".to_string())
        })?;

    ordered_source_files.iter().for_each(|file| debug!("Ordered file: {}", file.display()));

    let bytecode = Rc::new(RefCell::new(Bytecode::new()));

    let mut modules = HashMap::<String, Rc<RefCell<Scope>>>::new();
    let mut entry_file_path = None;

    for source_file_path in ordered_source_files.into_iter() {
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
        let module = ZincFile::try_from(source_file_path)
            .map_err(Error::Compiler)?
            .try_into_module(bytecode.clone(), modules.clone())
            .map_err(Error::Compiler)?;

        modules.insert(module_name, module);
    }

    match entry_file_path.take() {
        Some(entry_file_path) => {
            bytecode
                .borrow_mut()
                .start_new_file(entry_file_path.to_string_lossy().as_ref());

            log::info!("Compiling {:?}", entry_file_path);
            ZincFile::try_from(entry_file_path)
                .map_err(Error::Compiler)?
                .try_into_entry(bytecode.clone(), modules)
                .map_err(Error::Compiler)?;
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

    let bytecode = Rc::try_unwrap(bytecode)
        .expect(zinc_compiler::PANIC_LAST_SHARED_REFERENCE)
        .into_inner();

    File::create(&args.bytecode_output_path)
        .map_err(OutputError::Creating)
        .map_err(Error::BytecodeOutput)?
        .write_all(bytecode.into_bytes().as_slice())
        .map_err(OutputError::Writing)
        .map_err(Error::BytecodeOutput)?;
    log::info!("Compiled to {:?}", args.bytecode_output_path);

    Ok(())
}

