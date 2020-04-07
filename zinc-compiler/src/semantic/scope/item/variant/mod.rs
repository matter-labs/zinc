//!
//! The semantic analyzer scope item variant.
//!

pub mod variable;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::Type;
use crate::semantic::scope::Scope;

use self::variable::Variable;

#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    Variable(Variable),
    Constant(Constant),
    Type(Type),
    Module(Rc<RefCell<Scope>>),
}

impl Variant {
    pub fn is_namespace(&self) -> bool {
        match self {
            Self::Variable(_) => false,
            Self::Constant(_) => false,
            Self::Type(Type::Enumeration { .. }) => false,
            Self::Type(_) => true,
            Self::Module(_) => true,
        }
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Variable(inner) => write!(f, "{}", inner),
            Self::Constant(inner) => write!(f, "{}", inner),
            Self::Type(inner) => write!(f, "{}", inner),
            Self::Module(_) => write!(f, "<module>"),
        }
    }
}
