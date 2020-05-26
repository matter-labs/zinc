//!
//! The source code file.
//!

pub mod error;
pub mod index;

use std::cell::RefCell;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::rc::Rc;

use crate::error::Error as CompilerError;
use crate::generator::bytecode::Bytecode;
use crate::generator::program::Program;
use crate::semantic::analyzer::entry::Analyzer as EntryAnalyzer;
use crate::source::error::Error as SourceError;
use crate::source::Source;
use crate::syntax::parser::Parser;
use crate::syntax::tree::module::Module as SyntaxModule;

use self::error::Error;
use self::index::INDEX;

///
/// The Zinc source code file, which consists of its path and parsed syntax tree.
///
#[derive(Debug, Clone)]
pub struct File {
    pub path: PathBuf,
    pub name: String,
    pub tree: SyntaxModule,
}

impl File {
    ///
    /// Initializes an application module file.
    ///
    pub fn try_from_path(path: &PathBuf) -> Result<Self, SourceError> {
        let mut file = fs::File::open(&path)
            .map_err(Error::Opening)
            .map_err(SourceError::File)?;

        let size = file
            .metadata()
            .map_err(Error::Metadata)
            .map_err(SourceError::File)?
            .len() as usize;

        let mut code = String::with_capacity(size);
        file.read_to_string(&mut code)
            .map_err(Error::Reading)
            .map_err(SourceError::File)?;

        let source_file_extension = path
            .extension()
            .ok_or(Error::ExtensionNotFound)
            .map_err(SourceError::File)?;
        if source_file_extension != crate::SOURCE_FILE_EXTENSION {
            return Err(SourceError::File(Error::ExtensionInvalid(
                source_file_extension.to_owned(),
            )));
        }

        let name = path
            .file_stem()
            .ok_or(Error::StemNotFound)
            .map_err(SourceError::File)?
            .to_string_lossy()
            .to_string();

        let next_file_id = INDEX.next(path, code.clone());
        let tree = Parser::default()
            .parse(&code, Some(next_file_id))
            .map_err(|error| error.format())
            .map_err(SourceError::Compiling)?;

        Ok(Self {
            path: path.to_owned(),
            name,
            tree,
        })
    }

    ///
    /// Gets all the intermediate represenation scattered around the application scope tree and
    /// writes it to the bytecode.
    ///
    pub fn compile(self) -> Result<Rc<RefCell<Bytecode>>, SourceError> {
        let scope = EntryAnalyzer::define(Source::File(self))
            .map_err(CompilerError::Semantic)
            .map_err(|error| error.format())
            .map_err(SourceError::Compiling)?;

        let bytecode = Bytecode::new().wrap();
        Program::new(scope.borrow().get_intermediate()).write_all_to_bytecode(bytecode.clone());

        Ok(bytecode)
    }

    ///
    /// Checks whether the file is the entry point.
    ///
    pub fn is_entry(&self) -> bool {
        self.is_application_entry() || self.is_module_entry()
    }

    ///
    /// Checks whether the file is the application entry point.
    ///
    pub fn is_application_entry(&self) -> bool {
        self.name.as_str() == crate::APPLICATION_ENTRY_FILE_NAME
    }

    ///
    /// Checks whether the file is the module entry point.
    ///
    pub fn is_module_entry(&self) -> bool {
        self.name.as_str() == crate::MODULE_ENTRY_FILE_NAME
    }

    ///
    /// Initialized a test module file.
    ///
    pub fn test(input: &str) -> Result<Self, CompilerError> {
        Ok(Self {
            path: PathBuf::from("test.zn"),
            name: "test".to_owned(),
            tree: Parser::default().parse(input, None)?,
        })
    }
}
