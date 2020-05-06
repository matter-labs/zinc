//!
//! The semantic analyzer scope item.
//!

pub mod constant;
pub mod module;
pub mod r#type;
pub mod variable;

use std::fmt;

use crate::lexical::token::location::Location;

use self::constant::Constant;
use self::module::Module;
use self::r#type::Type;
use self::variable::Variable;

///
/// An item declared within a scope.
///
/// Items are variables, constants, types, modules, etc.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Variable(Variable),
    Constant(Constant),
    Type(Type),
    Module(Module),
}

impl Item {
    pub fn location(&self) -> Option<Location> {
        match self {
            Self::Variable(inner) => Some(inner.location),
            Self::Constant(inner) => Some(inner.location),
            Self::Type(inner) => inner.location,
            Self::Module(inner) => inner.location,
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Variable(inner) => write!(f, "{}", inner),
            Self::Constant(inner) => write!(f, "{}", inner),
            Self::Type(inner) => write!(f, "{}", inner),
            Self::Module(inner) => write!(f, "{}", inner),
        }
    }
}
