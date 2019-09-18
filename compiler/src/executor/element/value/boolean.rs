//!
//! The interpreter boolean value.
//!

use std::fmt;

use serde_derive::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Boolean {
    value: bool,
}

impl Default for Boolean {
    fn default() -> Self {
        Self { value: false }
    }
}

impl Boolean {
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    pub fn new_false() -> Self {
        Self { value: false }
    }

    pub fn new_true() -> Self {
        Self { value: true }
    }

    pub fn is_false(&self) -> bool {
        !self.value
    }

    pub fn is_true(&self) -> bool {
        self.value
    }

    pub fn not(&self) -> Self {
        Self::new(!self.value)
    }

    pub fn or(&self, other: &Self) -> Self {
        Self::new(self.value || other.value)
    }

    pub fn xor(&self, other: &Self) -> Self {
        Self::new((self.value && !other.value) || (!self.value && other.value))
    }

    pub fn and(&self, other: &Self) -> Self {
        Self::new(self.value && other.value)
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
