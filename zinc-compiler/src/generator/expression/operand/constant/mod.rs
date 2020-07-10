//!
//! The generator expression constant operand.
//!

pub mod boolean;
pub mod integer;

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::state::State;
use crate::semantic::element::constant::Constant as SemanticConstant;

use self::boolean::Boolean;
use self::integer::Integer;

#[derive(Debug, Clone)]
pub enum Constant {
    Boolean(Boolean),
    Integer(Integer),
    Group(Vec<Self>),
}

impl Constant {
    pub fn try_from_semantic(constant: &SemanticConstant) -> Option<Self> {
        match constant {
            SemanticConstant::Boolean(inner) => Some(Self::Boolean(Boolean::from_semantic(inner))),
            SemanticConstant::Integer(inner) => Some(Self::Integer(Integer::from_semantic(inner))),
            SemanticConstant::Array(inner) => {
                let group: Vec<Self> = inner
                    .values
                    .iter()
                    .filter_map(Self::try_from_semantic)
                    .collect();
                if group.is_empty() {
                    None
                } else {
                    Some(Self::Group(group))
                }
            }
            SemanticConstant::Tuple(inner) => {
                let group: Vec<Self> = inner
                    .values
                    .iter()
                    .filter_map(Self::try_from_semantic)
                    .collect();
                if group.is_empty() {
                    None
                } else {
                    Some(Self::Group(group))
                }
            }
            SemanticConstant::Structure(inner) => {
                let group: Vec<Self> = inner
                    .values
                    .iter()
                    .filter_map(|(_name, value)| Self::try_from_semantic(value))
                    .collect();
                if group.is_empty() {
                    None
                } else {
                    Some(Self::Group(group))
                }
            }
            _ => None,
        }
    }

    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<State>>) {
        match self {
            Self::Boolean(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Integer(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Group(inner) => {
                for constant in inner.into_iter() {
                    constant.write_all_to_bytecode(bytecode.clone());
                }
            }
        }
    }
}
