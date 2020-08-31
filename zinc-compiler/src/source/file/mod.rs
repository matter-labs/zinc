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
use crate::generator::module::Module;
use crate::generator::state::State;
use crate::generator::IBytecodeWritable;
use crate::semantic::analyzer::entry::Analyzer as EntryAnalyzer;
use crate::source::error::Error as SourceError;
use crate::source::Source;
use crate::syntax::parser::Parser;
use crate::syntax::tree::module::Module as SyntaxModule;

use self::error::Error;
use self::index::Data;
use self::index::INDEX;

///
/// The Zinc source code file, which consists of its path and parsed syntax tree.
///
#[derive(Debug, Clone)]
pub struct File {
    /// The full file path.
    pub path: PathBuf,
    /// The file name without the extension.
    pub name: String,
    /// The source code syntax tree.
    pub tree: SyntaxModule,
}

impl File {
    ///
    /// Initializes an application module from a string.
    ///
    /// `path` is used to set the virtual module path within a project.
    ///
    pub fn try_from_string(file: zinc_source::File) -> Result<Self, SourceError> {
        let path = PathBuf::from(file.path);

        let next_file_id = INDEX.next(&path, file.code);
        let tree = Parser::default()
            .parse(
                INDEX
                    .inner
                    .read()
                    .expect(zinc_const::panic::MUTEX_SYNC)
                    .get(&next_file_id)
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
                    .code
                    .as_str(),
                Some(next_file_id),
            )
            .map_err(|error| error.format())
            .map_err(SourceError::Compiling)?;

        Ok(Self {
            path,
            name: file.name,
            tree,
        })
    }

    ///
    /// Initializes an application module from a hard disk file.
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
        if source_file_extension != zinc_const::extension::SOURCE {
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

        let next_file_id = INDEX.next(path, code);
        let tree = Parser::default()
            .parse(
                INDEX
                    .inner
                    .read()
                    .expect(zinc_const::panic::MUTEX_SYNC)
                    .get(&next_file_id)
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
                    .code
                    .as_str(),
                Some(next_file_id),
            )
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
    pub fn compile(self, name: String) -> Result<Rc<RefCell<State>>, SourceError> {
        let scope = EntryAnalyzer::define(Source::File(self))
            .map_err(CompilerError::Semantic)
            .map_err(|error| error.format())
            .map_err(SourceError::Compiling)?;

        let state = State::new(name).wrap();
        Module::new(scope.borrow().get_intermediate()).write_all(state.clone());

        Ok(state)
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
        self.name.as_str() == zinc_const::file_name::APPLICATION_ENTRY
    }

    ///
    /// Checks whether the file is the module entry point.
    ///
    pub fn is_module_entry(&self) -> bool {
        self.name.as_str() == zinc_const::file_name::MODULE_ENTRY
    }

    ///
    /// Initializes a test module file.
    ///
    pub fn test(code: &str, path: PathBuf, file_index: usize) -> Result<Self, CompilerError> {
        INDEX
            .inner
            .write()
            .expect(zinc_const::panic::MUTEX_SYNC)
            .insert(
                file_index,
                Data {
                    path: path.clone(),
                    code: code.to_owned(),
                },
            );

        Ok(Self {
            path,
            name: "test".to_owned(),
            tree: Parser::default().parse(code, Some(file_index))?,
        })
    }
}
