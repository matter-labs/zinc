//!
//! The generator expression constant operand.
//!

pub mod boolean;
pub mod integer;

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::zinc_vm::State as ZincVMState;
use crate::generator::IBytecodeWritable;
use crate::semantic::element::constant::Constant as SemanticConstant;

use self::boolean::Boolean;
use self::integer::Integer;

///
/// The constant operand, which is pushed directly into the bytecode.
///
#[derive(Debug, Clone)]
pub enum Constant {
    /// The boolean constant.
    Boolean(Boolean),
    /// The integer constant.
    Integer(Integer),
    /// The constant group, which is created from an array, tuple, structure, etc.
    Group(Vec<Self>),
}

impl Constant {
    ///
    /// Tries to create a constant from the semantic one.
    ///
    /// If the constant only exists at compile time, `None` is returned.
    ///
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
}

impl IBytecodeWritable for Constant {
    fn write_to_zinc_vm(self, state: Rc<RefCell<ZincVMState>>) {
        match self {
            Self::Boolean(inner) => inner.write_to_zinc_vm(state),
            Self::Integer(inner) => inner.write_to_zinc_vm(state),
            Self::Group(inner) => {
                for constant in inner.into_iter() {
                    constant.write_to_zinc_vm(state.clone());
                }
            }
        }
    }
}
