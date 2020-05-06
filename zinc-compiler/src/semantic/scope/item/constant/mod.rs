//!
//! The semantic analyzer scope constant item.
//!

pub mod index;

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::constant::Constant as ConstantElement;

use self::index::HARD as INDEX_HARD;

///
/// The constant item, declared using a `const` statement.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Constant {
    pub location: Location,
    pub index_hard_id: usize,
    pub inner: ConstantElement,
}

impl Constant {
    pub fn new(location: Location, inner: ConstantElement) -> Self {
        let index_hard_id = INDEX_HARD.next(inner.to_string());

        Self {
            location,
            index_hard_id,
            inner,
        }
    }

    pub fn into_inner(self) -> ConstantElement {
        self.inner
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}
