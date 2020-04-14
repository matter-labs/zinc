//!
//! The source code.
//!

pub mod error;
pub mod file;

use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::PathBuf;
use std::rc::Rc;

use crate::generator::bytecode::Bytecode;

use self::error::Error;
use self::file::File;

///
/// The Zinc project source code, which consists of some modules and the entry point.
///
pub struct Source {
    pub modules: Vec<File>,
    pub entry: File,
}

pub static MODULE_MAIN_NAME: &str = "main";

impl Source {
    pub fn compile(self) -> Result<Bytecode, String> {
        let bytecode = Rc::new(RefCell::new(Bytecode::new()));

        let mut dependencies = HashMap::with_capacity(self.modules.len());
        for module in self.modules.into_iter() {
            log::info!("Compiling module {:?}", module.path);
            bytecode
                .borrow_mut()
                .start_new_file(module.path.to_string_lossy().as_ref());

            dependencies.insert(
                module.name.clone(),
                module.try_into_module(bytecode.clone())?,
            );
        }

        log::info!("Compiling entry {:?}", self.entry.path);
        bytecode
            .borrow_mut()
            .start_new_file(self.entry.path.to_string_lossy().as_ref());

        self.entry.try_into_entry(bytecode.clone(), dependencies)?;

        let bytecode = Rc::try_unwrap(bytecode)
            .expect(crate::panic::LAST_SHARED_REFERENCE)
            .into_inner();

        Ok(bytecode)
    }
}

impl TryFrom<Vec<PathBuf>> for Source {
    type Error = Error;

    fn try_from(source_paths: Vec<PathBuf>) -> Result<Self, Self::Error> {
        let mut modules = Vec::with_capacity(source_paths.len());
        let mut entry = None;

        for source_file_path in source_paths.into_iter() {
            let file = File::try_from(&source_file_path)
                .map_err(|error| Error::File(source_file_path.into_os_string(), error))?;

            if file.name == MODULE_MAIN_NAME {
                entry = Some(file);
                continue;
            }

            modules.push(file);
        }

        match entry.take() {
            Some(entry) => Ok(Self { modules, entry }),
            None => Err(Error::EntrySourceFileNotFound),
        }
    }
}
