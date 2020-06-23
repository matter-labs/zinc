//!
//! The lexical token string literal lexeme.
//!

use std::fmt;

///
/// The lexical string literal.
///
#[derive(Debug, Clone, PartialEq)]
pub struct String {
    /// The inner string contents.
    pub inner: ::std::string::String,
}

impl String {
    ///
    /// Creates a string literal value.
    ///
    pub fn new(inner: ::std::string::String) -> Self {
        Self { inner }
    }
}

impl Into<::std::string::String> for String {
    fn into(self) -> ::std::string::String {
        self.inner
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}
