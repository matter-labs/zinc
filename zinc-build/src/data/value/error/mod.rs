//!
//! The Zinc VM template value error.
//!

pub mod context;
pub mod r#type;

use std::fmt;

use failure::Fail;

use self::context::IContext as IErrorContext;
use self::r#type::Type as ErrorType;

///
/// The template value error.
///
#[derive(Debug, Fail)]
pub struct Error {
    /// The path to the invalid value.
    path: Vec<String>,
    /// The error variant.
    error: ErrorType,
}

impl<T> IErrorContext for Result<T, Error> {
    ///
    /// Propagates the error location in an array.
    ///
    fn push_array(self, index: usize) -> Self {
        self.map_err(|mut e| {
            e.path.push(format!("[{}]", index));
            e
        })
    }

    ///
    /// Propagates the error location in a structure.
    ///
    fn push_structure(self, name: &str) -> Self {
        self.map_err(|mut e| {
            e.path.push(name.into());
            e
        })
    }
}

impl From<ErrorType> for Error {
    fn from(error: ErrorType) -> Self {
        Self {
            path: Vec::new(),
            error,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if self.path.is_empty() {
            write!(f, "{}", self.error)
        } else {
            write!(
                f,
                "{} at {}",
                self.error,
                self.path
                    .clone()
                    .into_iter()
                    .rev()
                    .collect::<Vec<String>>()
                    .join(".")
            )
        }
    }
}
