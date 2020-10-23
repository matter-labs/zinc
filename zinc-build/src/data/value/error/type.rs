//!
//! The Zinc VM template value error type.
//!

use failure::Fail;
use serde_json::Value as JsonValue;

use zinc_math::InferenceError;

///
/// The inner type error variant.
///
#[derive(Debug, Fail)]
pub enum Type {
    /// The primitive value does not match the expected primitive type.
    #[fail(display = "type mismatch: expected `{}`, found `{}`", expected, found)]
    TypeError {
        /// The expected type.
        expected: String,
        /// The found type.
        found: String,
    },

    /// The number could not be parsed successfully.
    #[fail(
        display = "failed to parse a number: expected a binary, octal, decimal, or hexadecimal string, found `{}`",
        _0
    )]
    InvalidNumberFormat(String),

    /// The number is too large for the type.
    #[fail(display = "value inference: {}", inner)]
    ValueOverflow { inner: InferenceError },

    /// The structure field is missing.
    #[fail(display = "value for field `{}` is missing", _0)]
    MissingField(String),

    /// The field could not be found in the structure type.
    #[fail(display = "unexpected field `{}`", _0)]
    UnexpectedField(String),

    /// The variant could not be found in the enumeration type.
    #[fail(display = "unexpected variant `{}`", _0)]
    UnexpectedVariant(String),

    /// The data size does not match the type size.
    #[fail(
        display = "expected a data structure of size {}, but found {} values",
        expected, found
    )]
    UnexpectedSize {
        /// The expected type.
        expected: usize,
        /// The found type.
        found: usize,
    },

    /// The map input is malformed.
    #[fail(
        display = "expected an array with `key` and `value` fields, found `{}`",
        _0
    )]
    InvalidMapFormat(String),
}

impl Type {
    ///
    /// A shortcut constructor.
    ///
    pub fn type_error(expected: String, found: JsonValue) -> Self {
        let found: String = match found {
            JsonValue::Null => "null".into(),
            JsonValue::Bool(value) => format!("boolean ({})", value),
            JsonValue::Number(value) => format!("number ({})", value),
            JsonValue::String(value) => format!("string (\"{}\")", value),
            JsonValue::Array(_) => "array".into(),
            JsonValue::Object(_) => "structure".into(),
        };

        Self::TypeError { expected, found }
    }
}
