//!
//! The semantic analyzer scope item.
//!

pub mod constant;
pub mod field;
pub mod index;
pub mod module;
pub mod r#type;
pub mod variable;
pub mod variant;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::generator::statement::Statement as GeneratorStatement;
use crate::semantic::error::Error;
use zinc_lexical::Location;

use self::constant::Constant;
use self::field::Field;
use self::module::Module;
use self::r#type::Type;
use self::variable::Variable;
use self::variant::Variant;

///
/// An item declared within a scope.
///
/// Items are variables, constants, types, modules, etc.
///
/// Items are not defined at once. At first, they are only declared. Then, they are hoisted to the
/// top of their scope, where the item names are stored with their syntax representations.
/// When an item is referenced for the first time, it is **defined**.
///
/// **Definition** means that the syntax construction is analyzed for its semantic meaning and some
/// actions related to the item type are taken, e.g. a constant value is assigned to the item name.
/// This approach allows to reference items which were declared **above** the item being analyzed.
///
/// ```
/// const A: u8 = C;
///
/// const B: u8 = 2;
///
/// const C: u8 = 40 + B;
/// ```
///
/// The items are declared and defined in the following order:
///
/// 1. Hoist and declare item `A`.
/// 2. Hoist and declare item `B`.
/// 3. Hoist and declare item `C`.
/// 4. Define item `A`, which references the declared, but not defined, item `C`.
/// 5. Define item `C`, which was referenced by the item `A` definition, and also references the item `B`.
/// 6. Define item `B`, which was referenced by the item `C`.
///
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Item {
    /// The variable item. See the inner element description.
    Variable(Variable),
    /// The contract field item. See the inner element description.
    Field(Field),
    /// The constant item. See the inner element description.
    Constant(Constant),
    /// The enumeration variant item. See the inner element description.
    Variant(Variant),
    /// The type item. See the inner element description.
    Type(Type),
    /// The module item. See the inner element description.
    Module(Module),
}

impl Item {
    ///
    /// Wraps the item into `Rc<RefCell<_>>` simplifying most of initializations.
    ///
    pub fn wrap(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    ///
    /// Internally defines the item.
    ///
    /// Has no effect if the item has been already defined.
    ///
    pub fn define(&self) -> Result<(), Error> {
        match self {
            Self::Variable(_) => {}
            Self::Field(_) => {}
            Self::Constant(inner) => {
                inner.define()?;
            }
            Self::Variant(_) => {}
            Self::Type(inner) => {
                inner.define()?;
            }
            Self::Module(inner) => {
                inner.define()?;
            }
        }

        Ok(())
    }

    ///
    /// The location where the item has been declared.
    ///
    pub fn location(&self) -> Option<Location> {
        match self {
            Self::Variable(inner) => inner.location,
            Self::Field(inner) => Some(inner.location),
            Self::Constant(inner) => Some(inner.location),
            Self::Variant(inner) => Some(inner.location),
            Self::Type(inner) => inner.location,
            Self::Module(inner) => inner.location,
        }
    }

    ///
    /// The globally allocated item ID.
    ///
    pub fn item_id(&self) -> usize {
        match self {
            Self::Variable(inner) => inner.item_id,
            Self::Field(inner) => inner.item_id,
            Self::Constant(inner) => inner.item_id,
            Self::Variant(inner) => inner.item_id,
            Self::Type(inner) => inner.item_id,
            Self::Module(inner) => inner.item_id,
        }
    }

    ///
    /// Extracts the intermediate representation from the element.
    ///
    pub fn get_intermediate(&self) -> Vec<GeneratorStatement> {
        match self {
            Self::Variable(_) => vec![],
            Self::Field(_) => vec![],
            Self::Constant(_) => vec![],
            Self::Variant(_) => vec![],
            Self::Type(inner) => inner.get_intermediate(),
            Self::Module(inner) => inner.get_intermediate(),
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Variable(inner) => write!(f, "variable {}", inner),
            Self::Field(inner) => write!(f, "field {}", inner),
            Self::Constant(inner) => write!(f, "constant {}", inner),
            Self::Variant(inner) => write!(f, "variant {}", inner),
            Self::Type(inner) => write!(f, "type {}", inner),
            Self::Module(inner) => write!(f, "module {}", inner),
        }
    }
}
