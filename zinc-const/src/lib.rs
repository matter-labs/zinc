//!
//! The Zinc constants.
//!

pub mod app_name;
pub mod base;
pub mod bitlength;
pub mod directory;
pub mod exit_code;
pub mod extension;
pub mod file_name;
pub mod http;
pub mod limit;
pub mod panic;
pub mod path;
pub mod postgresql;
pub mod source;
pub mod zargo;

pub use self::exit_code::unit_test::ExitCode as UnitTestExitCode;
