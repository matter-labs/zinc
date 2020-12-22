//!
//! The Zinc tester library.
//!

pub(crate) mod error;
pub(crate) mod one_file;
pub(crate) mod ordinar;
pub(crate) mod summary;

pub use self::error::Error;
pub use self::one_file::directory::Directory as OneFileTestsDirectory;
pub use self::one_file::runners::evaluation::Runner as EvaluationRunner;
pub use self::one_file::runners::IRunnable;
pub use self::ordinar::directory::Directory as OrdinarTestsDirectory;
pub use self::summary::Summary;

/// The one-file tests metadata line prefix.
pub static ONE_FILE_TEST_METADATA_PREFIX: &str = "//!";

/// The one-file tests directory.
pub static ONE_FILE_TESTS_DIRECTORY: &str = "zinc-tester/one-file/";

/// The ordinar project tests directory.
pub static ORDINAR_PROJECTS_DIRECTORY: &str = "zinc-tester/ordinar/";
