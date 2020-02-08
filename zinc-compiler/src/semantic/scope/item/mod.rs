//!
//! The semantic analyzer scope item.
//!

pub mod r#static;
pub mod variable;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::Type;
use crate::semantic::scope::Scope;

use self::r#static::Static;
use self::variable::Variable;

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Variable(Variable),
    Constant(Constant),
    Static(Static),
    Type(Type),
    Module(Rc<RefCell<Scope>>),
}

impl Item {
    pub fn is_namespace(&self) -> bool {
        match self {
            Self::Variable(_) => false,
            Self::Constant(_) => false,
            Self::Static(_) => false,
            Self::Type(Type::Enumeration { .. }) => false,
            Self::Type(_) => true,
            Self::Module(_) => true,
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Variable(variable) => write!(f, "{:?}", variable),
            Self::Constant(constant) => write!(f, "{}", constant),
            Self::Static(r#static) => write!(f, "{:?}", r#static),
            Self::Type(r#type) => write!(f, "{}", r#type),
            Self::Module(_) => write!(f, "<module>"),
        }
    }
}
