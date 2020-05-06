//!
//! The semantic analyzer scope module item.
//!

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::lexical::token::location::Location;
use crate::semantic::scope::Scope;

///
/// The module item, declared using a `mod` statement.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub location: Option<Location>,
    pub scope: Rc<RefCell<Scope>>,
}

impl Module {
    pub fn new(location: Option<Location>, scope: Rc<RefCell<Scope>>) -> Self {
        Self { location, scope }
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<module>")
    }
}
