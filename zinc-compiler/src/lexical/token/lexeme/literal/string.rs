//!
//! The lexical token string literal lexeme.
//!

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct String {
    pub inner: ::std::string::String,
}

impl String {
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}
