//!
//! The semantic analyzer scope.
//!

mod error;
mod item;

pub use self::error::Error;
pub use self::item::Item;
pub use self::item::Static as StaticItem;
pub use self::item::Variable as VariableItem;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::Constant;
use crate::semantic::Error as SemanticError;
use crate::semantic::Path;
use crate::semantic::Type;
use crate::semantic::Type as TypeItem;

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    parent: Option<Rc<RefCell<Self>>>,
    items: HashMap<String, Item>,
}

impl Default for Scope {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Scope {
    pub fn new(parent: Option<Rc<RefCell<Self>>>) -> Self {
        Self {
            parent,
            items: HashMap::new(),
        }
    }

    pub fn new_global() -> Self {
        Self {
            parent: None,
            items: Self::default_items(),
        }
    }

    pub fn declare_item(&mut self, identifier: String, item: Item) -> Result<(), Error> {
        if self.is_item_declared(&identifier) {
            return Err(Error::ItemRedeclared(identifier));
        }
        self.items.insert(identifier, item);
        Ok(())
    }

    pub fn declare_variable(
        &mut self,
        identifier: String,
        variable: VariableItem,
    ) -> Result<(), Error> {
        if self.is_item_declared(&identifier) {
            return Err(Error::ItemRedeclared(identifier));
        }
        self.items.insert(identifier, Item::Variable(variable));
        Ok(())
    }

    pub fn declare_constant(
        &mut self,
        identifier: String,
        constant: Constant,
    ) -> Result<(), Error> {
        if self.is_item_declared(&identifier) {
            return Err(Error::ItemRedeclared(identifier));
        }
        self.items.insert(identifier, Item::Constant(constant));
        Ok(())
    }

    pub fn declare_static(
        &mut self,
        identifier: String,
        r#static: StaticItem,
    ) -> Result<(), Error> {
        if self.is_item_declared(&identifier) {
            return Err(Error::ItemRedeclared(identifier));
        }
        self.items.insert(identifier, Item::Static(r#static));
        Ok(())
    }

    pub fn declare_type(&mut self, identifier: String, r#type: TypeItem) -> Result<(), Error> {
        if self.is_item_declared(&identifier) {
            return Err(Error::ItemRedeclared(identifier));
        }
        self.items.insert(identifier, Item::Type(r#type));
        Ok(())
    }

    pub fn declare_module(
        &mut self,
        identifier: String,
        scope: Rc<RefCell<Scope>>,
    ) -> Result<(), Error> {
        if self.is_item_declared(&identifier) {
            return Err(Error::ItemRedeclared(identifier));
        }
        self.items.insert(identifier, Item::Module(scope));
        Ok(())
    }

    pub fn resolve_path(scope: Rc<RefCell<Scope>>, path: &Path) -> Result<Item, SemanticError> {
        let mut current_scope = scope;

        for (index, identifier) in path.elements.iter().enumerate() {
            let item = Self::resolve_item(current_scope.clone(), &identifier.name)
                .map_err(|error| SemanticError::Scope(identifier.location, error))?;

            if index == path.elements.len() - 1 {
                return Ok(item);
            }

            match item {
                Item::Module(ref scope) => current_scope = scope.to_owned(),
                Item::Type(Type::Enumeration { ref scope, .. }) => current_scope = scope.to_owned(),
                Item::Type(Type::Structure { ref scope, .. }) => current_scope = scope.to_owned(),
                _ => {
                    return Err(SemanticError::Scope(
                        identifier.location,
                        Error::ItemIsNotNamespace(identifier.name.to_owned()),
                    ))
                }
            }
        }

        Err(SemanticError::Scope(
            path.location,
            Error::ItemUndeclared(path.to_string()),
        ))
    }

    pub fn resolve_item(scope: Rc<RefCell<Scope>>, identifier: &str) -> Result<Item, Error> {
        match scope.borrow().items.get(identifier) {
            Some(item) => Ok(item.to_owned()),
            None => match scope.borrow().parent {
                Some(ref parent) => Self::resolve_item(parent.to_owned(), identifier),
                None => Err(Error::ItemUndeclared(identifier.to_owned())),
            },
        }
    }

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

    pub fn new_child(parent: Rc<RefCell<Scope>>) -> Rc<RefCell<Scope>> {
        Rc::new(RefCell::new(Scope::new(Some(parent))))
    }

    fn default_items() -> HashMap<String, Item> {
        let mut items = HashMap::with_capacity(2);

        items.insert("dbg".to_owned(), Item::Type(Type::new_dbg_function()));
        items.insert("assert".to_owned(), Item::Type(Type::new_assert_function()));

        let mut std_scope = Scope::default();
        std_scope.items.insert(
            "sha256".to_owned(),
            Item::Type(Type::new_std_function(BuiltinIdentifier::CryptoSha256)),
        );
        std_scope.items.insert(
            "pedersen".to_owned(),
            Item::Type(Type::new_std_function(BuiltinIdentifier::CryptoPedersen)),
        );
        items.insert(
            "std".to_owned(),
            Item::Module(Rc::new(RefCell::new(std_scope))),
        );

        items
    }
}
