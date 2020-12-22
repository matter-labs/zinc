//!
//! The semantic analyzer scope module item.
//!

pub mod state;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use zinc_lexical::Keyword;
use zinc_lexical::Location;

use crate::generator::statement::Statement as GeneratorStatement;
use crate::semantic::analyzer::module::Analyzer as ModuleAnalyzer;
use crate::semantic::error::Error;
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
    /// The location where the module was declared. `None` for intrinsic modules.
    pub location: Option<Location>,
    /// The unique module ID, allocated upon declaration.
    pub item_id: usize,
    /// The module identifier.
    pub identifier: String,
    /// The definition state, which is either `declared` or `defined`.
    pub state: RefCell<Option<State>>,
}

impl Module {
    ///
    /// Initializes an application entry module scope.
    ///
    pub fn new_entry(
        module: Source,
        project: zinc_project::ManifestProject,
        dependencies: HashMap<String, Rc<RefCell<Scope>>>,
        is_dependency_entry: bool,
    ) -> Result<Rc<RefCell<ScopeItem>>, Error> {
        let scope = Scope::new_module(
            module.name().to_owned(),
            dependencies.clone(),
            Some(project),
            is_dependency_entry,
        )
        .wrap();

        let module = Self::new_declared(
            None,
            scope.clone(),
            module.name().to_owned(),
            module,
            scope.clone(),
            None,
            dependencies,
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
    #[allow(clippy::too_many_arguments)]
    pub fn new_declared(
        location: Option<Location>,
        scope: Rc<RefCell<Scope>>,
        identifier: String,
        module: Source,
        scope_crate: Rc<RefCell<Scope>>,
        scope_super: Option<Rc<RefCell<Scope>>>,
        dependencies: HashMap<String, Rc<RefCell<Scope>>>,
        is_entry: bool,
    ) -> Result<Self, Error> {
        let item_id = ITEM_INDEX.next(format!("module {}", identifier));

        let (module, modules) = match module {
            Source::File(file) => (file.tree, HashMap::new()),
            Source::Directory(directory) => (directory.entry.tree, directory.modules),
        };

        let (module, implementation_scopes) = ModuleAnalyzer::declare(
            scope.clone(),
            module,
            modules,
            scope_crate.clone(),
            dependencies,
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

        Self {
            location,
            item_id,
            identifier,
            state: RefCell::new(Some(State::Defined { scope })),
        }
    }

    ///
    /// Useful method to declare an intrinsic module without a `location`.
    ///
    pub fn new_built_in(identifier: String, scope: Rc<RefCell<Scope>>) -> Self {
        let item_id = ITEM_INDEX.next(format!("module {}", identifier));

        Self {
            location: None,
            item_id,
            identifier,
            state: RefCell::new(Some(State::Defined { scope })),
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
                self.state.replace(Some(State::Defined {
                    scope: scope.clone(),
                }));

                let crate_item = Scope::get_module_self_alias(scope_crate);
                let super_item = scope_super.map(Scope::get_module_self_alias);

                ModuleAnalyzer::define(
                    scope.clone(),
                    module,
                    implementation_scopes,
                    crate_item,
                    super_item,
                )?;

                Ok(scope)
            }
            Some(State::Defined { scope }) => {
                self.state.replace(Some(State::Defined {
                    scope: scope.clone(),
                }));

                Ok(scope)
            }
            None => Err(Error::ScopeReferenceLoop {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
            }),
        }
    }

    ///
    /// Returns the module scope regardless of whether it is declared or defined.
    ///
    pub fn scope(&self) -> Result<Rc<RefCell<Scope>>, Error> {
        match self.state.borrow().as_ref() {
            Some(state) => Ok(state.scope()),
            None => Err(Error::ScopeReferenceLoop {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
            }),
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}
