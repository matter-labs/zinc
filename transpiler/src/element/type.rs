//!
//! Transpiler type element.
//!

use std::fmt;

#[derive(Clone)]
pub struct Element {
    pub inner: String,
}

impl Element {
    pub fn new(inner: String) -> Self {
        Self { inner }
    }
}

impl Into<String> for Element {
    fn into(self) -> String {
        self.inner
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}
