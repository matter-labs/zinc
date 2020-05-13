//!
//! The semantic analyzer scope type item state.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::statement::Statement as GeneratorStatement;
use crate::semantic::element::r#type::Type as TypeElement;
use crate::semantic::scope::item::r#type::statement::Statement as TypeStatementVariant;
use crate::semantic::scope::Scope;

#[derive(Debug, Clone)]
pub enum State {
    /// Waiting to be resolved during the second pass
    Unresolved {
        inner: TypeStatementVariant,
        scope: Rc<RefCell<Scope>>,
    },
    /// Resolved element ready to be used from anywhere
    Resolved {
        inner: TypeElement,
        intermediate: Option<GeneratorStatement>,
    },
}

impl State {
    pub fn get_intermediate(&self) -> Vec<GeneratorStatement> {
        match self {
            Self::Resolved {
                inner,
                intermediate,
            } => match inner {
                TypeElement::Function(_) => match intermediate.to_owned().take() {
                    Some(intermediate) => vec![intermediate],
                    None => vec![],
                },
                TypeElement::Structure(ref inner) => {
                    Scope::get_intermediate(inner.scope.to_owned())
                }
                TypeElement::Enumeration(ref inner) => {
                    Scope::get_intermediate(inner.scope.to_owned())
                }
                TypeElement::Contract(ref inner) => Scope::get_intermediate(inner.scope.to_owned()),
                _ => vec![],
            },
            Self::Unresolved { .. } => vec![],
        }
    }
}
