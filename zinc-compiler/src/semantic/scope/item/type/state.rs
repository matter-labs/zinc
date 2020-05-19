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
    /// Waiting to be defined during the second pass
    Declared {
        inner: TypeStatementVariant,
        scope: Rc<RefCell<Scope>>,
    },
    /// Defined element ready to be used from anywhere
    Defined {
        inner: TypeElement,
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
                TypeElement::Contract(ref inner) => inner.scope.borrow().get_intermediate(),
                _ => vec![],
            },
            Self::Declared { .. } => vec![],
        }
    }
}
