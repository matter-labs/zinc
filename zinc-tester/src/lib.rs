//!
//! The Zinc tester library.
//!

pub(crate) mod metadata;
pub(crate) mod directory;
pub(crate) mod file;
pub(crate) mod panic;
pub(crate) mod program;
pub(crate) mod runners;
pub(crate) mod summary;

pub use self::metadata::Metadata;
pub use self::directory::Directory;
pub use self::file::File;
pub use self::runners::evaluation::Runner as EvaluationRunner;
pub use self::runners::proof_check::Runner as ProofCheckRunner;
pub use self::runners::Runnable;
pub use self::summary::Summary;

pub static TESTS_DIRECTORY: &str = "zinc-tester/tests/";
pub static TEST_FILE_EXTENSION: &str = "zn";
pub static TEST_LINE_PREFIX: &str = "//#";

pub(crate) fn default_entry() -> String {
    zinc_compiler::FUNCTION_MAIN_IDENTIFIER.to_owned()
}
