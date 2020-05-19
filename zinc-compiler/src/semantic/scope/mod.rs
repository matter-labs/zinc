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
use crate::source::Source;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::statement::contract::Statement as ContractStatement;
use crate::syntax::tree::statement::r#const::Statement as ConstStatement;

use self::builtin::BuiltInScope;
use self::error::Error;
use self::item::constant::Constant as ConstantItem;
use self::item::module::Module as ModuleItem;
use self::item::r#type::statement::Statement as TypeStatementVariant;
use self::item::r#type::Type as TypeItem;
use self::item::variable::memory_type::MemoryType;
use self::item::variable::Variable as VariableItem;
use self::item::Item;

///
/// A scope consists of a hashmap of the declared items and a reference to its parent.
///
/// The global scope has the `built-in` scope with the `std` library and built-in functions as its parent.
///
/// Modules are connected to the entry scope hierarchy horizontally, being stored as module items.
///
#[derive(Debug, Clone)]
pub struct Scope {
    pub name: String,
    pub parent: Option<Rc<RefCell<Self>>>,
    pub items: RefCell<HashMap<String, Rc<RefCell<Item>>>>,
    pub is_built_in: bool,
}

impl Scope {
    const ITEMS_INITIAL_CAPACITY: usize = 1024;

    ///
    /// Initializes a scope with an explicit optional parent.
    ///
    /// Beware that if you omit the `parent`, built-in functions and `std` will not be available
    /// throughout the scope stack. To create a scope with such items available, use `new_global`.
    ///
    pub fn new(name: String, parent: Option<Rc<RefCell<Self>>>) -> Self {
        Self {
            name,
            parent,
            items: RefCell::new(HashMap::with_capacity(Self::ITEMS_INITIAL_CAPACITY)),
            is_built_in: false,
        }
    }

    ///
    /// Initializes a global scope without the built-in one as its parent.
    ///
    pub fn new_global(name: String) -> Self {
        Self {
            name,
            parent: Some(BuiltInScope::initialize().wrap()),
            items: RefCell::new(HashMap::with_capacity(Self::ITEMS_INITIAL_CAPACITY)),
            is_built_in: false,
        }
    }

    ///
    /// Initializes the built-in scope which is used for `std` and built-in function definitions.
    ///
    pub fn new_built_in(name: &'static str) -> Self {
        Self {
            name: name.to_owned(),
            parent: None,
            items: RefCell::new(HashMap::with_capacity(Self::ITEMS_INITIAL_CAPACITY)),
            is_built_in: true,
        }
    }

    ///
    /// Creates a child scope with `parent` as its parent.
    ///
    pub fn new_child(name: String, parent: Rc<RefCell<Scope>>) -> Rc<RefCell<Self>> {
        Self::new(name, Some(parent)).wrap()
    }

