//!
//! The semantic analyzer scope module item.
//!

pub mod state;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::generator::statement::Statement as GeneratorStatement;
use crate::lexical::token::location::Location;
use crate::semantic::analyzer::module::Analyzer as ModuleAnalyzer;
use crate::semantic::error::Error;
use crate::semantic::scope::error::Error as ScopeError;
use crate::semantic::scope::item::index::INDEX as ITEM_INDEX;
use crate::semantic::scope::Scope;
use crate::source::module::Module as SourceModule;

use self::state::State;

///
/// The module item, declared using a `mod` statement.
///
#[derive(Debug, Clone)]
pub struct Module {
    pub location: Option<Location>,
    pub item_id: usize,
    pub identifier: String,
    pub state: RefCell<Option<State>>,
}

impl Module {
    ///
    /// Creates an unresolved module, which must be resolved during the second pass or when
    /// the item is referenced for the first time.
    ///
    /// Is used during module items hoisting.
    ///
    pub fn new_unresolved(
        location: Option<Location>,
        identifier: String,
        inner: SourceModule,
    ) -> Self {
        let item_id = ITEM_INDEX.next(format!("module {}", identifier));

        Self {
            location,
            item_id,
            identifier,
            state: RefCell::new(Some(State::Unresolved { inner })),
        }
    }

    ///
    /// Creates a resolved module, which is ready to be used from anywhere.
    ///
    /// Is used for items which are not hoisted.
    ///
    pub fn new_resolved(location: Location, identifier: String, scope: Rc<RefCell<Scope>>) -> Self {
        let item_id = ITEM_INDEX.next(format!("module {}", identifier));

        Self {
            location: Some(location),
            item_id,
            identifier,
            state: RefCell::new(Some(State::Resolved { inner: scope })),
        }
    }

    ///
    /// Useful method to declare a built-in module without a `location`.
    ///
    pub fn new_built_in(identifier: String, scope: Rc<RefCell<Scope>>) -> Self {
        let item_id = ITEM_INDEX.next(format!("module {}", identifier));

        Self {
            location: None,
            item_id,
            identifier,
            state: RefCell::new(Some(State::Resolved { inner: scope })),
        }
    }

    pub fn resolve(&self) -> Result<Rc<RefCell<Scope>>, Error> {
        let variant = self.state.borrow_mut().take();

        match variant {
            Some(State::Unresolved { inner }) => {
                let resolved = ModuleAnalyzer::analyze(inner)?;
                self.state.replace(Some(State::Resolved {
                    inner: resolved.clone(),
                }));

                Ok(resolved)
            }
            Some(State::Resolved { inner }) => {
                self.state.replace(Some(State::Resolved {
                    inner: inner.clone(),
                }));

                Ok(inner)
            }
            None => Err(Error::Scope(ScopeError::ReferenceLoop {
                location: self.location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
            })),
        }
    }

    pub fn is_resolved(&self) -> bool {
        match self.state.borrow().as_ref() {
            Some(State::Resolved { .. }) => true,
            _ => false,
        }
    }

    pub fn get_intermediate(&self) -> Vec<GeneratorStatement> {
        self.state
            .borrow()
            .as_ref()
            .map(|state| state.get_intermediate())
            .unwrap_or_default()
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}
