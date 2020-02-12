//!
//! The semantic analyzer scope.
//!

pub mod error;
pub mod item;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::element::constant::Constant;
use crate::semantic::element::path::Path;
use crate::semantic::element::r#type::function::Function as FunctionType;
use crate::semantic::element::r#type::Type;
use crate::semantic::Error as SemanticError;

use self::error::Error;
use self::item::r#static::Static as StaticItem;
use self::item::variable::Variable as VariableItem;
use self::item::Item;

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

    pub fn declare_type(&mut self, identifier: String, r#type: Type) -> Result<(), Error> {
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

    pub fn declare_self(&mut self, r#type: Type) {
        self.items.insert("Self".to_owned(), Item::Type(r#type));
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
                Item::Type(Type::Structure(ref structure)) => {
                    current_scope = structure.scope.to_owned()
                }
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
        let mut std_crypto_scope = Scope::default();
        let std_crypto_sha256 = FunctionType::new_std(BuiltinIdentifier::CryptoSha256);
        let std_crypto_pedersen = FunctionType::new_std(BuiltinIdentifier::CryptoPedersen);
        std_crypto_scope.items.insert(
            std_crypto_sha256.identifier(),
            Item::Type(Type::Function(std_crypto_sha256)),
        );
        std_crypto_scope.items.insert(
            std_crypto_pedersen.identifier(),
            Item::Type(Type::Function(std_crypto_pedersen)),
        );

        let mut std_convert_scope = Scope::default();
        let std_convert_to_bits = FunctionType::new_std(BuiltinIdentifier::ToBits);
        let std_convert_from_bits_unsigned =
            FunctionType::new_std(BuiltinIdentifier::UnsignedFromBits);
        let std_convert_from_bits_signed = FunctionType::new_std(BuiltinIdentifier::SignedFromBits);
        let std_convert_from_bits_field = FunctionType::new_std(BuiltinIdentifier::FieldFromBits);
        std_convert_scope.items.insert(
            std_convert_to_bits.identifier(),
            Item::Type(Type::Function(std_convert_to_bits)),
        );
        std_convert_scope.items.insert(
            std_convert_from_bits_unsigned.identifier(),
            Item::Type(Type::Function(std_convert_from_bits_unsigned)),
        );
        std_convert_scope.items.insert(
            std_convert_from_bits_signed.identifier(),
            Item::Type(Type::Function(std_convert_from_bits_signed)),
        );
        std_convert_scope.items.insert(
            std_convert_from_bits_field.identifier(),
            Item::Type(Type::Function(std_convert_from_bits_field)),
        );

        let mut std_array_scope = Scope::default();
        let std_array_reverse = FunctionType::new_std(BuiltinIdentifier::ArrayReverse);
        let std_array_truncate = FunctionType::new_std(BuiltinIdentifier::ArrayTruncate);
        let std_array_pad = FunctionType::new_std(BuiltinIdentifier::ArrayPad);
        std_array_scope.items.insert(
            std_array_reverse.identifier(),
            Item::Type(Type::Function(std_array_reverse)),
        );
        std_array_scope.items.insert(
            std_array_truncate.identifier(),
            Item::Type(Type::Function(std_array_truncate)),
        );
        std_array_scope.items.insert(
            std_array_pad.identifier(),
            Item::Type(Type::Function(std_array_pad)),
        );

        let mut std_scope = Scope::default();
        std_scope.items.insert(
            "crypto".to_owned(),
            Item::Module(Rc::new(RefCell::new(std_crypto_scope))),
        );
        std_scope.items.insert(
            "convert".to_owned(),
            Item::Module(Rc::new(RefCell::new(std_convert_scope))),
        );
        std_scope.items.insert(
            "array".to_owned(),
            Item::Module(Rc::new(RefCell::new(std_array_scope))),
        );

        let mut items = HashMap::with_capacity(3);
        let builtin_function_dbg = FunctionType::new_dbg();
        let builtin_function_assert = FunctionType::new_assert();
        items.insert(
            builtin_function_dbg.identifier(),
            Item::Type(Type::Function(builtin_function_dbg)),
        );
        items.insert(
            builtin_function_assert.identifier(),
            Item::Type(Type::Function(builtin_function_assert)),
        );
        items.insert(
            "std".to_owned(),
            Item::Module(Rc::new(RefCell::new(std_scope))),
        );
        items
    }
}
