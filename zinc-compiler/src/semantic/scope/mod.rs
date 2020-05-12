//!
//! The semantic analyzer scope.
//!

mod tests;

pub mod builtin;
pub mod error;
pub mod item;
pub mod stack;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str;

use crate::generator::statement::Statement as GeneratorStatement;
use crate::lexical::token::lexeme::keyword::Keyword;
use crate::lexical::token::location::Location;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::path::Path;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;
use crate::source::module::Module as SourceModule;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::statement::contract::Statement as ContractStatement;
use crate::syntax::tree::statement::r#const::Statement as ConstStatement;

use self::builtin::BuiltInScope;
use self::error::Error;
use self::item::constant::Constant as ConstantItem;
use self::item::module::Module as ModuleItem;
use self::item::r#type::statement::Statement as TypeStatementVariant;
use self::item::r#type::Type as TypeItem;
use self::item::variable::Variable as VariableItem;
use self::item::Item;

///
/// A scope consists of a hashmap of the declared items and a reference to its parent.
/// The global scope has no parent.
/// Modules are connected to the program scope hierarchy horizontally, being stored as module items.
///
#[derive(Debug, Default, Clone)]
pub struct Scope {
    parent: Option<Rc<RefCell<Self>>>,
    items: HashMap<String, Item>,
}

impl Scope {
    const ITEMS_INITIAL_CAPACITY: usize = 1024;

    ///
    /// Initializes a scope with an explicit optional parent.
    ///
    /// Beware that if you omit the `parent`, built-in functions and `std` will not be available
    /// throughout the scope stack. To create a scope with such items available, use `new_global`.
    ///
    pub fn new(parent: Option<Rc<RefCell<Self>>>) -> Self {
        Self {
            parent,
            items: HashMap::with_capacity(Self::ITEMS_INITIAL_CAPACITY),
        }
    }

    ///
    /// Initializes a global scope without the built-in one as its parent.
    ///
    pub fn new_global() -> Self {
        Self {
            parent: Some(Rc::new(RefCell::new(BuiltInScope::initialize()))),
            items: HashMap::with_capacity(Self::ITEMS_INITIAL_CAPACITY),
        }
    }

