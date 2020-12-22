//!
//! The semantic analyzer scope type item state.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::statement::Statement as GeneratorStatement;
use crate::semantic::element::r#type::Type as TypeElement;
use crate::semantic::scope::item::r#type::statement::Statement as TypeStatementVariant;
use crate::semantic::scope::Scope;

///
/// The definition state, which is either `declared` or `defined`.
///
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum State {
    /// Waiting to be defined during the second pass.
    Declared {
        /// The statement syntax representation.
        inner: TypeStatementVariant,
        /// The scope, where the type is declared and must be defined later.
        scope: Rc<RefCell<Scope>>,
    },
    /// Defined element ready to be used from anywhere.
    Defined {
        /// The semantic type element.
        inner: TypeElement,
        /// There bytecode generator IR representation, if exists.
        intermediate: Option<GeneratorStatement>,
    },
}

impl State {
    ///
    /// Extracts the intermediate representation from the element.
    ///
    pub fn get_intermediate(&self) -> Vec<GeneratorStatement> {
        match self {
            Self::Defined {
                inner,
                intermediate,
            } => match inner {
                TypeElement::Function(_) => match intermediate.to_owned().take() {
                    Some(intermediate) => vec![intermediate],
                    None => vec![],
                },
                TypeElement::Structure(ref inner) => inner.scope.borrow().get_intermediate(),
                TypeElement::Enumeration(ref inner) => inner.scope.borrow().get_intermediate(),
                TypeElement::Contract(ref inner) => {
                    let mut intermediate = match intermediate.to_owned().take() {
                        Some(intermediate) => vec![intermediate],
                        None => vec![],
                    };
                    intermediate.extend(inner.scope.borrow().get_intermediate());
                    intermediate
                }
                _ => vec![],
            },
            Self::Declared { .. } => vec![],
        }
    }
}
