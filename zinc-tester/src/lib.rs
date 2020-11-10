//!
//! The Zinc tester library.
//!

pub(crate) mod directory;
pub(crate) mod error;
pub(crate) mod file;
pub(crate) mod instance;
pub(crate) mod metadata;
pub(crate) mod runners;
pub(crate) mod summary;

pub use self::directory::Directory;
pub use self::file::File;
pub use self::metadata::Metadata;
pub use self::runners::evaluation::Runner as EvaluationRunner;
pub use self::runners::proof_check::Runner as ProofCheckRunner;
pub use self::runners::IRunnable;
pub use self::summary::Summary;
