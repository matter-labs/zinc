//!
//! The semantic analyzer scope.
//!

mod tests;

pub mod error;
pub mod item;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::lexical::Keyword;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::path::Path;
use crate::semantic::element::r#type::function::Function as FunctionType;
use crate::semantic::element::r#type::Type;
use crate::semantic::Error as SemanticError;
use crate::syntax::Identifier;

use self::error::Error;
use self::item::r#static::Static as StaticItem;
use self::item::variable::Variable as VariableItem;
use self::item::Item;
use self::item::Variant as ItemVariant;

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

    pub fn declare_item(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        item: Item,
    ) -> Result<(), Error> {
        if let Ok(item) = Self::resolve_item(scope.clone(), &identifier.name) {
            return Err(Error::ItemRedeclared(
                identifier.name,
                item.location.unwrap_or_default(),
            ));
        }
        scope.borrow_mut().items.insert(identifier.name, item);
        Ok(())
    }

    pub fn declare_variable(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        variable: VariableItem,
    ) -> Result<(), Error> {
        if let Ok(item) = Self::resolve_item(scope.clone(), &identifier.name) {
            return Err(Error::ItemRedeclared(
                identifier.name,
                item.location.unwrap_or_default(),
            ));
        }
        scope.borrow_mut().items.insert(
            identifier.name,
            Item::new(ItemVariant::Variable(variable), Some(identifier.location)),
        );
        Ok(())
    }

    pub fn declare_constant(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        constant: Constant,
    ) -> Result<(), Error> {
        if let Ok(item) = Self::resolve_item(scope.clone(), &identifier.name) {
            return Err(Error::ItemRedeclared(
                identifier.name,
                item.location.unwrap_or_default(),
            ));
        }
        scope.borrow_mut().items.insert(
            identifier.name,
            Item::new(ItemVariant::Constant(constant), Some(identifier.location)),
        );
        Ok(())
    }

    pub fn declare_static(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        r#static: StaticItem,
    ) -> Result<(), Error> {
        if let Ok(item) = Self::resolve_item(scope.clone(), &identifier.name) {
            return Err(Error::ItemRedeclared(
                identifier.name,
                item.location.unwrap_or_default(),
            ));
        }
        scope.borrow_mut().items.insert(
            identifier.name,
            Item::new(ItemVariant::Static(r#static), Some(identifier.location)),
        );
        Ok(())
    }

    pub fn declare_type(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        r#type: Type,
    ) -> Result<(), Error> {
        if let Ok(item) = Self::resolve_item(scope.clone(), &identifier.name) {
            return Err(Error::ItemRedeclared(
                identifier.name,
                item.location.unwrap_or_default(),
            ));
        }
        scope.borrow_mut().items.insert(
            identifier.name,
            Item::new(ItemVariant::Type(r#type), Some(identifier.location)),
        );
        Ok(())
    }

    pub fn declare_module(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        module: Rc<RefCell<Scope>>,
    ) -> Result<(), Error> {
        if let Ok(item) = Self::resolve_item(scope.clone(), &identifier.name) {
            return Err(Error::ItemRedeclared(
                identifier.name,
                item.location.unwrap_or_default(),
            ));
        }
        scope.borrow_mut().items.insert(
            identifier.name,
            Item::new(ItemVariant::Module(module), Some(identifier.location)),
        );
        Ok(())
    }

    pub fn declare_self(&mut self, r#type: Type) {
        self.items.insert(
            Keyword::AliasSelf.to_string(),
            Item::new(ItemVariant::Type(r#type), None),
        );
    }

    pub fn resolve_path(scope: Rc<RefCell<Scope>>, path: &Path) -> Result<Item, SemanticError> {
        let mut current_scope = scope;

        for (index, identifier) in path.elements.iter().enumerate() {
            let item = Self::resolve_item(current_scope.clone(), &identifier.name)
                .map_err(|error| SemanticError::Scope(identifier.location, error))?;

            if index == path.elements.len() - 1 {
                return Ok(item);
            }

            match item.variant {
                ItemVariant::Module(ref scope) => current_scope = scope.to_owned(),
                ItemVariant::Type(Type::Enumeration { ref scope, .. }) => {
                    current_scope = scope.to_owned()
                }
                ItemVariant::Type(Type::Structure(ref structure)) => {
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

    // TODO: refactor towards the structure hierarchy
    pub const TYPE_ID_STD_CRYPTO_ECC_POINT: usize = 0;
    pub const TYPE_ID_STD_CRYPTO_SCHNORR_PUBLIC_KEY: usize = 1;
    pub const TYPE_ID_STD_CRYPTO_SCHNORR_SIGNATURE: usize = 2;
    pub const TYPE_ID_FIRST_AVAILABLE: usize = 3;

    fn default_items() -> HashMap<String, Item> {
        let mut std_crypto_scope = Scope::default();
        let std_crypto_sha256 = FunctionType::new_std(BuiltinIdentifier::CryptoSha256);
        let std_crypto_pedersen = FunctionType::new_std(BuiltinIdentifier::CryptoPedersen);
        let std_crypto_schnorr_verify =
            FunctionType::new_std(BuiltinIdentifier::CryptoSchnorrVerify);
        std_crypto_scope.items.insert(
            std_crypto_sha256.identifier(),
            Item::new(ItemVariant::Type(Type::Function(std_crypto_sha256)), None),
        );
        std_crypto_scope.items.insert(
            std_crypto_pedersen.identifier(),
            Item::new(ItemVariant::Type(Type::Function(std_crypto_pedersen)), None),
        );
        let mut std_crypto_schnorr = Scope::default();
        std_crypto_schnorr.items.insert(
            std_crypto_schnorr_verify.identifier(),
            Item::new(
                ItemVariant::Type(Type::Function(std_crypto_schnorr_verify)),
                None,
            ),
        );
        std_crypto_scope.items.insert(
            "schnorr".to_owned(),
            Item::new(
                ItemVariant::Module(Rc::new(RefCell::new(std_crypto_schnorr))),
                None,
            ),
        );

        let mut std_convert_scope = Scope::default();
        let std_convert_to_bits = FunctionType::new_std(BuiltinIdentifier::ToBits);
        let std_convert_from_bits_unsigned =
            FunctionType::new_std(BuiltinIdentifier::UnsignedFromBits);
        let std_convert_from_bits_signed = FunctionType::new_std(BuiltinIdentifier::SignedFromBits);
        let std_convert_from_bits_field = FunctionType::new_std(BuiltinIdentifier::FieldFromBits);
        std_convert_scope.items.insert(
            std_convert_to_bits.identifier(),
            Item::new(ItemVariant::Type(Type::Function(std_convert_to_bits)), None),
        );
        std_convert_scope.items.insert(
            std_convert_from_bits_unsigned.identifier(),
            Item::new(
                ItemVariant::Type(Type::Function(std_convert_from_bits_unsigned)),
                None,
            ),
        );
        std_convert_scope.items.insert(
            std_convert_from_bits_signed.identifier(),
            Item::new(
                ItemVariant::Type(Type::Function(std_convert_from_bits_signed)),
                None,
            ),
        );
        std_convert_scope.items.insert(
            std_convert_from_bits_field.identifier(),
            Item::new(
                ItemVariant::Type(Type::Function(std_convert_from_bits_field)),
                None,
            ),
        );

        let mut std_array_scope = Scope::default();
        let std_array_reverse = FunctionType::new_std(BuiltinIdentifier::ArrayReverse);
        let std_array_truncate = FunctionType::new_std(BuiltinIdentifier::ArrayTruncate);
        let std_array_pad = FunctionType::new_std(BuiltinIdentifier::ArrayPad);
        std_array_scope.items.insert(
            std_array_reverse.identifier(),
            Item::new(ItemVariant::Type(Type::Function(std_array_reverse)), None),
        );
        std_array_scope.items.insert(
            std_array_truncate.identifier(),
            Item::new(ItemVariant::Type(Type::Function(std_array_truncate)), None),
        );
        std_array_scope.items.insert(
            std_array_pad.identifier(),
            Item::new(ItemVariant::Type(Type::Function(std_array_pad)), None),
        );

        let mut std_scope = Scope::default();
        std_scope.items.insert(
            "crypto".to_owned(),
            Item::new(
                ItemVariant::Module(Rc::new(RefCell::new(std_crypto_scope))),
                None,
            ),
        );
        std_scope.items.insert(
            "convert".to_owned(),
            Item::new(
                ItemVariant::Module(Rc::new(RefCell::new(std_convert_scope))),
                None,
            ),
        );
        std_scope.items.insert(
            "array".to_owned(),
            Item::new(
                ItemVariant::Module(Rc::new(RefCell::new(std_array_scope))),
                None,
            ),
        );

        let mut items = HashMap::with_capacity(3);
        let builtin_function_dbg = FunctionType::new_dbg();
        let builtin_function_assert = FunctionType::new_assert();
        items.insert(
            builtin_function_dbg.identifier(),
            Item::new(
                ItemVariant::Type(Type::Function(builtin_function_dbg)),
                None,
            ),
        );
        items.insert(
            builtin_function_assert.identifier(),
            Item::new(
                ItemVariant::Type(Type::Function(builtin_function_assert)),
                None,
            ),
        );
        items.insert(
            "std".to_owned(),
            Item::new(ItemVariant::Module(Rc::new(RefCell::new(std_scope))), None),
        );
        items
    }
}