    ///
    /// Wraps the scope into `Rc<RefCell<_>>` simplifying most of initializations.
    ///
    pub fn wrap(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    ///
    /// Internally resolves all the items in the order they have been declared.
    ///
    pub fn resolve(&self) -> Result<(), SemanticError> {
        let mut items: Vec<&Item> = self.items.values().collect();
        items.sort_by_key(|item| item.item_index_id());
        for item in items.into_iter() {
            item.resolve()?;
        }

        Ok(())
    }

    ///
    /// Defines an item of arbitrary type.
    ///
    pub fn define_item(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        item: Item,
    ) -> Result<(), Error> {
        if let Ok(item) = Self::resolve_item(scope.clone(), &identifier, true) {
            return Err(Error::ItemRedeclared {
                location: identifier.location,
                name: identifier.name,
                reference: item.location(),
            });
        }

        scope.borrow_mut().items.insert(identifier.name, item);

        Ok(())
    }

    ///
    /// Defines a variable, which is normally a `let` binding or a function actual parameter.
    ///
    pub fn define_variable(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        is_mutable: bool,
        r#type: Type,
    ) -> Result<(), Error> {
        if let Ok(item) = Self::resolve_item(scope.clone(), &identifier, true) {
            return Err(Error::ItemRedeclared {
                location: identifier.location,
                name: identifier.name,
                reference: item.location(),
            });
        }

        scope.borrow_mut().items.insert(
            identifier.name.clone(),
            Item::Variable(VariableItem::new(
                identifier.location,
                is_mutable,
                identifier.name,
                r#type,
            )),
        );

        Ok(())
    }

    ///
    /// Declares a constant, saving the `const` statement to define itself later during the second
    /// pass or referencing for the first time.
    ///
    pub fn declare_constant(
        scope: Rc<RefCell<Scope>>,
        statement: ConstStatement,
    ) -> Result<(), Error> {
        if let Ok(item) = Self::resolve_item(scope.clone(), &statement.identifier, true) {
            return Err(Error::ItemRedeclared {
                location: statement.location,
                name: statement.identifier.name,
                reference: item.location(),
            });
        }

        scope.borrow_mut().items.insert(
            statement.identifier.name.clone(),
            Item::Constant(ConstantItem::new_unresolved(
                statement.identifier.location,
                statement,
                scope.clone(),
            )),
        );

        Ok(())
    }

    ///
    /// Defines a constant, which has been instantly evaluated.
    ///
    pub fn define_constant(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        constant: Constant,
    ) -> Result<(), Error> {
        if let Ok(item) = Self::resolve_item(scope.clone(), &identifier, true) {
            return Err(Error::ItemRedeclared {
                location: identifier.location,
                name: identifier.name,
                reference: item.location(),
            });
        }

        scope.borrow_mut().items.insert(
            identifier.name,
            Item::Constant(ConstantItem::new_resolved(identifier.location, constant)),
        );

        Ok(())
    }

    ///
    /// Declares a type, saving the `type`, `struct`, `enum`, `contract` or another statement to
    /// define itself later during the second pass or referencing for the first time.
    ///
    pub fn declare_type(
        scope: Rc<RefCell<Scope>>,
        statement: TypeStatementVariant,
    ) -> Result<(), Error> {
        if let Ok(item) = Self::resolve_item(scope.clone(), &statement.identifier(), true) {
            return Err(Error::ItemRedeclared {
                location: statement.location(),
                name: statement.identifier().name.to_owned(),
                reference: item.location(),
            });
        }

        scope.borrow_mut().items.insert(
            statement.identifier().name.clone(),
            Item::Type(TypeItem::new_unresolved(
                Some(statement.location()),
                statement,
                scope.clone(),
            )),
        );

        Ok(())
    }

    ///
    /// Defines a type, which has been instantly evaluated.
    ///
    pub fn define_type(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        r#type: Type,
        intermediate: Option<GeneratorStatement>,
    ) -> Result<(), Error> {
        if let Ok(item) = Self::resolve_item(scope.clone(), &identifier, true) {
            return Err(Error::ItemRedeclared {
                location: r#type.location().unwrap_or(identifier.location),
                name: identifier.name,
                reference: item.location(),
            });
        }

        scope.borrow_mut().items.insert(
            identifier.name,
            Item::Type(TypeItem::new_resolved(
                Some(identifier.location),
                r#type,
                false,
                intermediate,
            )),
        );

        Ok(())
    }

    ///
    /// Defines a `contract` type, also checks whether it is the only contract in the scope.
    ///
    pub fn declare_contract(
        scope: Rc<RefCell<Scope>>,
        statement: ContractStatement,
    ) -> Result<(), Error> {
        if let Some(location) = scope.borrow().get_contract_location() {
            return Err(Error::ContractRedeclared {
                location: statement.location,
                reference: location,
            });
        }

        Self::declare_type(scope, TypeStatementVariant::Contract(statement))
    }

    ///
    /// Declares a modules, saving the its syntax tree to define itself later during the second
    /// pass or referencing for the first time.
    ///
    pub fn declare_module(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        module: SourceModule,
    ) -> Result<(), Error> {
        if let Ok(item) = Self::resolve_item(scope.clone(), &identifier, true) {
            return Err(Error::ItemRedeclared {
                location: identifier.location,
                name: identifier.name,
                reference: item.location(),
            });
        }

        scope.borrow_mut().items.insert(
            identifier.name.clone(),
            Item::Module(ModuleItem::new_unresolved(
                Some(identifier.location),
                identifier.name,
                module,
            )),
        );

        Ok(())
    }

    ///
    /// Defines a module, which is normally a `mod` binding.
    ///
    pub fn define_module(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        module: Rc<RefCell<Scope>>,
    ) -> Result<(), Error> {
        if let Ok(item) = Self::resolve_item(scope.clone(), &identifier, true) {
            return Err(Error::ItemRedeclared {
                location: identifier.location,
                name: identifier.name,
                reference: item.location(),
            });
        }

        scope.borrow_mut().items.insert(
            identifier.name.clone(),
            Item::Module(ModuleItem::new_resolved(
                identifier.location,
                identifier.name,
                module,
            )),
        );

        Ok(())
    }

    ///
    /// Defines the `Self` alias within a type implementation.
    ///
    /// Since `Self` is the reserved keyword, it is not being checked for being already declared.
    ///
    pub fn define_self(&mut self, r#type: Type) {
        self.items.insert(
            Keyword::SelfUppercase.to_string(),
            Item::Type(TypeItem::new_resolved(
                r#type.location(),
                r#type,
                true,
                None,
            )),
        );
    }

    ///
    /// Gets an item at the specified path by looking through modules, implementations,
    /// and enumerations along the way.
    ///
    pub fn resolve_path(scope: Rc<RefCell<Scope>>, path: &Path) -> Result<Item, SemanticError> {
        let mut current_scope = scope;

        for (index, identifier) in path.elements.iter().enumerate() {
            let item = Self::resolve_item(current_scope.clone(), identifier, index == 0)?;

            if index == path.elements.len() - 1 {
                return Ok(item);
            }

            current_scope = match item {
                Item::Module(module) => module.resolve()?,
                Item::Type(r#type) => {
                    let r#type = r#type.resolve()?;
                    match r#type {
                        Type::Enumeration(ref inner) => inner.scope.to_owned(),
                        Type::Structure(ref inner) => inner.scope.to_owned(),
                        Type::Contract(ref inner) => inner.scope.to_owned(),
                        _ => {
                            return Err(SemanticError::Scope(Error::ItemNotNamespace {
                                location: identifier.location,
                                name: identifier.name.to_owned(),
                            }))
                        }
                    }
                }
                _ => {
                    return Err(SemanticError::Scope(Error::ItemNotNamespace {
                        location: identifier.location,
                        name: identifier.name.to_owned(),
                    }))
                }
            };
        }

        Err(SemanticError::Scope(Error::ItemUndeclared {
            location: path.location,
            name: path.to_string(),
        }))
    }

