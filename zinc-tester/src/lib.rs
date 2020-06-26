//!
//! The Zinc tester library.
//!

pub(crate) mod directory;
pub(crate) mod file;
pub(crate) mod metadata;
pub(crate) mod panic;
pub(crate) mod program;
pub(crate) mod runners;
pub(crate) mod summary;

pub use self::directory::Directory;
pub use self::file::File;
pub use self::metadata::Metadata;
pub use self::runners::evaluation::Runner as EvaluationRunner;
pub use self::runners::proof_check::Runner as ProofCheckRunner;
pub use self::runners::IRunnable;
pub use self::summary::Summary;

/// The default tests directory in the Zinc framework repository.
pub static TEST_DEFAULT_DIRECTORY: &str = "zinc-tester/tests/";
/// The integration test metadata line prefix.
pub static TEST_LINE_PREFIX: &str = "//#";

///
/// The default application entry name constructor for the `serde` default macro.
///
pub(crate) fn default_entry() -> String {
    zinc_const::source::FUNCTION_MAIN_IDENTIFIER.to_owned()
}
