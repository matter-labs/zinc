//!
//! The Zinc compiler bundler error.
//!

use thiserror::Error;

///
/// The Zinc compiler bundler error.
///
#[derive(Debug, Error)]
pub enum Error {
    /// A dependency cycle between some projects has been detected.
    #[error("projects `{parent}` and `{child}` depend on each other")]
    DependencyCycle {
        /// The parent project identifier.
        parent: String,
        /// The child project identifier.
        child: String,
    },
    /// A dependency relation between such project types is forbidden.
    #[error("dependency relation between the {parent_type} `{parent}` and {child_type} `{child}` is forbidden")]
    ProjectTypesRelationForbidden {
        /// The parent project identifier.
        parent: String,
        /// The parent project type.
        parent_type: String,
        /// The child project identifier.
        child: String,
        /// The child project type.
        child_type: String,
    },
}
