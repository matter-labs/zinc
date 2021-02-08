//!
//! The generator function role.
//!

///
/// The generator function role.
///
/// Describes special function traits, e.g. circuit entry or contract method.
///
#[derive(Debug, Clone)]
pub enum Role {
    /// The ordinar user-defined function.
    Ordinar,
    /// The circuit entry.
    CircuitEntry,
    /// A contract constructor.
    ContractConstuctor {
        /// The `project` section of the contract project manifest.
        project: zinc_project::ManifestProject,
        /// Whether the contract constructor is located in a dependency project.
        is_dependency: bool,
    },
    /// A contract method entry.
    ContractMethodEntry,
    /// A unit test.
    UnitTest,
}