    ///
    /// Resolves the item within the current scope hierarchy.
    ///
    pub fn resolve_item(
        scope: Rc<RefCell<Scope>>,
        identifier: &Identifier,
        recursive: bool,
    ) -> Result<Item, SemanticError> {
        match scope.borrow().items.get(identifier.name.as_str()) {
            Some(item) => {
                item.resolve()?;
                Ok(item.to_owned())
            }
            None => match scope.borrow().parent {
                Some(ref parent) if recursive => {
                    Self::resolve_item(parent.to_owned(), identifier, recursive)
                }
                Some(_) | None => Err(SemanticError::Scope(Error::ItemUndeclared {
                    location: identifier.location,
                    name: identifier.name.to_owned(),
                })),
            },
        }
    }

    ///
    /// Checks whether the item is declared within the current scope hierarchy.
    ///
    pub fn is_item_declared(&self, identifier: &str) -> bool {
        if self.items.contains_key(identifier) {
            true
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().is_item_declared(identifier),
                None => false,
            }
        }
    }

    ///
    /// Gets the `main` function location from the current scope.
    ///
    pub fn get_main_location(&self) -> Option<Location> {
        self.items
            .get(crate::FUNCTION_MAIN_IDENTIFIER)
            .and_then(|main| main.location())
    }

    ///
    /// Gets the contract type definition from the current scope.
    ///
    pub fn get_contract_location(&self) -> Option<Location> {
        for (_name, item) in self.items.iter() {
            match item {
                Item::Type(r#type) if r#type.is_contract() => return item.location(),
                _ => {}
            }
        }

        None
    }

    ///
    /// Creates a child scope with `parent` as its parent.
    ///
    pub fn new_child(parent: Rc<RefCell<Scope>>) -> Rc<RefCell<Self>> {
        Self::new(Some(parent)).wrap()
    }
}
