//!
//! The Zinc compiler bundler.
//!

pub mod dependency;
pub mod error;

use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::PathBuf;
use std::rc::Rc;

use anyhow::Context;

use crate::generator::zinc_vm::State as ZincVMState;
use crate::semantic::scope::Scope;
use crate::source::Source;

use self::dependency::Dependency;
use self::error::Error;

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
    cache: HashMap<(String, semver::Version), Dependency>,
    /// The allocated dependency graph node indexes.
    node_indexes: HashMap<(String, semver::Version), petgraph::graph::NodeIndex>,
    /// The dependency graph.
    graph: petgraph::Graph<zinc_project::ManifestProject, ()>,
}

impl Bundler {
    /// The dependencies hashmap default capacity.
    const DEPENDENCIES_INITIAL_CAPACITY: usize = 64;
    /// The dependency graph node indexes hashmap default capacity.
    const NODE_INDEXES_INITIAL_CAPACITY: usize = 64;

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
            node_indexes: HashMap::with_capacity(Self::NODE_INDEXES_INITIAL_CAPACITY),
            graph: petgraph::Graph::new(),
        }
    }

    ///
    /// Compiles the project source code with its entire dependency tree.
    ///
    pub fn bundle(&mut self) -> anyhow::Result<zinc_types::Build> {
        let manifest = zinc_project::Manifest::try_from(&self.project_path)
            .with_context(|| self.project_path.to_string_lossy().to_string())?;

        let node_index = self.graph.add_node(manifest.project.clone());

        let dependencies = match manifest.dependencies {
            Some(ref dependencies) => self.compile_list(node_index, &dependencies)?,
            None => HashMap::new(),
        };

        let mut source_directory_path = self.project_path.to_owned();
        source_directory_path.push(zinc_const::directory::SOURCE);

        let source = Source::try_from_entry(&source_directory_path)?;
        let state = source.compile(manifest, dependencies)?;
        let application =
            ZincVMState::unwrap_rc(state).into_application(self.optimize_dead_function_elimination);

        Ok(application.into_build())
    }

    ///
    /// Compiles a dependency and stores its scope in the bundler instance cache.
    ///
    fn compile_list(
        &mut self,
        parent_node_index: petgraph::graph::NodeIndex,
        dependencies: &HashMap<String, semver::Version>,
    ) -> anyhow::Result<HashMap<String, Rc<RefCell<Scope>>>> {
        let mut compiled = HashMap::with_capacity(dependencies.len());

        for (name, version) in dependencies.iter() {
            let scope = match self.cache.get(&(name.clone(), version.clone())) {
                Some(dependency) => {
                    self.graph
                        .add_edge(parent_node_index, dependency.node_index, ());
                    self.check_dependency(parent_node_index, dependency.node_index)?;

                    dependency.scope.to_owned()
                }
                None => {
                    let mut path = self.dependencies_directory_path.to_owned();
                    path.push(format!("{}-{}", name, version));

                    let manifest = zinc_project::Manifest::try_from(&path)
                        .with_context(|| path.to_string_lossy().to_string())?;

                    let node_index = self.node_index(&manifest.project);
                    self.graph.add_edge(parent_node_index, node_index, ());
                    self.check_dependency(parent_node_index, node_index)?;

                    let dependencies = match manifest.dependencies {
                        Some(dependencies) => self.compile_list(node_index, &dependencies)?,
                        None => HashMap::new(),
                    };

                    let mut source_directory_path = path.clone();
                    source_directory_path.push(zinc_const::directory::SOURCE);
                    let source = Source::try_from_entry(&source_directory_path)?;
                    let scope = source.modularize(manifest.project.clone(), dependencies)?;

                    let dependency = Dependency::new(manifest.project, scope.clone(), node_index);
                    self.cache
                        .insert((name.to_owned(), version.to_owned()), dependency.clone());
                    dependency.scope
                }
            };

            compiled.insert(name.to_owned(), scope);
        }

        Ok(compiled)
    }

    ///
    /// Checks the dependencies for validity:
    ///
    /// 1. The must be no dependency cycles.
    /// 2. Only the following project type relations allowed:
    ///     - contract-contract
    ///     - contract-library
    ///     - circuit-library
    ///     - library-library
    ///
    fn check_dependency(
        &self,
        parent_node_index: petgraph::graph::NodeIndex,
        child_node_index: petgraph::graph::NodeIndex,
    ) -> anyhow::Result<()> {
        let parent = &self.graph[parent_node_index];
        let child = &self.graph[child_node_index];

        match (parent.r#type, child.r#type) {
            (zinc_project::ProjectType::Contract, zinc_project::ProjectType::Contract) => {}
            (zinc_project::ProjectType::Contract, zinc_project::ProjectType::Library) => {}
            (zinc_project::ProjectType::Circuit, zinc_project::ProjectType::Library) => {}
            (zinc_project::ProjectType::Library, zinc_project::ProjectType::Library) => {}
            (parent_type, child_type) => anyhow::bail!(Error::ProjectTypesRelationForbidden {
                parent: format!("{}-{}", parent.name, parent.version),
                parent_type: parent_type.to_string(),
                child: format!("{}-{}", child.name, child.version),
                child_type: child_type.to_string(),
            }),
        }

        if petgraph::algo::is_cyclic_directed(&self.graph) {
            anyhow::bail!(Error::DependencyCycle {
                parent: format!("{}-{}", parent.name, parent.version),
                child: format!("{}-{}", child.name, child.version),
            });
        }

        Ok(())
    }

    ///
    /// Inserts a node into the dependency graph and updates the node indexes hashmap, if the
    /// dependency has not been added to the graph yet.
    ///
    /// Returns the dependency's node index.
    ///
    fn node_index(
        &mut self,
        project: &zinc_project::ManifestProject,
    ) -> petgraph::graph::NodeIndex {
        match self
            .node_indexes
            .get(&(project.name.to_owned(), project.version.to_owned()))
            .copied()
        {
            Some(node_index) => node_index,
            None => {
                let node_index = self.graph.add_node(project.to_owned());
                self.node_indexes.insert(
                    (project.name.to_owned(), project.version.to_owned()),
                    node_index,
                );
                node_index
            }
        }
    }
}
