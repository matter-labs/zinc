//!
//! The Zinc constants.
//!

pub mod app_name;
pub mod base;
pub mod bitlength;
pub mod exit_code;
pub mod extensions;
pub mod file_names;
pub mod http;
pub mod limit;
pub mod mongodb;
pub mod panic;
pub mod source;
pub mod zargo;

pub use self::exit_code::unit_test::ExitCode as UnitTestExitCode;
