//!
//! The Zinc compiler bundler.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::PathBuf;
use std::rc::Rc;

use anyhow::Context;

use crate::generator::state::State;
use crate::semantic::scope::Scope;
use crate::source::Source;

///
/// The Zinc compiler bundler.
///
pub struct Bundler {
    /// The main project path.
    project_path: PathBuf,
    /// The dependency directory path.
    dependencies_directory_path: PathBuf,

    /// The optimization flag.
    optimize_dead_function_elimination: bool,

    /// The compiled dependency modules cache.
    cache: HashMap<(String, semver::Version), Rc<RefCell<Scope>>>,
}

impl Bundler {
    /// The dependencies hashmap default capacity.
    const DEPENDENCIES_INITIAL_CAPACITY: usize = 64;

    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        project_path: PathBuf,
        dependencies_directory_path: PathBuf,
        optimize_dead_function_elimination: bool,
    ) -> Self {
        Self {
            project_path,
            dependencies_directory_path,
            optimize_dead_function_elimination,
            cache: HashMap::with_capacity(Self::DEPENDENCIES_INITIAL_CAPACITY),
        }
    }

    ///
    /// Compiles the project source code with its entire dependency tree.
    ///
    pub fn bundle(&mut self) -> anyhow::Result<zinc_build::Build> {
        let manifest = zinc_manifest::Manifest::try_from(&self.project_path)
            .with_context(|| self.project_path.to_string_lossy().to_string())?;

        let dependencies = match manifest.dependencies {
            Some(ref dependencies) => self.compile_list(&dependencies)?,
            None => HashMap::new(),
        };

        let mut source_directory_path = self.project_path.to_owned();
        source_directory_path.push(zinc_const::directory::SOURCE);

        let source = Source::try_from_entry(&source_directory_path)?;
        let state = source.compile(manifest, dependencies)?;
        let application =
            State::unwrap_rc(state).into_application(self.optimize_dead_function_elimination);

        Ok(application.into_build())
    }

    ///
    /// Compiles a dependency and stores its scope in the bundler instance cache.
    ///
    fn compile_list(
        &mut self,
        dependencies: &HashMap<String, semver::Version>,
    ) -> anyhow::Result<HashMap<String, Rc<RefCell<Scope>>>> {
        let mut compiled = HashMap::with_capacity(dependencies.len());

        for (name, version) in dependencies.iter() {
            let scope = match self.cache.get(&(name.clone(), version.clone())) {
                Some(dependency) => dependency.to_owned(),
                None => {
                    let mut path = self.dependencies_directory_path.to_owned();
                    path.push(format!("{}-{}", name, version));

                    let manifest = zinc_manifest::Manifest::try_from(&path)
                        .with_context(|| path.to_string_lossy().to_string())?;

                    let dependencies = match manifest.dependencies {
                        Some(dependencies) => self.compile_list(&dependencies)?,
                        None => HashMap::new(),
                    };

                    let mut source_directory_path = path.clone();
                    source_directory_path.push(zinc_const::directory::SOURCE);
                    let source = Source::try_from_entry(&source_directory_path)?;
                    let scope = source.modularize(dependencies)?;

                    self.cache
                        .insert((name.to_owned(), version.to_owned()), scope.clone());
                    scope
                }
            };

            compiled.insert(name.to_owned(), scope);
        }

        Ok(compiled)
    }
}
