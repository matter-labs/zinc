//!
//! The source code file reader.
//!

pub mod error;

use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::Read;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::RwLock;

use lazy_static::lazy_static;

use crate::generator::bytecode::Bytecode;
use crate::semantic::analyzer::entry::Analyzer as EntryAnalyzer;
use crate::semantic::analyzer::module::Analyzer as ModuleAnalyzer;
use crate::semantic::scope::Scope;
use crate::syntax::parser::Parser;

use self::error::Error;

///
/// The Zinc source code file, which consists of its path and the code.
/// The code is used to be passed to the syntax analyzer and to provide context for error messages.
///
pub struct File {
    pub path: PathBuf,
    pub name: String,
    pub code: String,
}

lazy_static! {
    ///
    /// The global file path array where a `Location` can get the file path by its index.
    ///
    pub static ref INDEX: RwLock<Vec<PathBuf>> = RwLock::new(Vec::new());
}

pub static SOURCE_FILE_EXTENSION: &str = "zn";

impl File {
    pub fn try_into_entry(
        self,
        bytecode: Rc<RefCell<Bytecode>>,
        dependencies: HashMap<String, Rc<RefCell<Scope>>>,
    ) -> Result<(), String> {
        let lines = self.code.lines().collect::<Vec<&str>>();

        let next_file_id = INDEX.read().expect(crate::panic::MUTEX_SYNC).len();
        INDEX
            .write()
            .expect(crate::panic::MUTEX_SYNC)
            .push(self.path);

        let syntax_tree = Parser::default()
            .parse(&self.code, Some(next_file_id))
            .map_err(|error| error.format(&lines))?;

        EntryAnalyzer::new()
            .compile(syntax_tree, dependencies)
            .map_err(|error| error.format(&lines))?
            .write_all_to_bytecode(bytecode);

        Ok(())
    }

    pub fn try_into_module(
        self,
        bytecode: Rc<RefCell<Bytecode>>,
    ) -> Result<Rc<RefCell<Scope>>, String> {
        let lines = self.code.lines().collect::<Vec<&str>>();

        let next_file_id = INDEX.read().expect(crate::panic::MUTEX_SYNC).len();
        INDEX
            .write()
            .expect(crate::panic::MUTEX_SYNC)
            .push(self.path);

        let syntax_tree = Parser::default()
            .parse(&self.code, Some(next_file_id))
            .map_err(|error| error.format(&lines))?;

        let (scope, intermediate) = ModuleAnalyzer::new()
            .compile(syntax_tree)
            .map_err(|error| error.format(&lines))?;

        intermediate.write_all_to_bytecode(bytecode);

        Ok(scope)
    }
}

impl TryFrom<&PathBuf> for File {
    type Error = Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let mut file = ::std::fs::File::open(path).map_err(Error::Opening)?;

        let size = file.metadata().map_err(Error::Metadata)?.len() as usize;

        let mut code = String::with_capacity(size);
        file.read_to_string(&mut code).map_err(Error::Reading)?;

        let source_file_extension = path.extension().ok_or(Error::ExtensionNotFound)?;
        if source_file_extension != SOURCE_FILE_EXTENSION {
            return Err(Error::ExtensionInvalid(source_file_extension.to_owned()));
        }

        let source_file_stem = path
            .file_stem()
            .ok_or(Error::StemNotFound)?
            .to_string_lossy()
            .to_string();

        Ok(Self {
            path: path.to_owned(),
            name: source_file_stem,
            code,
        })
    }
}
