//!
//! The Zinc constants.
//!

pub mod app_name;
pub mod base;
pub mod bitlength;
pub mod crate_name;
pub mod directory;
pub mod exit_code;
pub mod extension;
pub mod file_name;
pub mod limit;
pub mod panic;
pub mod path;
pub mod size;
pub mod source;
pub mod tester;
pub mod zandbox;
pub mod zargo;

pub use self::exit_code::unit_test::ExitCode as UnitTestExitCode;
