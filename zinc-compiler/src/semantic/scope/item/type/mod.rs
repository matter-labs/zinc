//!
//! The semantic analyzer scope type item.
//!

pub mod index;

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type as TypeElement;

use self::index::HARD as INDEX_HARD;

///
/// The type item, declared using a `type`, `struct`, `enum`, or another statement.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub location: Option<Location>,
    pub index_hard_id: usize,
    pub inner: TypeElement,
}

impl Type {
    pub fn new(location: Option<Location>, inner: TypeElement) -> Self {
        let index_hard_id = INDEX_HARD.next(inner.to_string());

        Self {
            location,
            index_hard_id,
            inner,
        }
    }

    pub fn into_inner(self) -> TypeElement {
        self.inner
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}
