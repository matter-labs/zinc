//!
//! The semantic analyzer scope.
//!

#[cfg(test)]
mod tests;

pub mod intrinsic;
pub mod item;
pub mod stack;
pub mod r#type;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str;

use zinc_lexical::Keyword;
use zinc_lexical::Location;
use zinc_syntax::ConstStatement;
use zinc_syntax::ContractStatement;
use zinc_syntax::Identifier;

use crate::generator::statement::Statement as GeneratorStatement;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::path::Path;
use crate::semantic::element::r#type::Type as SemanticType;
use crate::semantic::error::Error;
use crate::semantic::scope::intrinsic::IntrinsicTypeId;
use crate::source::Source;

use self::intrinsic::IntrinsicScope;
use self::item::constant::Constant as ConstantItem;
use self::item::field::Field as FieldItem;
use self::item::module::Module as ModuleItem;
use self::item::r#type::statement::Statement as TypeStatementVariant;
use self::item::r#type::Type as TypeItem;
use self::item::variable::Variable as VariableItem;
use self::item::variant::Variant as VariantItem;
use self::item::Item;
use self::r#type::Type as ScopeType;

///
/// A scope consists of a hashmap of the declared items and a reference to its parent.
///
/// The global scope has the `root` scope as its parent with intrinsic functions and libraries.
///
/// Modules are connected to the entry scope hierarchy horizontally, being stored as module items.
///
#[derive(Debug, Clone)]
pub struct Scope {
    /// The scope name, e.g. module name, structure name, etc.
    name: String,
    /// The scope type, e.g. module, implementation, function, etc.
    r#type: ScopeType,
    /// The vertical parent scope, which the current one has access to.
    parent: Option<Rc<RefCell<Self>>>,
    /// The hashmap with items declared at the current scope level, with item names as keys.
    items: RefCell<HashMap<String, Rc<RefCell<Item>>>>,
}

impl Scope {
    /// The scope items hashmap default capacity.
    const ITEMS_INITIAL_CAPACITY: usize = 64;

    ///
    /// Initializes a scope with an explicit optional parent.
    ///
    /// Beware that if you omit the `parent`, intrinsic functions and `std` will not be available
    /// throughout the scope stack. To create a scope with such items available, use `new_global`.
    ///
    pub fn new(name: String, r#type: ScopeType, parent: Option<Rc<RefCell<Self>>>) -> Self {
        Self {
            name,
            r#type,
            parent,
            items: RefCell::new(HashMap::with_capacity(Self::ITEMS_INITIAL_CAPACITY)),
        }
    }

    ///
    /// Initializes a global scope with the intrinsic one as its parent and the dependency modules.
    ///
    pub fn new_module(
        name: String,
        dependencies: HashMap<String, Rc<RefCell<Scope>>>,
        entry: Option<zinc_project::ManifestProject>,
        is_dependency_entry: bool,
    ) -> Self {
        let mut items = HashMap::with_capacity(Self::ITEMS_INITIAL_CAPACITY + dependencies.len());
        for (name, scope) in dependencies.into_iter() {
            let module = ModuleItem::new_defined(None, name.clone(), scope, false);

            items.insert(name, Item::Module(module).wrap());
        }

        let r#type = if let Some(project) = entry {
            ScopeType::Entry {
                project,
                is_dependency: is_dependency_entry,
            }
        } else {
            ScopeType::Module {
                is_dependency: is_dependency_entry,
            }
        };

