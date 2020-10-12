//!
//! The semantic analyzer scope module item state.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_syntax::Module as SyntaxModule;

use crate::generator::statement::Statement as GeneratorStatement;
use crate::semantic::scope::Scope;

///
/// The definition state, which is either `declared` or `defined`.
///
#[derive(Debug, Clone)]
pub enum State {
    /// Waiting to be defined during the second pass.
    Declared {
        /// The module scope, which contains all the items declared within.
        scope: Rc<RefCell<Scope>>,
        /// The module syntax representation.
        module: SyntaxModule,
        /// The scopes of the `impl` statement, which must be defined for their types during module definition.
        implementation_scopes: Vec<Rc<RefCell<Scope>>>,
        /// The reference to the application root scope.
        scope_crate: Rc<RefCell<Scope>>,
        /// The reference to the parent scope, if the current scope is not the root one.
        scope_super: Option<Rc<RefCell<Scope>>>,
    },
    /// Defined element ready to be used from anywhere.
    Defined {
        /// The module scope, which contains all the items defined within.
        scope: Rc<RefCell<Scope>>,
    },
}

impl State {
    ///
    /// Returns the module scope regardless of whether it is declared or defined.
    ///
    pub fn scope(&self) -> Rc<RefCell<Scope>> {
        match self {
            Self::Declared { scope, .. } => scope.to_owned(),
            Self::Defined { scope } => scope.to_owned(),
        }
    }

    ///
    /// Extracts the intermediate representation from the element.
    ///
    pub fn get_intermediate(&self) -> Vec<GeneratorStatement> {
        match self {
            Self::Defined { scope } => scope.borrow().get_intermediate(),
            _ => vec![],
        }
    }
}
