//!
//! The source code file.
//!

pub mod error;
pub mod index;

use std::convert::TryFrom;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

use crate::error::Error as CompilerError;
use crate::source::module::error::Error as ModuleError;
use crate::syntax::parser::Parser;
use crate::syntax::tree::module::Module as SyntaxModule;

use self::error::Error;
use self::index::INDEX;

///
/// The Zinc source code file, which consists of its path and the code.
/// The code is used to be passed to the syntax analyzer and to provide context for error messages.
///
#[derive(Debug, Clone)]
pub struct File {
    pub path: PathBuf,
    pub name: String,
    pub code: String,
    pub tree: SyntaxModule,
}

impl File {
    pub fn test(input: &str) -> Result<Self, CompilerError> {
        Ok(Self {
            path: PathBuf::from("test.zn"),
            name: "test".to_owned(),
            code: input.to_owned(),
            tree: Parser::default().parse(input, None)?,
        })
    }

    pub fn is_application_entry(&self) -> bool {
        self.name.as_str() == crate::APPLICATION_ENTRY_FILE_NAME
    }

    pub fn is_module_entry(&self) -> bool {
        self.name.as_str() == crate::MODULE_ENTRY_FILE_NAME
    }
}

impl TryFrom<&PathBuf> for File {
    type Error = ModuleError;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let mut file = fs::File::open(&path)
            .map_err(Error::Opening)
            .map_err(ModuleError::File)?;

        let size = file
            .metadata()
            .map_err(Error::Metadata)
            .map_err(ModuleError::File)?
            .len() as usize;

        let mut code = String::with_capacity(size);
        file.read_to_string(&mut code)
            .map_err(Error::Reading)
            .map_err(ModuleError::File)?;

        let source_file_extension = path
            .extension()
            .ok_or(Error::ExtensionNotFound)
            .map_err(ModuleError::File)?;
        if source_file_extension != crate::SOURCE_FILE_EXTENSION {
            return Err(ModuleError::File(Error::ExtensionInvalid(
                source_file_extension.to_owned(),
            )));
        }

        let name = path
            .file_stem()
            .ok_or(Error::StemNotFound)
            .map_err(ModuleError::File)?
            .to_string_lossy()
            .to_string();

        let lines = code.lines().collect::<Vec<&str>>();

        let next_file_id = INDEX.next(path);
        let tree = Parser::default()
            .parse(&code, Some(next_file_id))
            .map_err(|error| error.format(&lines))
            .map_err(Error::Compiling)
            .map_err(ModuleError::File)?;

        Ok(Self {
            path: path.to_owned(),
            name,
            code,
            tree,
        })
    }
}