        Self {
            name,
            r#type,
            parent: Some(IntrinsicScope::initialize()),
            items: RefCell::new(items),
        }
    }

    ///
    /// Initializes the root scope with intrinsic function and library definitions.
    ///
    pub fn new_intrinsic(name: &'static str) -> Self {
        Self {
            name: name.to_owned(),
            r#type: ScopeType::Intrinsic,
            parent: None,
            items: RefCell::new(HashMap::with_capacity(Self::ITEMS_INITIAL_CAPACITY)),
        }
    }

    ///
    /// Creates a child scope with `parent` as its parent.
    ///
    pub fn new_child(
        name: String,
        r#type: ScopeType,
        parent: Rc<RefCell<Scope>>,
    ) -> Rc<RefCell<Self>> {
        Self::new(name, r#type, Some(parent)).wrap()
    }

    ///
    /// Returns the scope name.
    ///
    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    ///
    /// Returns the scope type.
    ///
    pub fn r#type(&self) -> ScopeType {
        self.r#type.clone()
    }

    ///
    /// Returns the scope parent.
    ///
    pub fn parent(&self) -> Option<Rc<RefCell<Self>>> {
        self.parent.to_owned()
    }

    ///
    /// Wraps the scope into `Rc<RefCell<_>>` simplifying most of initializations.
    ///
    pub fn wrap(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    ///
    /// Extracts the scope from `Rc<RefCell<_>>`.
    ///
    pub fn unwrap_rc(scope: Rc<RefCell<Self>>) -> Self {
        Rc::try_unwrap(scope)
            .expect(zinc_const::panic::LAST_SHARED_REFERENCE)
            .into_inner()
    }

    ///
    /// Returns the entry description. Is `None` for non-entry module items.
    ///
    pub fn entry(&self) -> Option<(zinc_project::ManifestProject, bool)> {
        if let ScopeType::Entry {
            ref project,
            is_dependency,
        } = self.r#type
        {
            Some((project.to_owned(), is_dependency))
        } else {
            match self.parent {
                Some(ref parent) => parent.borrow().entry(),
                None => None,
            }
        }
    }

    ///
    /// Internally defines all the items in the order they have been declared.
    ///
    pub fn define(&self) -> Result<(), Error> {
        let mut items: Vec<(String, Rc<RefCell<Item>>)> =
            self.items.clone().into_inner().into_iter().collect();
        items.sort_by_key(|(_name, item)| RefCell::borrow(&item).item_id());

        for (name, item) in items.into_iter() {
            if Keyword::is_alias(name.as_str()) {
                continue;
            }

            RefCell::borrow(&item).define()?;
        }

        Ok(())
    }

    ///
    /// Inserts an item, does not check if the item has been already declared.
    ///
    pub fn insert_item(scope: Rc<RefCell<Scope>>, name: String, item: Rc<RefCell<Item>>) {
        RefCell::borrow(&scope)
            .items
            .borrow_mut()
            .insert(name, item);
    }

    ///
    /// Defines an item of arbitrary type, checks if the item has been already declared.
    ///
    pub fn define_item(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        item: Rc<RefCell<Item>>,
    ) -> Result<(), Error> {
        if let Ok(item) = RefCell::borrow(&scope).resolve_item(&identifier, true) {
            return Err(Error::ScopeItemRedeclared {
                location: identifier.location,
                name: identifier.name.clone(),
                reference: RefCell::borrow(&item).location(),
            });
        }

        RefCell::borrow(&scope)
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
        r#type: SemanticType,
    ) -> Result<(), Error> {
        if let Ok(item) =
            RefCell::borrow(&scope).resolve_item(&identifier, !identifier.is_self_lowercase())
        {
            return Err(Error::ScopeItemRedeclared {
                location: identifier.location,
                name: identifier.name.clone(),
                reference: RefCell::borrow(&item).location(),
            });
        }

        let name = identifier.name.clone();
        let item = Item::Variable(VariableItem::new(
            Some(identifier.location),
            is_mutable,
            identifier.name,
            r#type,
        ));

        RefCell::borrow(&scope)
            .items
            .borrow_mut()
            .insert(name, item.wrap());

        Ok(())
    }

    ///
    /// Defines a contract field.
    ///
    pub fn define_field(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        r#type: SemanticType,
        index: usize,
        is_public: bool,
        is_implicit: bool,
        is_immutable: bool,
    ) -> Result<(), Error> {
        if let Ok(item) = RefCell::borrow(&scope).resolve_item(&identifier, false) {
            return Err(Error::ScopeItemRedeclared {
                location: identifier.location,
                name: identifier.name.clone(),
                reference: RefCell::borrow(&item).location(),
            });
        }

        let name = identifier.name.clone();
        let item = Item::Field(FieldItem::new(
            identifier.location,
            identifier.name,
            r#type,
            index,
            is_public,
            is_implicit,
            is_immutable,
        ));

        RefCell::borrow(&scope)
            .items
            .borrow_mut()
            .insert(name, item.wrap());

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
        if let Ok(item) = RefCell::borrow(&scope).resolve_item(&statement.identifier, true) {
            return Err(Error::ScopeItemRedeclared {
                location: statement.location,
                name: statement.identifier.name.clone(),
                reference: RefCell::borrow(&item).location(),
            });
        }

        let name = statement.identifier.name.clone();
        let item = Item::Constant(ConstantItem::new_declared(
            statement.identifier.location,
            statement,
            scope.clone(),
        ));

        RefCell::borrow(&scope)
            .items
            .borrow_mut()
            .insert(name, item.wrap());

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
        if let Ok(item) = RefCell::borrow(&scope).resolve_item(&identifier, true) {
            return Err(Error::ScopeItemRedeclared {
                location: identifier.location,
                name: identifier.name.clone(),
                reference: RefCell::borrow(&item).location(),
            });
        }

        let name = identifier.name;
        let item = Item::Constant(ConstantItem::new_defined(identifier.location, constant));

        RefCell::borrow(&scope)
            .items
            .borrow_mut()
            .insert(name, item.wrap());

        Ok(())
    }

    ///
    /// Defines an enumeration variant, which has been instantly evaluated.
    ///
    pub fn define_variant(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        constant: Constant,
    ) -> Result<(), Error> {
        if let Ok(item) = RefCell::borrow(&scope).resolve_item(&identifier, false) {
            return Err(Error::ScopeItemRedeclared {
                location: identifier.location,
                name: identifier.name.clone(),
                reference: RefCell::borrow(&item).location(),
            });
        }

        let name = identifier.name;
        let item = Item::Variant(VariantItem::new(
            identifier.location,
            name.clone(),
            constant,
        ));

        RefCell::borrow(&scope)
            .items
            .borrow_mut()
            .insert(name, item.wrap());

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
        if let Ok(item) = RefCell::borrow(&scope).resolve_item(&statement.identifier(), true) {
            return Err(Error::ScopeItemRedeclared {
                location: statement.location(),
                name: statement.identifier().name.to_owned(),
                reference: RefCell::borrow(&item).location(),
            });
        }

        let name = statement.identifier().name.clone();
        let item = Item::Type(TypeItem::new_declared(
            Some(statement.location()),
            statement,
            scope.clone(),
        )?);

        RefCell::borrow(&scope)
            .items
            .borrow_mut()
            .insert(name, item.wrap());

        Ok(())
    }

    ///
    /// Defines a type, which has been instantly evaluated.
    ///
    pub fn define_type(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        r#type: SemanticType,
        intermediate: Option<GeneratorStatement>,
    ) -> Result<(), Error> {
        if let Ok(item) = RefCell::borrow(&scope).resolve_item(&identifier, true) {
            return Err(Error::ScopeItemRedeclared {
                location: r#type.location().unwrap_or(identifier.location),
                name: identifier.name.clone(),
                reference: RefCell::borrow(&item).location(),
            });
        }

        let name = identifier.name;
        let item = Item::Type(TypeItem::new_defined(
            Some(identifier.location),
            r#type,
            false,
            intermediate,
        ));

        RefCell::borrow(&scope)
            .items
            .borrow_mut()
            .insert(name, item.wrap());

        Ok(())
    }

    ///
    /// Defines a `contract` type, also checks whether it is the only contract in the scope.
    ///
    pub fn declare_contract(
        scope: Rc<RefCell<Scope>>,
        statement: ContractStatement,
    ) -> Result<(), Error> {
        if let Some(location) = RefCell::borrow(&scope).get_contract_location() {
            return Err(Error::ScopeContractRedeclared {
                location: statement.location,
                reference: location,
            });
        }

        Scope::declare_type(scope, TypeStatementVariant::Contract(statement))
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
        dependencies: HashMap<String, Rc<RefCell<Scope>>>,
    ) -> Result<(), Error> {
        if let Ok(item) = RefCell::borrow(&scope).resolve_item(&identifier, true) {
            return Err(Error::ScopeItemRedeclared {
                location: identifier.location,
                name: identifier.name.clone(),
                reference: RefCell::borrow(&item).location(),
            });
        }

        let name = identifier.name.clone();
        let module_scope =
            Self::new_module(identifier.name.clone(), dependencies.clone(), None, false).wrap();
        let module = ModuleItem::new_declared(
            Some(identifier.location),
            module_scope.clone(),
            identifier.name,
            module,
            scope_crate,
            Some(scope.clone()),
            dependencies,
            false,
        )?;
        let item = Item::Module(module).wrap();

        RefCell::borrow(&module_scope)
            .items
            .borrow_mut()
            .insert(Keyword::SelfLowercase.to_string(), item.clone());
        RefCell::borrow(&scope)
            .items
            .borrow_mut()
            .insert(name, item);

        Ok(())
    }

    ///
    /// Returns the module `self` alias. Panics if the scope does not belong to a module or
    /// the alias has not been declared yet.
    ///
    pub fn get_module_self_alias(scope: Rc<RefCell<Scope>>) -> Rc<RefCell<Item>> {
        RefCell::borrow(&scope)
            .items
            .borrow()
            .get(&Keyword::SelfLowercase.to_string())
            .cloned()
            .expect(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS)
    }

    ///
    /// Resolves an item at the specified path by looking through modules and type scopes.
    ///
    /// If the `path` consists of only one element, the path is resolved recursively, that is,
    /// looking through the whole scope hierarchy up to the module level and global intrinsic scope.
    ///
    /// If the `path` consists if more than one element, the elements starting from the 2nd are
    /// resolved non-recursively, that is, looking only at the first-level scope of the path element.
    ///
    pub fn resolve_path(
        scope: Rc<RefCell<Scope>>,
        path: &Path,
    ) -> Result<Rc<RefCell<Item>>, Error> {
        let mut current_scope = scope;

        for (index, identifier) in path.elements.iter().enumerate() {
            let is_element_first = index == 0;
            let is_element_last = index == path.elements.len() - 1;

            let item =
                RefCell::borrow(&current_scope).resolve_item(identifier, is_element_first)?;
            RefCell::borrow(&item).define()?;

            if is_element_last {
                return Ok(item);
            }

            current_scope = match *RefCell::borrow(&item) {
                Item::Module(ref module) => module.define()?,
                Item::Type(ref r#type) => {
                    let r#type = r#type.define()?;
                    match r#type {
                        SemanticType::Enumeration(ref inner) => inner.scope.to_owned(),
                        SemanticType::Structure(ref inner) => inner.scope.to_owned(),
                        SemanticType::Contract(ref inner) => inner.scope.to_owned(),
                        _ => {
                            return Err(Error::ScopeExpectedNamespace {
                                location: identifier.location,
                                name: identifier.name.to_owned(),
                            });
                        }
                    }
                }
                _ => {
                    return Err(Error::ScopeExpectedNamespace {
                        location: identifier.location,
                        name: identifier.name.to_owned(),
                    });
                }
            };
        }

        Err(Error::ScopeItemUndeclared {
            location: path.location,
            name: path.to_string(),
        })
    }

    ///
    /// Resolves the item with `identifier` within the current `scope`. Looks through the parent scopes
    /// if `recursive` is true.
    ///
    pub fn resolve_item(
        &self,
        identifier: &Identifier,
        recursive: bool,
    ) -> Result<Rc<RefCell<Item>>, Error> {
        match self.items.borrow().get(identifier.name.as_str()) {
            Some(item) => Ok(item.to_owned()),
            None => match self.parent {
                Some(ref parent) if recursive => {
                    RefCell::borrow(&parent).resolve_item(identifier, recursive)
                }
                Some(_) | None => Err(Error::ScopeItemUndeclared {
                    location: identifier.location,
                    name: identifier.name.to_owned(),
                }),
            },
        }
    }

    ///
    /// Resolves the `std::collections::MTreeMap` type.
    ///
    /// Cannot panic, since the type is declared by the developer in the intrinsic module.
    ///
    pub fn resolve_mtreemap(location: Location, scope: Rc<RefCell<Scope>>) -> SemanticType {
        let item = Scope::resolve_path(
            scope,
            &Path::new_complex(
                location,
                vec![
                    Identifier::new(location, "std".to_owned()),
                    Identifier::new(location, "collections".to_owned()),
                    Identifier::new(location, "MTreeMap".to_owned()),
                ],
            ),
        )
        .expect(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS);

        let item = RefCell::borrow(&item);
        match &*item {
            Item::Type(ref r#type) => match r#type
                .define()
                .expect(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS)
            {
                SemanticType::Structure(mut structure)
                    if structure.type_id == IntrinsicTypeId::StdCollectionsMTreeMap as usize =>
                {
                    structure
                        .set_generics(
                            location,
                            Some(vec![
                                SemanticType::integer_unsigned(
                                    None,
                                    zinc_const::bitlength::ETH_ADDRESS,
                                ),
                                SemanticType::integer_unsigned(
                                    None,
                                    zinc_const::bitlength::INTEGER_MAX,
                                ),
                            ]),
                        )
                        .expect(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS);
                    SemanticType::Structure(structure)
                }
                _type => panic!(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS),
            },
            _item => panic!(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS),
        }
    }

    ///
    /// Gets the `main` function location from the current scope.
    ///
    pub fn get_main_location(&self) -> Option<Location> {
        self.items
            .borrow()
            .get(zinc_const::source::FUNCTION_MAIN_IDENTIFIER)
            .and_then(|main| RefCell::borrow(main).location())
    }

    ///
    /// Gets the contract type definition from the current scope.
    ///
    pub fn get_contract_location(&self) -> Option<Location> {
        for (_name, item) in self.items.borrow().iter() {
            match *RefCell::borrow(item) {
                Item::Type(ref r#type) if r#type.is_contract() => {
                    return RefCell::borrow(item).location()
                }
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
                    return None;
                }

                Some(RefCell::borrow(item).get_intermediate())
            })
            .flatten()
            .collect()
    }

    ///
    /// Displays the scope hierarchy.
    ///
    /// Is used for testing purposes.
    ///
    pub fn show(&self, level: usize) {
        println!(
            "{}==== {:?} `{}` ====",
            "    ".repeat(level),
            self.r#type,
            self.name
        );

        for (name, item) in self.items.borrow().iter() {
            println!(
                "{}{}: {}",
                "    ".repeat(level),
                name,
                RefCell::borrow(&item)
            );

            if Keyword::is_alias(name.as_str()) {
                continue;
            }

            if let Item::Module(ref module) = *RefCell::borrow(item) {
                match module.scope() {
                    Ok(scope) => RefCell::borrow(&scope).show(level + 1),
                    Err(error) => log::warn!("During definition: {:?}", error),
                }
            }
        }

        if let Some(parent) = self.parent.as_ref() {
            if matches!(RefCell::borrow(parent).r#type(), ScopeType::Intrinsic) {
                return;
            }

            RefCell::borrow(parent).show(level + 1);
        }
    }
}
