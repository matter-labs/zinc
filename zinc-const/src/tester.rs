//!
//! The Zinc integration tester constants.
//!

/// The default tests directory in the Zinc framework repository.
pub static DEFAULT_DIRECTORY: &str = "zinc-tester/tests/";

/// The integration test metadata line prefix.
pub static METADATA_LINE_PREFIX: &str = "//!";

///
/// The default application entry name constructor for the `serde` default macro.
///
pub fn default_method() -> String {
    crate::source::FUNCTION_MAIN_IDENTIFIER.to_owned()
}