    ///
    /// Wraps the scope into `Rc<RefCell<_>>` simplifying most of initializations.
    ///
    pub fn wrap(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    ///
    /// Internally defines all the items in the order they have been declared.
    ///
    pub fn define(&self) -> Result<(), SemanticError> {
        let mut items: Vec<(String, Rc<RefCell<Item>>)> =
            self.items.clone().into_inner().into_iter().collect();
        items.sort_by_key(|(_name, item)| item.borrow().item_id());

        for (name, item) in items.into_iter() {
            if Keyword::is_alias(name.as_str()) {
                continue;
            }

            item.borrow().define()?;
        }

        Ok(())
    }

    ///
    /// Defines an item of arbitrary type.
    ///
    pub fn define_item(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        item: Rc<RefCell<Item>>,
    ) -> Result<(), SemanticError> {
        if let Ok(item) = scope.borrow().resolve_item(&identifier, true) {
            return Err(SemanticError::Scope(Error::ItemRedeclared {
                location: identifier.location,
                name: identifier.name.clone(),
                reference: item.borrow().location(),
            }));
        }

        scope
            .borrow()
            .items
            .borrow_mut()
            .insert(identifier.name, item);

        Ok(())
    }

    ///
    /// Defines a variable, which is usually a `let` binding or a function actual parameter.
    ///
    /// If the variable is the object instance `self` alias, it is not checked for being redeclared
    /// recursively to avoid collision with the module `self` alias.
    ///
    pub fn define_variable(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        is_mutable: bool,
        r#type: Type,
        memory_type: MemoryType,
    ) -> Result<(), SemanticError> {
        if let Ok(item) = scope
            .borrow()
            .resolve_item(&identifier, !identifier.is_self())
        {
            return Err(SemanticError::Scope(Error::ItemRedeclared {
                location: identifier.location,
                name: identifier.name.clone(),
                reference: item.borrow().location(),
            }));
        }

        let name = identifier.name.clone();
        let item = Item::Variable(VariableItem::new(
            identifier.location,
            is_mutable,
            identifier.name,
            r#type,
            memory_type,
        ));

        scope.borrow().items.borrow_mut().insert(name, item.wrap());

        Ok(())
    }

    ///
    /// Declares a constant, saving the `const` statement to define itself later during the second
    /// pass or referencing for the first time.
    ///
    pub fn declare_constant(
        scope: Rc<RefCell<Scope>>,
        statement: ConstStatement,
    ) -> Result<(), SemanticError> {
        if let Ok(item) = scope.borrow().resolve_item(&statement.identifier, true) {
            return Err(SemanticError::Scope(Error::ItemRedeclared {
                location: statement.location,
                name: statement.identifier.name.clone(),
                reference: item.borrow().location(),
            }));
        }

        let name = statement.identifier.name.clone();
        let item = Item::Constant(ConstantItem::new_declared(
            statement.identifier.location,
            statement,
            scope.clone(),
        ));

        scope.borrow().items.borrow_mut().insert(name, item.wrap());

        Ok(())
    }

    ///
    /// Defines a constant, which has been instantly evaluated.
    ///
    pub fn define_constant(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        constant: Constant,
    ) -> Result<(), SemanticError> {
        if let Ok(item) = scope.borrow().resolve_item(&identifier, true) {
            return Err(SemanticError::Scope(Error::ItemRedeclared {
                location: identifier.location,
                name: identifier.name.clone(),
                reference: item.borrow().location(),
            }));
        }

        let name = identifier.name;
        let item = Item::Constant(ConstantItem::new_defined(identifier.location, constant));

        scope.borrow().items.borrow_mut().insert(name, item.wrap());

        Ok(())
    }

    ///
    /// Declares a type, saving the `type`, `struct`, `enum`, `contract` or another statement to
    /// define itself later during the second pass or referencing for the first time.
    ///
    pub fn declare_type(
        scope: Rc<RefCell<Scope>>,
        statement: TypeStatementVariant,
    ) -> Result<(), SemanticError> {
        if let Ok(item) = scope.borrow().resolve_item(&statement.identifier(), true) {
            return Err(SemanticError::Scope(Error::ItemRedeclared {
                location: statement.location(),
                name: statement.identifier().name.to_owned(),
                reference: item.borrow().location(),
            }));
        }

        let name = statement.identifier().name.clone();
        let item = Item::Type(TypeItem::new_declared(
            Some(statement.location()),
            statement,
            scope.clone(),
        )?);

        scope.borrow().items.borrow_mut().insert(name, item.wrap());

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
    ) -> Result<(), SemanticError> {
        if let Ok(item) = scope.borrow().resolve_item(&identifier, true) {
            return Err(SemanticError::Scope(Error::ItemRedeclared {
                location: r#type.location().unwrap_or(identifier.location),
                name: identifier.name.clone(),
                reference: item.borrow().location(),
            }));
        }

        let name = identifier.name;
        let item = Item::Type(TypeItem::new_defined(
            Some(identifier.location),
            r#type,
            false,
            intermediate,
        ));

        scope.borrow().items.borrow_mut().insert(name, item.wrap());

        Ok(())
    }

    ///
    /// Defines a `contract` type, also checks whether it is the only contract in the scope.
    ///
    pub fn declare_contract(
        scope: Rc<RefCell<Scope>>,
        statement: ContractStatement,
    ) -> Result<(), SemanticError> {
        if let Some(location) = scope.borrow().get_contract_location() {
            return Err(SemanticError::Scope(Error::ContractRedeclared {
                location: statement.location,
                reference: location,
            }));
        }

        Scope::declare_type(scope, TypeStatementVariant::Contract(statement))
    }

    ///
    /// Defines the `Self` alias within a type implementation.
    ///
    /// Since `Self` is a reserved keyword, it is not checked for being already declared.
    ///
    pub fn define_type_self_alias(scope: Rc<RefCell<Scope>>, r#type: Type) {
        let name = Keyword::SelfUppercase.to_string();
        let item = Item::Type(TypeItem::new_defined(r#type.location(), r#type, true, None));

        scope.borrow().items.borrow_mut().insert(name, item.wrap());
    }

    ///
    /// Declares a module, saving its representation to define itself later during the second
    /// pass or referencing for the first time.
    ///
    pub fn declare_module(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        module: Source,
        scope_crate: Rc<RefCell<Scope>>,
        is_entry: bool,
    ) -> Result<(), SemanticError> {
        if let Ok(item) = scope.borrow().resolve_item(&identifier, true) {
            return Err(SemanticError::Scope(Error::ItemRedeclared {
                location: identifier.location,
                name: identifier.name.clone(),
                reference: item.borrow().location(),
            }));
        }

        let name = identifier.name.clone();
        let module_scope = Self::new_global(identifier.name.clone()).wrap();
        let module = ModuleItem::new_declared(
            Some(identifier.location),
            module_scope.clone(),
            identifier.name,
            module,
            scope_crate,
            Some(scope.clone()),
            is_entry,
        )?;
        let item = Item::Module(module).wrap();

        module_scope
            .borrow()
            .items
            .borrow_mut()
            .insert(Keyword::SelfLowercase.to_string(), item.clone());
        scope.borrow().items.borrow_mut().insert(name, item);

        Ok(())
    }

    ///
    /// Resolves an item at the specified path by looking through modules and type scopes.
    ///
    pub fn resolve_path(
        scope: Rc<RefCell<Scope>>,
        path: &Path,
    ) -> Result<Rc<RefCell<Item>>, SemanticError> {
        let mut current_scope = scope;

        for (index, identifier) in path.elements.iter().enumerate() {
            let is_element_first = index == 0;
            let is_element_last = index == path.elements.len() - 1;

            let item = current_scope
                .borrow()
                .resolve_item(identifier, is_element_first)?;

            if is_element_last {
                return Ok(item);
            }

            current_scope = match *item.borrow() {
                Item::Module(ref module) => module.scope()?,
                Item::Type(ref r#type) => {
                    let r#type = r#type.define()?;
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
    /// Resolves the item with `identifier` within the current `scope`. Looks through the parent scopes
    /// if `recursive` is true.
    ///
    pub fn resolve_item(
        &self,
        identifier: &Identifier,
        recursive: bool,
    ) -> Result<Rc<RefCell<Item>>, SemanticError> {
        match self.items.borrow().get(identifier.name.as_str()) {
            Some(item) => Ok(item.to_owned()),
            None => match self.parent {
                Some(ref parent) if recursive => {
                    parent.borrow().resolve_item(identifier, recursive)
                }
                Some(_) | None => Err(SemanticError::Scope(Error::ItemUndeclared {
                    location: identifier.location,
                    name: identifier.name.to_owned(),
                })),
            },
        }
    }

    ///
    /// Gets the `main` function location from the current scope.
    ///
    pub fn get_main_location(&self) -> Option<Location> {
        self.items
            .borrow()
            .get(crate::FUNCTION_MAIN_IDENTIFIER)
            .and_then(|main| main.borrow().location())
    }

    ///
    /// Gets the contract type definition from the current scope.
    ///
    pub fn get_contract_location(&self) -> Option<Location> {
        for (_name, item) in self.items.borrow().iter() {
            match *item.borrow() {
                Item::Type(ref r#type) if r#type.is_contract() => return item.borrow().location(),
                _ => {}
            }
        }

        None
    }

    ///
    /// Extracts the intermediate representation from the element.
    ///
    pub fn get_intermediate(&self) -> Vec<GeneratorStatement> {
        self.items
            .borrow()
            .iter()
            .filter_map(|(name, item)| {
                if Keyword::is_alias(name.as_str()) {
                    None
                } else {
                    Some(item.borrow().get_intermediate())
                }
            })
            .flatten()
            .collect()
    }

    ///
    /// Displays the scopr hierarchy.
    ///
    /// Is used for testing purposes.
    ///
    pub fn show(&self, level: usize) {
        println!("{}==== Scope <{}> ====", "    ".repeat(level), self.name);

        for (name, item) in self.items.borrow().iter() {
            println!("{}{}: {}", "    ".repeat(level), name, item.borrow());

            if Keyword::is_alias(name.as_str()) {
                continue;
            }

            if let Item::Module(ref module) = *item.borrow() {
                match module.scope() {
                    Ok(scope) => scope.borrow().show(level + 1),
                    Err(error) => log::warn!("SCOPE IS UNAVAILABLE: {:?}", error),
                }
            }
        }

        if let Some(parent) = self.parent.as_ref() {
            if parent.borrow().is_built_in {
                return;
            }

            parent.borrow().show(level + 1);
        }
    }
}
