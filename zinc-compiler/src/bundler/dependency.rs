//!
//! The Zinc compiler dependency.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::scope::Scope;

///
/// The dependency data.
///
#[derive(Debug, Clone)]
pub struct Dependency {
    /// The project description.
    pub project: zinc_project::ManifestProject,
    /// The semantic scope tree.
    pub scope: Rc<RefCell<Scope>>,
    /// The dependency node index in the graph.
    pub node_index: petgraph::graph::NodeIndex,
}

impl Dependency {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        project: zinc_project::ManifestProject,
        scope: Rc<RefCell<Scope>>,
        node_index: petgraph::graph::NodeIndex,
    ) -> Self {
        Self {
            project,
            scope,
            node_index,
        }
    }
}
