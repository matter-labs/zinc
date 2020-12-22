//!
//! The semantic analyzer type element.
//!

#[cfg(test)]
mod tests;

pub mod array;
pub mod contract;
pub mod enumeration;
pub mod function;
pub mod i_typed;
pub mod range;
pub mod range_inclusive;
pub mod structure;
pub mod tuple;

use std::cell::RefCell;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use zinc_lexical::Location;
use zinc_syntax::BlockExpression;
use zinc_syntax::Type as SyntaxType;
use zinc_syntax::TypeVariant as SyntaxTypeVariant;
use zinc_syntax::Variant;

use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::binding::Binding;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::intrinsic::IntrinsicTypeId;
use crate::semantic::scope::item::r#type::index::INDEX as TYPE_INDEX;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::Scope;

use self::array::Array;
use self::contract::field::Field as ContractField;
use self::contract::Contract;
use self::enumeration::Enumeration;
use self::function::Function;
use self::i_typed::ITyped;
use self::range::Range;
use self::range_inclusive::RangeInclusive;
use self::structure::Structure;
use self::tuple::Tuple;

///
/// The semantic type is converted from a syntax type during syntax analysis.
///
/// `Structure`, `Enumeration`, `Function`, `Contract` are resolved from the scope hierarchy.
///
#[derive(Debug, Clone)]
pub enum Type {
    /// The `()` type.
    Unit(Option<Location>),
    /// The `bool` type.
    Boolean(Option<Location>),
    /// The `u{N}` type.
    IntegerUnsigned {
        /// The location where the type appears in the code.
        location: Option<Location>,
        /// The integer type bitlength.
        bitlength: usize,
    },
    /// The `i{N}` type.
    IntegerSigned {
        /// The location where the type appears in the code.
        location: Option<Location>,
        /// The integer type bitlength.
        bitlength: usize,
    },
    /// The `field` type.
    Field(Option<Location>),
    /// The compile-time only type used mostly for `dbg!` format strings and `require` messages.
    String(Option<Location>),
    /// The compile-time only type used for loop bounds and array slicing.
    Range(Range),
    /// The compile-time only type used for loop bounds and array slicing.
    RangeInclusive(RangeInclusive),
    /// The ordinar array type.
    Array(Array),
    /// The ordinar tuple type.
    Tuple(Tuple),
    /// The ordinar structure type declared with a `struct` statement.
    Structure(Structure),
    /// The ordinar enumeration type declared with an `enum` statement.
    Enumeration(Enumeration),
    /// The special function type declared with an `fn` statement.
    Function(Function),
    /// The special contract type declared with a `contract` statement.
    Contract(Contract),
}

