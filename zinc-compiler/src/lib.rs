//!
//! The Zinc compiler library.
//!

#![allow(clippy::implicit_hasher)]

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
pub use self::syntax::SyntaxTree;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::rc::Rc;

use self::error::InputError;

pub const BASE_DECIMAL: usize = 10;
pub const BASE_HEXADECIMAL: usize = 16;
pub const BITLENGTH_BOOLEAN: usize = 1;
pub const BITLENGTH_BYTE: usize = 8;
pub const BITLENGTH_INDEX: usize = 64;
pub const BITLENGTH_MAX_INT: usize = 248;
pub const BITLENGTH_FIELD: usize = 254;

pub const SHA256_HASH_SIZE_BITS: usize = 256;

pub static PANIC_LAST_SHARED_REFERENCE: &str = "There are no other references at this point";

pub fn compile_module(
    path: PathBuf,
    bytecode: Rc<RefCell<Bytecode>>,
) -> Result<Rc<RefCell<Scope>>, Error> {
    let syntax_tree = parse(path)?;
    LibraryAnalyzer::new(bytecode).compile(syntax_tree)
}

pub fn compile_entry(
    path: PathBuf,
    bytecode: Rc<RefCell<Bytecode>>,
    dependencies: HashMap<String, Rc<RefCell<Scope>>>,
) -> Result<(), Error> {
    let syntax_tree = parse(path)?;
    BinaryAnalyzer::new(bytecode).compile(syntax_tree, dependencies)
}

pub fn compile_test(code: &str) -> Result<Bytecode, Error> {
    let syntax_tree = Parser::default().parse(code)?;
    let bytecode = Rc::new(RefCell::new(Bytecode::new()));
    BinaryAnalyzer::new(bytecode.clone()).compile(syntax_tree, HashMap::new())?;
    Ok(Rc::try_unwrap(bytecode)
        .expect(PANIC_LAST_SHARED_REFERENCE)
        .into_inner())
}

pub fn parse(path: PathBuf) -> Result<SyntaxTree, Error> {
    let mut file = File::open(path).map_err(|error| InputError::Opening(error.to_string()))?;
    let size = file
        .metadata()
        .map_err(|error| InputError::Metadata(error.to_string()))?
        .len() as usize;
    let mut string = String::with_capacity(size);
    file.read_to_string(&mut string)
        .map_err(|error| InputError::Reading(error.to_string()))?;

    Parser::default().parse(&string)
}
