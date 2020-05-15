//!
//! The semantic analyzer scope item.
//!

pub mod constant;
pub mod index;
pub mod module;
pub mod r#type;
pub mod variable;

use std::fmt;

use crate::generator::statement::Statement as GeneratorStatement;
use crate::lexical::token::location::Location;
use crate::semantic::error::Error;

use self::constant::Constant;
use self::module::Module;
use self::r#type::Type;
use self::variable::Variable;

///
/// An item declared within a scope.
///
/// Items are variables, constants, types, modules, etc.
///
#[derive(Debug, Clone)]
pub enum Item {
    Variable(Variable),
    Constant(Constant),
    Type(Type),
    Module(Module),
}

impl Item {
    pub fn resolve(&self) -> Result<(), Error> {
        match self {
            Self::Variable(_) => {}
            Self::Constant(inner) => {
                inner.resolve()?;
            }
            Self::Type(inner) => {
                inner.resolve()?;
            }
            Self::Module(inner) => {
                inner.resolve()?;
            }
        }

        Ok(())
    }

    pub fn is_resolved(&self) -> bool {
        match self {
            Self::Constant(inner) => inner.is_resolved(),
            _ => true,
        }
    }

    pub fn location(&self) -> Option<Location> {
        match self {
            Self::Variable(inner) => Some(inner.location),
            Self::Constant(inner) => Some(inner.location),
            Self::Type(inner) => inner.location,
            Self::Module(inner) => inner.location,
        }
    }

    pub fn item_index_id(&self) -> usize {
        match self {
            Self::Variable(inner) => inner.item_id,
            Self::Constant(inner) => inner.item_id,
            Self::Type(inner) => inner.item_id,
            Self::Module(inner) => inner.item_id,
        }
    }

    pub fn get_intermediate(&self) -> Vec<GeneratorStatement> {
        match self {
            Self::Variable(_) => vec![],
            Self::Constant(_) => vec![],
            Self::Type(inner) => inner.get_intermediate(),
            Self::Module(inner) => inner.get_intermediate(),
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Variable(inner) => write!(f, "variable {}", inner),
            Self::Constant(inner) => write!(f, "constant {}", inner),
            Self::Type(inner) => write!(f, "type {}", inner),
            Self::Module(inner) => write!(f, "module {}", inner),
        }
    }
}
