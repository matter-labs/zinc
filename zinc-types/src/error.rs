//!
//! The template value error.
//!

use thiserror::Error;

///
/// The inner type error variant.
///
#[derive(Debug, Error)]
pub enum Error {
    /// The math error.
    #[error("{0}")]
    TypeMath(#[from] zinc_math::Error),

    /// The primitive value does not match the expected primitive type.
    #[error("type mismatch: expected `{expected}`, found `{found}`")]
    TypeError {
        /// The expected type.
        expected: String,
        /// The found type.
        found: String,
    },

    /// The number could not be parsed successfully.
    #[error("failed to parse a number: expected a binary, octal, decimal, or hexadecimal string, found `{0}`")]
    InvalidNumberFormat(String),

    /// The structure field is missing.
    #[error("value for field `{0}` is missing")]
    MissingField(String),

    /// The field could not be found in the structure type.
    #[error("unexpected field `{0}`")]
    UnexpectedField(String),

    /// The variant could not be found in the enumeration type.
    #[error("unexpected variant `{0}`")]
    UnexpectedVariant(String),

    /// The data size does not match the type size.
    #[error("expected a data structure of size {expected}, but found {found} values")]
    UnexpectedSize {
        /// The expected type.
        expected: usize,
        /// The found type.
        found: usize,
    },

    /// The map input is malformed.
    #[error("expected an array with `key` and `value` fields, found `{0}`")]
    InvalidMapFormat(String),
}

impl Error {
    ///
    /// A shortcut constructor.
    ///
    pub fn type_error(expected: String, found: serde_json::Value) -> Self {
        let found: String = match found {
            serde_json::Value::Null => "null".into(),
            serde_json::Value::Bool(value) => format!("boolean ({})", value),
            serde_json::Value::Number(value) => format!("number ({})", value),
            serde_json::Value::String(value) => format!("string (\"{}\")", value),
            serde_json::Value::Array(_) => "array".into(),
            serde_json::Value::Object(_) => "structure".into(),
        };

        Self::TypeError { expected, found }
    }
}
