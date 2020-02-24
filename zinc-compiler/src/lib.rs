//!
//! The Zinc compiler library.
//!

#![allow(clippy::implicit_hasher)]
#![allow(clippy::large_enum_variant)]

mod error;
mod lexical;
mod semantic;
mod syntax;

pub use self::error::Error;
pub use self::semantic::BinaryAnalyzer;
pub use self::semantic::Bytecode;
pub use self::semantic::LibraryAnalyzer;
pub use self::semantic::Scope;
pub use self::syntax::Parser;
pub use self::syntax::Tree;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::RwLock;

use lazy_static::lazy_static;

use self::error::file::Error as FileError;

pub const BASE_DECIMAL: usize = 10;
pub const BASE_HEXADECIMAL: usize = 16;
pub const BITLENGTH_BOOLEAN: usize = 1;
pub const BITLENGTH_BYTE: usize = 8;
pub const BITLENGTH_INDEX: usize = 64;
pub const BITLENGTH_MAX_INT: usize = 248;
pub const BITLENGTH_FIELD: usize = 254;

pub const SHA256_HASH_SIZE_BITS: usize = 256;

pub static PANIC_LAST_SHARED_REFERENCE: &str = "There are no other references at this point";
pub static PANIC_MUTEX_SYNC: &str = "Mutexes never panic";
pub static PANIC_FILE_INDEX: &str = "File record always exists";

lazy_static! {
    static ref FILE_INDEX: RwLock<HashMap<usize, PathBuf>> = RwLock::new(HashMap::new());
}

pub fn compile_module(
    path: PathBuf,
    bytecode: Rc<RefCell<Bytecode>>,
) -> Result<Rc<RefCell<Scope>>, String> {
    let syntax_tree = parse(path)?;
    LibraryAnalyzer::new(bytecode)
        .compile(syntax_tree)
        .map_err(|error| error.into())
}

pub fn compile_entry(
    path: PathBuf,
    bytecode: Rc<RefCell<Bytecode>>,
    dependencies: HashMap<String, Rc<RefCell<Scope>>>,
) -> Result<(), String> {
    let syntax_tree = parse(path)?;
    BinaryAnalyzer::new(bytecode)
        .compile(syntax_tree, dependencies)
        .map_err(|error| error.into())
}

pub fn compile_test(code: &str) -> Result<Bytecode, String> {
    let lines = code.lines().collect::<Vec<&str>>();

    let syntax_tree = Parser::default()
        .parse(code, None)
        .map_err(|error| error.format(&lines))?;
    let bytecode = Rc::new(RefCell::new(Bytecode::new()));
    BinaryAnalyzer::new(bytecode.clone())
        .compile(syntax_tree, HashMap::new())
        .map_err(|error| error.format(&lines))?;
    Ok(Rc::try_unwrap(bytecode)
        .expect(PANIC_LAST_SHARED_REFERENCE)
        .into_inner())
}

pub fn parse(path: PathBuf) -> Result<Tree, String> {
    let mut file = File::open(path.clone())
        .map_err(FileError::Opening)
        .map_err(|error| error.to_string())?;
    let size = file
        .metadata()
        .map_err(FileError::Metadata)
        .map_err(|error| error.to_string())?
        .len() as usize;
    let mut code = String::with_capacity(size);
    file.read_to_string(&mut code)
        .map_err(FileError::Reading)
        .map_err(|error| error.to_string())?;

    let lines = code.lines().collect::<Vec<&str>>();

    let next_file_id = FILE_INDEX.read().expect(PANIC_MUTEX_SYNC).len();
    FILE_INDEX
        .write()
        .expect(PANIC_MUTEX_SYNC)
        .insert(next_file_id, path);
    Parser::default()
        .parse(&code, Some(next_file_id))
        .map_err(|error| error.format(&lines))
}
