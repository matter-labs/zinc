//!
//! The file reader.
//!

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
use crate::syntax::tree::statement::local_mod::Statement;

use self::error::Error;

pub mod error;

pub struct File {
    pub path: PathBuf,
    pub code: String,
}


lazy_static! {
    pub static ref INDEX: RwLock<Vec<PathBuf>> = RwLock::new(Vec::new());
}

impl File {
    pub fn try_into_entry(
        self,
        bytecode: Rc<RefCell<Bytecode>>,
        dependencies: HashMap<String, Rc<RefCell<Scope>>>,
    ) -> Result<(), String> {
        let lines = self.code.lines().collect::<Vec<&str>>();

        let next_file_id = INDEX.read().expect(crate::PANIC_MUTEX_SYNC).len();
        INDEX
            .write()
            .expect(crate::PANIC_MUTEX_SYNC)
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
        dependencies: HashMap<String, Rc<RefCell<Scope>>>,
    ) -> Result<Rc<RefCell<Scope>>, String> {
        let lines = self.code.lines().collect::<Vec<&str>>();

        let next_file_id = INDEX.read().expect(crate::PANIC_MUTEX_SYNC).len();
        INDEX
            .write()
            .expect(crate::PANIC_MUTEX_SYNC)
            .push(self.path);

        let syntax_tree = Parser::default()
            .parse(&self.code, Some(next_file_id))
            .map_err(|error| error.format(&lines))?;

        let (scope, intermediate) = ModuleAnalyzer::new()
            .compile(syntax_tree, dependencies)
            .map_err(|error| error.format(&lines))?;

        intermediate.write_all_to_bytecode(bytecode);

        Ok(scope)
    }

    pub fn find_modules(self) -> Result<Vec<String>, String> {
        let lines = self.code.lines().collect::<Vec<&str>>();

        let next_file_id = INDEX.read().expect(crate::PANIC_MUTEX_SYNC).len();
        INDEX
            .write()
            .expect(crate::PANIC_MUTEX_SYNC)
            .push(self.path);

        let syntax_tree = Parser::default()
            .parse(&self.code, Some(next_file_id))
            .map_err(|error| error.format(&lines))?;

        Ok(syntax_tree.statements
            .into_iter()
            .fold(Vec::new(), |mut modules, statement| {
                if let Statement::Mod(s) = statement {
                    modules.push(s.identifier.name);
                }
                modules
            })
        )
    }
}

impl TryFrom<PathBuf> for File {
    type Error = String;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let mut file = ::std::fs::File::open(&path)
            .map_err(Error::Opening)
            .map_err(|error| error.to_string())?;

        let size = file
            .metadata()
            .map_err(Error::Metadata)
            .map_err(|error| error.to_string())?
            .len() as usize;

        let mut code = String::with_capacity(size);
        file.read_to_string(&mut code)
            .map_err(Error::Reading)
            .map_err(|error| error.to_string())?;

        Ok(Self { path, code })
    }
}
