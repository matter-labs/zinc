//!
//! The semantic analyzer scope type.
//!

///
/// The semantic analyzer scope type.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// The main file module of the application entry.
    Entry {
        /// The `project` section of the project manifest.
        project: zinc_project::ManifestProject,
        /// Whether the entry is of an application dependency.
        is_dependency: bool,
    },
    /// The non-entry application module file.
    Module {
        /// Whether the module is of an application dependency.
        is_dependency: bool,
    },
    /// The module with intrinsic items like the standard library functions.
    Intrinsic,
    /// The smart contract namespace, where its fields, methods, and associated items are declared.
    Contract,
    /// The structure namespace, where its fields, methods, and associated items are declared.
    Structure,
    /// The enumeration namespace, where its fields, methods, and associated items are declared.
    Enumeration,
    /// The function block.
    Function,
    /// The conditional block.
    Conditional,
    /// The for-loop block.
    Loop,
    /// The ordinar expression block.
    Block,
}

impl Type {
    ///
    /// Checks if the scope is a manually developed module, that is, written in Zinc.
    ///
    pub fn is_module(&self) -> bool {
        match self {
            Self::Entry { .. } => true,
            Self::Module { .. } => true,
            _ => false,
        }
    }

    ///
    /// Checks if the scope is a type implementation, e.g. contract, structure, or enumeration.
    ///
    pub fn is_implementation(&self) -> bool {
        match self {
            Self::Contract => true,
            Self::Structure => true,
            Self::Enumeration => true,
            _ => false,
        }
    }
}
