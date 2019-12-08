//!
//! The semantic analyzer scope item.
//!

mod r#static;
mod variable;

pub use self::r#static::Static;
pub use self::variable::Variable;

use std::fmt;
use std::rc::Rc;

use crate::semantic::Constant;
use crate::semantic::Scope;
use crate::semantic::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Variable(Variable),
    Constant(Constant),
    Static(Static),
    Type(Type),
    Module(Rc<Scope>),
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Variable(variable) => write!(f, "{:?}", variable),
            Self::Constant(constant) => write!(f, "{:?}", constant),
            Self::Static(r#static) => write!(f, "{:?}", r#static),
            Self::Type(r#type) => write!(f, "{}", r#type),
            Self::Module(scope) => write!(f, "{:?}", scope),
        }
    }
}
