//!
//! The semantic analyzer scope module item state.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::statement::Statement as GeneratorStatement;
use crate::semantic::scope::Scope;
use crate::syntax::tree::module::Module as SyntaxModule;

#[derive(Debug, Clone)]
pub enum State {
    /// Waiting to be defined during the second pass
    Declared {
        scope: Rc<RefCell<Scope>>,
        module: SyntaxModule,
        implementation_scopes: Vec<Rc<RefCell<Scope>>>,
        scope_crate: Rc<RefCell<Scope>>,
        scope_super: Option<Rc<RefCell<Scope>>>,
    },
    /// Defined element ready to be used from anywhere
    Defined { inner: Rc<RefCell<Scope>> },
}

impl State {
    pub fn scope(&self) -> Rc<RefCell<Scope>> {
        match self {
            Self::Declared { scope, .. } => scope.to_owned(),
            Self::Defined { inner } => inner.to_owned(),
        }
    }

    ///
    /// Extracts the intermediate representation from the element.
    ///
    pub fn get_intermediate(&self) -> Vec<GeneratorStatement> {
        match self {
            Self::Defined { inner } => inner.borrow().get_intermediate(),
            _ => vec![],
        }
    }
}
