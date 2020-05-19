//!
//! The semantic analyzer scope module item.
//!

pub mod state;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::generator::statement::Statement as GeneratorStatement;
use crate::lexical::token::lexeme::keyword::Keyword;
use crate::lexical::token::location::Location;
use crate::semantic::analyzer::module::Analyzer as ModuleAnalyzer;
use crate::semantic::error::Error;
use crate::semantic::scope::error::Error as ScopeError;
use crate::semantic::scope::item::index::INDEX as ITEM_INDEX;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::Scope;
use crate::source::Source;

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
    /// Initializes an application entry module scope.
    ///
    pub fn new_entry(module: Source) -> Result<Rc<RefCell<ScopeItem>>, Error> {
        let scope = Scope::new_global(module.name().to_owned()).wrap();

        let module = Self::new_declared(
            None,
            scope.clone(),
            module.name().to_owned(),
            module,
            scope.clone(),
            None,
            true,
        )?;
        let item = ScopeItem::Module(module).wrap();

        scope
            .borrow()
            .items
            .borrow_mut()
            .insert(Keyword::SelfLowercase.to_string(), item.clone());
        scope
            .borrow()
            .items
            .borrow_mut()
            .insert(Keyword::Crate.to_string(), item.clone());

        Ok(item)
    }

    ///
    /// Creates an declared module, which must be defined during the second pass or when
    /// the item is referenced for the first time.
    ///
    /// Is used during module items hoisting.
    ///
    pub fn new_declared(
        location: Option<Location>,
        scope: Rc<RefCell<Scope>>,
        identifier: String,
        module: Source,
        scope_crate: Rc<RefCell<Scope>>,
        scope_super: Option<Rc<RefCell<Scope>>>,
        is_entry: bool,
    ) -> Result<Self, Error> {
        let item_id = ITEM_INDEX.next(format!("module {}", identifier));
        log::trace!("Declared module with ID {}", item_id);

        let (module, dependencies) = match module {
            Source::File(file) => (file.tree, HashMap::new()),
            Source::Directory(directory) => (directory.entry.tree, directory.modules),
        };

        let (module, implementation_scopes) = ModuleAnalyzer::declare(
            scope.clone(),
            module,
            dependencies,
            scope_crate.clone(),
            is_entry,
        )?;

        Ok(Self {
            location,
            item_id,
            identifier,
            state: RefCell::new(Some(State::Declared {
                scope,
                module,
                implementation_scopes,
                scope_crate,
                scope_super,
            })),
        })
    }

    ///
    /// Creates a defined module, which is ready to be used from anywhere.
    ///
    /// Is used for items which are not hoisted.
    ///
    pub fn new_defined(
        location: Option<Location>,
        identifier: String,
        scope: Rc<RefCell<Scope>>,
        is_alias: bool,
    ) -> Self {
        let title = format!("{}{}", identifier, if is_alias { " (alias)" } else { "" });
        let item_id = ITEM_INDEX.next(format!("module {}", title));
        log::trace!("Defined module with ID {}", item_id);

        Self {
            location,
            item_id,
            identifier,
            state: RefCell::new(Some(State::Defined { inner: scope })),
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
            state: RefCell::new(Some(State::Defined { inner: scope })),
        }
    }

    ///
    /// Defines the declared module.
    ///
    /// The method is able to detect reference loops. It happens naturally when the method
    /// is reentered before the item being defined is put back into `variant`, which means that
    /// the item is taken twice during its resolution process.
    ///
    pub fn define(&self) -> Result<Rc<RefCell<Scope>>, Error> {
        let variant = self.state.borrow_mut().take();

        match variant {
            Some(State::Declared {
                scope,
                module,
                implementation_scopes,
                scope_crate,
                scope_super,
            }) => {
                log::trace!("Defining module with ID {}", self.item_id);

                self.state.replace(Some(State::Defined {
                    inner: scope.clone(),
                }));

                let crate_item = scope_crate
                    .borrow()
                    .items
                    .borrow()
                    .get(&Keyword::SelfLowercase.to_string())
                    .cloned()
                    .unwrap();
                let super_item = scope_super.map(|scope| {
                    scope
                        .borrow()
                        .items
                        .borrow()
                        .get(&Keyword::SelfLowercase.to_string())
                        .cloned()
                        .unwrap()
                });

                ModuleAnalyzer::define(
                    scope.clone(),
                    module,
                    implementation_scopes,
                    crate_item,
                    super_item,
                )?;

                Ok(scope)
            }
            Some(State::Defined { inner }) => {
                self.state.replace(Some(State::Defined {
                    inner: inner.clone(),
                }));

                Ok(inner)
            }
            None => Err(Error::Scope(ScopeError::ReferenceLoop {
                location: self.location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
            })),
        }
    }

    ///
    /// Returns the module scope regardless of whether it is declared or defined.
    ///
    pub fn scope(&self) -> Result<Rc<RefCell<Scope>>, Error> {
        match self.state.borrow().as_ref() {
            Some(state) => Ok(state.scope()),
            None => Err(Error::Scope(ScopeError::ReferenceLoop {
                location: self.location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
            })),
        }
    }

    ///
    /// Extracts the intermediate representation from the element.
    ///
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
