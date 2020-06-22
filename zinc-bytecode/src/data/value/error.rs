//!
//! The Zinc VM template value error.
//!

use std::fmt;

use failure::Fail;
use serde_json::Value as JsonValue;

#[derive(Debug, Fail)]
pub struct Error {
    path: Vec<String>,
    error: ErrorType,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if self.path.is_empty() {
            write!(f, "{}", self.error)
        } else {
            let mut path = self.path.clone();
            path.reverse();
            let p = path.as_slice().join(&String::from("."));
            write!(f, "{} at {}", self.error, p)
        }
    }
}

pub trait ErrorContext {
    fn in_struct(self, name: &str) -> Self;
    fn in_array(self, index: usize) -> Self;
}

///
/// Propagates error location.
///
impl<T> ErrorContext for Result<T, Error> {
    fn in_struct(self, name: &str) -> Self {
        self.map_err(|mut e| {
            e.path.push(name.into());
            e
        })
    }

    fn in_array(self, index: usize) -> Self {
        self.map_err(|mut e| {
            e.path.push(format!("[{}]", index));
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

#[derive(Debug, Fail)]
pub enum ErrorType {
    #[fail(display = "unexpected null value")]
    UnexpectedNull,

    #[fail(display = "type mismatch: expected {}, got {}", expected, actual)]
    TypeError { expected: String, actual: String },

    #[fail(
        display = "failed to parse number: expected decimal or hexadecimal string, got \"{}\"",
        _0
    )]
    InvalidNumberFormat(String),

    #[fail(display = "value for field \"{}\" is missing", _0)]
    MissingField(String),

    #[fail(display = "unexpected field \"{}\"", _0)]
    UnexpectedField(String),

    #[fail(
        display = "expected array/tuple of size {}, got {} elements",
        expected, actual
    )]
    UnexpectedSize { expected: usize, actual: usize },

    #[fail(display = "contract parsing forbidden")]
    ContractPassingForbidden,
}

impl ErrorType {
    pub fn type_error(expected: &str, actual: JsonValue) -> Self {
        let actual_string: String = match actual {
            JsonValue::Null => "null".into(),
            JsonValue::Bool(value) => format!("boolean ({})", value),
            JsonValue::Number(value) => format!("number ({})", value),
            JsonValue::String(value) => format!("string (\"{}\")", value),
            JsonValue::Array(_) => "array".into(),
            JsonValue::Object(_) => "structure".into(),
        };

        ErrorType::TypeError {
            expected: expected.into(),
            actual: actual_string,
        }
    }
}