impl Type {
    ///
    /// A shortcut constructor.
    ///
    pub fn unit(location: Option<Location>) -> Self {
        Self::Unit(location)
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn boolean(location: Option<Location>) -> Self {
        Self::Boolean(location)
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn integer_unsigned(location: Option<Location>, bitlength: usize) -> Self {
        Self::IntegerUnsigned {
            location,
            bitlength,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn integer_signed(location: Option<Location>, bitlength: usize) -> Self {
        Self::IntegerSigned {
            location,
            bitlength,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn integer(location: Option<Location>, is_signed: bool, bitlength: usize) -> Self {
        if is_signed {
            Self::integer_signed(location, bitlength)
        } else {
            Self::integer_unsigned(location, bitlength)
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn field(location: Option<Location>) -> Self {
        Self::Field(location)
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn scalar(location: Option<Location>, is_signed: bool, bitlength: usize) -> Self {
        if is_signed {
            Self::integer_signed(location, bitlength)
        } else {
            match bitlength {
                zinc_const::bitlength::BOOLEAN => Self::boolean(location),
                zinc_const::bitlength::FIELD => Self::field(location),
                bitlength => Self::integer_unsigned(location, bitlength),
            }
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn string(location: Option<Location>) -> Self {
        Self::String(location)
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn range(location: Option<Location>, r#type: Self) -> Self {
        Self::Range(Range::new(location, Box::new(r#type)))
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn range_inclusive(location: Option<Location>, r#type: Self) -> Self {
        Self::RangeInclusive(RangeInclusive::new(location, Box::new(r#type)))
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn array(location: Option<Location>, r#type: Self, size: usize) -> Self {
        Self::Array(Array::new(location, Box::new(r#type), size))
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn tuple(location: Option<Location>, types: Vec<Self>) -> Self {
        Self::Tuple(Tuple::new(location, types))
    }

    ///
    /// A helper type constructor, which allocates a unique sequence ID for the type.
    ///
    pub fn structure(
        location: Option<Location>,
        identifier: String,
        fields: Vec<(String, Self)>,
        generics: Option<Vec<String>>,
        scope: Rc<RefCell<Scope>>,
    ) -> Self {
        let type_id = TYPE_INDEX.next(format!("structure {}", identifier));

        Self::Structure(Structure::new(
            location, identifier, type_id, fields, generics, None, scope,
        ))
    }

    ///
    /// A helper type constructor, which allocates a unique sequence ID for the type.
    ///
    pub fn enumeration(
        location: Location,
        identifier: String,
        variants: Vec<Variant>,
        generics: Vec<String>,
        scope: Rc<RefCell<Scope>>,
    ) -> Result<Self, Error> {
        let type_id = TYPE_INDEX.next(format!("enumeration {}", identifier));

        Enumeration::new(location, identifier, type_id, variants, generics, scope)
            .map(Self::Enumeration)
    }

    ///
    /// A helper type constructor, which allocates a unique sequence ID for the type.
    ///
    pub fn runtime_function(
        location: Location,
        identifier: String,
        bindings: Vec<Binding>,
        return_type: Self,
    ) -> (Self, usize) {
        let type_id = TYPE_INDEX.next(format!("function {}", identifier));

        (
            Self::Function(Function::runtime(
                location,
                identifier,
                type_id,
                bindings,
                return_type,
            )),
            type_id,
        )
    }

    ///
    /// A helper type constructor, which allocates a unique sequence ID for the type.
    ///
    pub fn constant_function(
        location: Location,
        identifier: String,
        bindings: Vec<Binding>,
        return_type: Self,
        body: BlockExpression,
    ) -> Self {
        let type_id = TYPE_INDEX.next(format!("function {}", identifier));

        Self::Function(Function::constant(
            location,
            identifier,
            type_id,
            bindings,
            return_type,
            body,
        ))
    }

    ///
    /// A helper type constructor, which allocates a unique sequence ID for the type.
    ///
    pub fn test_function(location: Location, identifier: String) -> (Self, usize) {
        let type_id = TYPE_INDEX.next(format!("function {}", identifier));

        (
            Self::Function(Function::test(location, identifier, type_id)),
            type_id,
        )
    }

    ///
    /// A helper type constructor, which allocates a unique sequence ID for the type.
    ///
    pub fn contract(
        location: Location,
        identifier: String,
        project: zinc_project::ManifestProject,
        fields: Vec<ContractField>,
        scope: Rc<RefCell<Scope>>,
    ) -> Result<Self, Error> {
        let type_id = TYPE_INDEX.next(format!("contract {}", identifier));

        Contract::new(location, identifier, project, type_id, fields, scope).map(Self::Contract)
    }

    ///
    /// Returns the type size in the virtual machine data stack.
    ///
    /// The contract size is one, since its fields are stored in the contract storage, and only
    /// its address is stored in the data stack.
    ///
    pub fn size(&self) -> usize {
        match self {
            Self::Unit(_) => 0,
            Self::Boolean(_) => 1,
            Self::IntegerUnsigned { .. } => 1,
            Self::IntegerSigned { .. } => 1,
            Self::Field(_) => 1,
            Self::String(_) => 0,
            Self::Range(_) => 0,
            Self::RangeInclusive(_) => 0,
            Self::Array(inner) => inner.r#type.size() * inner.size,
            Self::Tuple(inner) => inner.types.iter().map(|r#type| r#type.size()).sum(),
            Self::Structure(inner) => inner
                .fields
                .iter()
                .map(|(_name, r#type)| r#type.size())
                .sum(),
            Self::Enumeration(_inner) => 1,
            Self::Contract(_inner) => 1,
            Self::Function(_inner) => 0,
        }
    }

    ///
    /// Checks if the type is scalar (a primitive non-unit type).
    ///
    pub fn is_scalar(&self) -> bool {
        match self {
            Self::Boolean(_) => true,
            Self::IntegerUnsigned { .. } => true,
            Self::IntegerSigned { .. } => true,
            Self::Field(_) => true,
            Self::Enumeration { .. } => true,
            _ => false,
        }
    }

    ///
    /// Checks if the type is an unsigned scalar one (booleans, unsigned integers, fields and
    /// enumeration values).
    ///
    pub fn is_scalar_unsigned(&self) -> bool {
        match self {
            Self::Boolean(_) => true,
            Self::IntegerUnsigned { .. } => true,
            Self::Field(_) => true,
            Self::Enumeration { .. } => true,
            _ => false,
        }
    }

    ///
    /// Checks if the type is a signed scalar one (only signed integer for now).
    ///
    pub fn is_scalar_signed(&self) -> bool {
        matches!(self, Self::IntegerSigned { .. })
    }

    ///
    /// Checks if the type is an unsigned integer one (unsigned integers, fields and enumeration values).
    ///
    pub fn is_integer_unsigned(&self) -> bool {
        match self {
            Self::IntegerUnsigned { .. } => true,
            Self::Field(_) => true,
            Self::Enumeration { .. } => true,
            _ => false,
        }
    }

    ///
    /// Checks if the type is a boolean (bit) array.
    ///
    pub fn is_bit_array(&self) -> bool {
        match self {
            Self::Array(array) => array.r#type.deref() == &Self::boolean(None),
            _ => false,
        }
    }

    ///
    /// Checks if the type is an unsigned 8-bit integer (byte) array.
    ///
    pub fn is_byte_array(&self) -> bool {
        match self {
            Self::Array(array) => {
                array.r#type.deref() == &Self::integer_unsigned(None, zinc_const::bitlength::BYTE)
            }
            _ => false,
        }
    }

    ///
    /// Checks if the type is an array of scalars (a primitive non-unit type).
    ///
    pub fn is_scalar_array(&self) -> bool {
        match self {
            Self::Array(array) => array.r#type.is_scalar(),
            _ => false,
        }
    }

    ///
    /// Checks if the type is a manually declared function, that is, not an intrinsic one.
    ///
    pub fn is_source_function(&self) -> bool {
        match self {
            Self::Function(Function::Runtime(_)) => true,
            Self::Function(Function::Constant(_)) => true,
            _ => false,
        }
    }

    ///
    /// Checks if the type can be instantiated.
    ///
    /// Instantiation is currently impossible for strings, ranges, functions,
    /// and maps beyond the contract storage.
    ///
    pub fn is_instantiatable(&self, is_contract_field: bool) -> bool {
        match self {
            Self::Unit(_) => true,
            Self::Boolean(_) => true,
            Self::IntegerUnsigned { .. } => true,
            Self::IntegerSigned { .. } => true,
            Self::Field(_) => true,
            Self::String(_) => false,
            Self::Range(_) => false,
            Self::RangeInclusive(_) => false,
            Self::Array(inner) => inner.r#type.is_instantiatable(false),
            Self::Tuple(inner) => inner
                .types
                .iter()
                .all(|r#type| Self::is_instantiatable(r#type, false)),
            Self::Structure(inner) => {
                (is_contract_field || !self.is_mtreemap())
                    && inner
                        .fields
                        .iter()
                        .map(|(_name, r#type)| r#type)
                        .all(|r#type| Self::is_instantiatable(r#type, false))
                    && inner
                        .params
                        .to_owned()
                        .unwrap_or_default()
                        .iter()
                        .map(|(_name, r#type)| r#type)
                        .all(|r#type| Self::is_instantiatable(r#type, false))
            }
            Self::Enumeration(_) => true,
            Self::Function(_) => false,
            Self::Contract(inner) => inner
                .fields
                .iter()
                .map(|field| &field.r#type)
                .all(|r#type| Self::is_instantiatable(r#type, true)),
        }
    }

    ///
    /// Checks if the type is an `std::collections::MTreeMap`, which is treated specially.
    ///
    pub fn is_mtreemap(&self) -> bool {
        if let Self::Structure(structure) = self {
            structure.type_id == IntrinsicTypeId::StdCollectionsMTreeMap as usize
        } else {
            false
        }
    }

    ///
    /// Sets the generic arguments for the type.
    ///
    /// Returns an error if the type does not expect any generic arguments.
    ///
    pub fn set_generics(
        &mut self,
        location: Location,
        generics: Option<Vec<Type>>,
    ) -> Result<(), Error> {
        match self {
            Self::Structure(inner) => inner.set_generics(location, generics),
            ref r#type if generics.is_some() => Err(Error::TypeUnexpectedGenerics {
                location: self.location().unwrap_or(location),
                r#type: r#type.to_string(),
            }),
            _ => Ok(()),
        }
    }

    ///
    /// Resolves the semantic type from the syntax one.
    ///
    /// For primitive types, the semantic type is simply converted from the syntax tree.
    /// For complex type, the path is resolved in the `scope` tree.
    ///
    pub fn try_from_syntax(r#type: SyntaxType, scope: Rc<RefCell<Scope>>) -> Result<Self, Error> {
        let location = r#type.location;

        Ok(match r#type.variant {
            SyntaxTypeVariant::Unit => Self::unit(Some(location)),
            SyntaxTypeVariant::Boolean => Self::boolean(Some(location)),
            SyntaxTypeVariant::IntegerUnsigned { bitlength } => {
                Self::integer_unsigned(Some(location), bitlength)
            }
            SyntaxTypeVariant::IntegerSigned { bitlength } => {
                Self::integer_signed(Some(location), bitlength)
            }
            SyntaxTypeVariant::Field => Self::field(Some(location)),
            SyntaxTypeVariant::Array { inner, size } => {
                let r#type = Self::try_from_syntax(*inner, scope.clone())?;

                let size_location = size.location;
                let size = match ExpressionAnalyzer::new(scope, TranslationRule::Constant)
                    .analyze(size)?
                {
                    (Element::Constant(Constant::Integer(integer)), _intermediate) => {
                        integer.to_usize()?
                    }
                    (element, _intermediate) => {
                        return Err(Error::ExpressionNonConstantElement {
                            location: size_location,
                            found: element.to_string(),
                        });
                    }
                };

                Self::array(Some(location), r#type, size)
            }
            SyntaxTypeVariant::Tuple { inners } => {
                let mut types = Vec::with_capacity(inners.len());
                for inner in inners.into_iter() {
                    types.push(Self::try_from_syntax(inner, scope.clone())?);
                }
                Self::tuple(Some(location), types)
            }
            SyntaxTypeVariant::Alias { path, generics } => {
                let location = path.location;
                match ExpressionAnalyzer::new(scope.clone(), TranslationRule::Type).analyze(path)? {
                    (Element::Type(mut r#type), _intermediate) => {
                        let generics = if let Some(generics) = generics {
                            let mut semantic_generics = Vec::with_capacity(generics.len());
                            for generic in generics.into_iter() {
                                semantic_generics
                                    .push(Self::try_from_syntax(generic, scope.clone())?);
                            }
                            Some(semantic_generics)
                        } else {
                            None
                        };

                        r#type.set_generics(location, generics)?;
                        r#type
                    }
                    (element, _intermediate) => {
                        return Err(Error::TypeAliasExpectedType {
                            location,
                            found: element.to_string(),
                        });
                    }
                }
            }
        })
    }

    ///
    /// Gets the semantic element type where it is possible.
    ///
    /// This method should not be called for elements where the type is impossible to get.
    /// In such cases, the method will panic.
    ///
    pub fn from_element(element: &Element, scope: Rc<RefCell<Scope>>) -> Result<Self, Error> {
        Ok(match element {
            Element::Value(value) => value.r#type(),
            Element::Constant(constant) => constant.r#type(),
            Element::Type(r#type) => r#type.to_owned(),
            Element::Path(path) => match *Scope::resolve_path(scope, &path)?.borrow() {
                ScopeItem::Variable(ref variable) => {
                    let mut r#type = variable.r#type.to_owned();
                    r#type.set_location(path.last().location);
                    r#type
                }
                ScopeItem::Constant(ref constant) => {
                    let mut constant = constant.define()?;
                    constant.set_location(path.last().location);
                    constant.r#type()
                }
                ScopeItem::Type(ref r#type) => {
                    let mut r#type = r#type.define()?;
                    r#type.set_location(path.last().location);
                    r#type
                }
                _ => panic!(zinc_const::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
            },
            Element::Place(place) => {
                let mut r#type = place.r#type.to_owned();
                r#type.set_location(place.identifier.location);
                r#type
            }
            _ => panic!(zinc_const::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
        })
    }

    ///
    /// Sets the location for the type element.
    ///
    pub fn set_location(&mut self, value: Location) {
        match self {
            Self::Unit(location) => *location = Some(value),
            Self::Boolean(location) => *location = Some(value),
            Self::IntegerUnsigned { location, .. } => *location = Some(value),
            Self::IntegerSigned { location, .. } => *location = Some(value),
            Self::Field(location) => *location = Some(value),
            Self::String(location) => *location = Some(value),
            Self::Range(inner) => inner.location = Some(value),
            Self::RangeInclusive(inner) => inner.location = Some(value),
            Self::Array(inner) => inner.location = Some(value),
            Self::Tuple(inner) => inner.location = Some(value),
            Self::Structure(inner) => inner.location = Some(value),
            Self::Enumeration(inner) => inner.location = Some(value),
            Self::Function(inner) => inner.set_location(value),
            Self::Contract(inner) => inner.location = value,
        }
    }

    ///
    /// Returns the location of the type element.
    ///
    pub fn location(&self) -> Option<Location> {
        match self {
            Self::Unit(location) => *location,
            Self::Boolean(location) => *location,
            Self::IntegerUnsigned { location, .. } => *location,
            Self::IntegerSigned { location, .. } => *location,
            Self::Field(location) => *location,
            Self::String(location) => *location,
            Self::Range(inner) => inner.location,
            Self::RangeInclusive(inner) => inner.location,
            Self::Array(inner) => inner.location,
            Self::Tuple(inner) => inner.location,
            Self::Structure(inner) => inner.location,
            Self::Enumeration(inner) => inner.location,
            Self::Function(inner) => inner.location(),
            Self::Contract(inner) => Some(inner.location),
        }
    }
}

impl PartialEq<Type> for Type {
    fn eq(&self, other: &Type) -> bool {
        match (self, other) {
            (Self::Unit(_), Self::Unit(_)) => true,
            (Self::Boolean(_), Self::Boolean(_)) => true,
            (
                Self::IntegerUnsigned { bitlength: b1, .. },
                Self::IntegerUnsigned { bitlength: b2, .. },
            ) => b1 == b2,
            (
                Self::IntegerSigned { bitlength: b1, .. },
                Self::IntegerSigned { bitlength: b2, .. },
            ) => b1 == b2,
            (Self::Field(_), Self::Field(_)) => true,
            (Self::String(_), Self::String(_)) => true,
            (Self::Range(inner_1), Self::Range(inner_2)) => inner_1.r#type == inner_2.r#type,
            (Self::RangeInclusive(inner_1), Self::RangeInclusive(inner_2)) => {
                inner_1.r#type == inner_2.r#type
            }
            (Self::Array(inner_1), Self::Array(inner_2)) => {
                inner_1.r#type == inner_2.r#type && inner_1.size == inner_2.size
            }
            (Self::Tuple(inner_1), Self::Tuple(inner_2)) => inner_1.types == inner_2.types,
            (Self::Structure(inner_1), Self::Structure(inner_2)) => inner_1 == inner_2,
            (Self::Enumeration(inner_1), Self::Enumeration(inner_2)) => inner_1 == inner_2,
            (Self::Contract(inner_1), Self::Contract(inner_2)) => inner_1 == inner_2,
            _ => false,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unit(_) => write!(f, "()"),
            Self::Boolean(_) => write!(f, "bool"),
            Self::IntegerUnsigned { bitlength, .. } => write!(f, "u{}", bitlength),
            Self::IntegerSigned { bitlength, .. } => write!(f, "i{}", bitlength),
            Self::Field(_) => write!(f, "field"),
            Self::String(_) => write!(f, "str"),
            Self::Range(inner) => write!(f, "range {}", inner),
            Self::RangeInclusive(inner) => write!(f, "range inclusive {}", inner),
            Self::Array(inner) => write!(f, "array {}", inner),
            Self::Tuple(inner) => write!(f, "tuple {}", inner),
            Self::Structure(inner) => write!(f, "structure {}", inner),
            Self::Enumeration(inner) => write!(f, "enumeration {}", inner),
            Self::Function(inner) => write!(f, "function {}", inner),
            Self::Contract(inner) => write!(f, "contract {}", inner),
        }
    }
}
