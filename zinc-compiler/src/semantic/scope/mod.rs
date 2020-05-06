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

use crate::lexical::token::lexeme::keyword::Keyword;
use crate::lexical::token::location::Location;
use crate::semantic::element::path::Path;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;
use crate::syntax::tree::identifier::Identifier;

use self::builtin::BuiltInScope;
use self::error::Error;
use self::item::constant::Constant as ConstantItem;
use self::item::module::Module as ModuleItem;
use self::item::r#type::Type as TypeItem;
use self::item::variable::Variable as VariableItem;
use self::item::Item;

///
/// A scope consists of a hashmap of the declared items and a reference to its parent.
/// The global scope has no parent.
/// Modules are connected to the program scope hierarchy horizontally, being stored as module items.
///
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Scope {
    parent: Option<Rc<RefCell<Self>>>,
    items: HashMap<String, Item>,
}

impl Scope {
    ///
    /// Initializes a nested scope with an explicit optional parent.
    ///
    pub fn new(parent: Option<Rc<RefCell<Self>>>) -> Self {
        Self {
            parent,
            items: HashMap::new(),
        }
    }

    ///
    /// Initializes a global scope without a parent and with default items.
    ///
    pub fn new_global() -> Self {
        Self {
            parent: Some(Rc::new(RefCell::new(BuiltInScope::initialize()))),
            items: HashMap::new(),
        }
    }

    ///
    /// Declares a general item.
    ///
    pub fn declare_item(
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
    /// Declares a variable, which is normally a `let` binding or a function actual parameter.
    ///
    pub fn declare_variable(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        variable: VariableItem,
    ) -> Result<(), Error> {
        if let Ok(item) = Self::resolve_item(scope.clone(), &identifier, true) {
            return Err(Error::ItemRedeclared {
                location: variable.location,
                name: identifier.name,
                reference: item.location(),
            });
        }

        scope
            .borrow_mut()
            .items
            .insert(identifier.name, Item::Variable(variable));

        Ok(())
    }

    ///
    /// Declares a constant, which is normally a `const` binding.
    ///
    pub fn declare_constant(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        constant: ConstantItem,
    ) -> Result<(), Error> {
        if let Ok(item) = Self::resolve_item(scope.clone(), &identifier, true) {
            return Err(Error::ItemRedeclared {
                location: constant.location,
                name: identifier.name,
                reference: item.location(),
            });
        }

        scope
            .borrow_mut()
            .items
            .insert(identifier.name, Item::Constant(constant));

        Ok(())
    }

    ///
    /// Declares a type, which is normally a `type`, `struct`, or `enum` binding.
    ///
    pub fn declare_type(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        r#type: TypeItem,
    ) -> Result<(), Error> {
        if let Ok(item) = Self::resolve_item(scope.clone(), &identifier, true) {
            return Err(Error::ItemRedeclared {
                location: r#type.location.unwrap_or(identifier.location),
                name: identifier.name,
                reference: item.location(),
            });
        }

        scope
            .borrow_mut()
            .items
            .insert(identifier.name, Item::Type(r#type));

        Ok(())
    }

    ///
    /// Declares a `contract` type, also checks whether it is the only contract in the scope.
    ///
    pub fn declare_contract(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        r#type: TypeItem,
    ) -> Result<(), Error> {
        if let Some(location) = scope.borrow().get_contract_location() {
            return Err(Error::ContractRedeclared {
                location: r#type.location.unwrap_or(identifier.location),
                reference: location,
            });
        }

        Self::declare_type(scope, identifier, r#type)
    }

    ///
    /// Declares a module, which is normally a `mod` binding.
    ///
    pub fn declare_module(
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
            identifier.name,
            Item::Module(ModuleItem::new(Some(identifier.location), module)),
        );

        Ok(())
    }

    ///
    /// Declares the `Self` alias within a type implementation.
    ///
    /// Since `Self` is the reserved keyword, it is not being checked for being already declared.
    ///
    pub fn declare_self(&mut self, r#type: TypeItem) {
        self.items
            .insert(Keyword::SelfUppercase.to_string(), Item::Type(r#type));
    }

    ///
    /// Gets an item at the specified path by looking through modules, implementations,
    /// and enumerations along the way.
    ///
    pub fn resolve_path(scope: Rc<RefCell<Scope>>, path: &Path) -> Result<Item, SemanticError> {
        let mut current_scope = scope;

        for (index, identifier) in path.elements.iter().enumerate() {
            let item = Self::resolve_item(current_scope.clone(), identifier, index == 0)
                .map_err(SemanticError::Scope)?;

            if index == path.elements.len() - 1 {
                return Ok(item);
            }

            current_scope = match item {
                Item::Module(ref inner) => inner.scope.to_owned(),
                Item::Type(TypeItem {
                    inner: Type::Enumeration(ref inner),
                    ..
                }) => inner.scope.to_owned(),
                Item::Type(TypeItem {
                    inner: Type::Structure(ref inner),
                    ..
                }) => inner.scope.to_owned(),
                Item::Type(TypeItem {
                    inner: Type::Contract(ref inner),
                    ..
                }) => inner.scope.to_owned(),
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
    ) -> Result<Item, Error> {
        match scope.borrow().items.get(identifier.name.as_str()) {
            Some(item) => Ok(item.to_owned()),
            None => match scope.borrow().parent {
                Some(ref parent) if recursive => {
                    Self::resolve_item(parent.to_owned(), identifier, recursive)
                }
                Some(_) | None => Err(Error::ItemUndeclared {
                    location: identifier.location,
                    name: identifier.name.to_owned(),
                }),
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
            if let Item::Type(TypeItem {
                inner: Type::Contract(_),
                ..
            }) = item
            {
                return item.location();
            }
        }

        None
    }

    ///
    /// Creates a child scope with `parent` as its parent.
    ///
    pub fn new_child(parent: Rc<RefCell<Scope>>) -> Rc<RefCell<Scope>> {
        Rc::new(RefCell::new(Scope::new(Some(parent))))
    }
}
