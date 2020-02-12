//!
//! The semantic analyzer type element.
//!

pub mod function;
pub mod structure;

use std::cell::RefCell;
use std::convert::TryFrom;
use std::fmt;
use std::rc::Rc;

use num_bigint::BigInt;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::analyzer::error::Error;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::analyzer::translation_hint::TranslationHint;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::Element;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::Scope;
use crate::syntax::Identifier;
use crate::syntax::TypeVariant;
use crate::syntax::Variant;

use self::function::Function;
use self::structure::Structure;

#[derive(Debug, Clone)]
pub enum Type {
    Unit,
    Boolean,
    IntegerUnsigned {
        bitlength: usize,
    },
    IntegerSigned {
        bitlength: usize,
    },
    Field,
    String,
    Range {
        r#type: Box<Self>,
    },
    RangeInclusive {
        r#type: Box<Self>,
    },
    Array {
        r#type: Box<Self>,
        size: usize,
    },
    Tuple {
        types: Vec<Self>,
    },
    Structure(Structure),
    Enumeration {
        identifier: String,
        unique_id: usize,
        bitlength: usize,
        scope: Rc<RefCell<Scope>>,
    },
    Function(Function),
}

pub static mut UNIQUE_ID: usize = 0;

impl Default for Type {
    fn default() -> Self {
        Self::Unit
    }
}

impl Type {
    pub fn unit() -> Self {
        Self::Unit
    }

    pub fn boolean() -> Self {
        Self::Boolean
    }

    pub fn integer_unsigned(bitlength: usize) -> Self {
        Self::IntegerUnsigned { bitlength }
    }

    pub fn integer_signed(bitlength: usize) -> Self {
        Self::IntegerSigned { bitlength }
    }

    pub fn integer(is_signed: bool, bitlength: usize) -> Self {
        if is_signed {
            Self::integer_signed(bitlength)
        } else {
            Self::integer_unsigned(bitlength)
        }
    }

    pub fn field() -> Self {
        Self::Field
    }

    pub fn scalar(is_signed: bool, bitlength: usize) -> Self {
        if is_signed {
            Self::integer_signed(bitlength)
        } else {
            match bitlength {
                crate::BITLENGTH_BOOLEAN => Self::Boolean,
                crate::BITLENGTH_FIELD => Self::Field,
                bitlength => Self::integer_unsigned(bitlength),
            }
        }
    }

    pub fn string() -> Self {
        Self::String
    }

    pub fn range(r#type: Self) -> Self {
        Self::Range {
            r#type: Box::new(r#type),
        }
    }

    pub fn range_inclusive(r#type: Self) -> Self {
        Self::RangeInclusive {
            r#type: Box::new(r#type),
        }
    }

    pub fn array(r#type: Self, size: usize) -> Self {
        Self::Array {
            r#type: Box::new(r#type),
            size,
        }
    }

    pub fn tuple(types: Vec<Self>) -> Self {
        Self::Tuple { types }
    }

    pub fn structure(
        identifier: String,
        unique_id: usize,
        fields: Vec<(String, Self)>,
        scope_parent: Option<Rc<RefCell<Scope>>>,
    ) -> Self {
        Self::Structure(Structure::new(identifier, unique_id, fields, scope_parent))
    }

    pub fn enumeration(
        identifier: Identifier,
        unique_id: usize,
        variants: Vec<Variant>,
        scope_parent: Option<Rc<RefCell<Scope>>>,
    ) -> Result<Self, Error> {
        let scope = Rc::new(RefCell::new(Scope::new(scope_parent)));

        let mut variants_bigint = Vec::with_capacity(variants.len());
        for variant in variants.into_iter() {
            let value = IntegerConstant::try_from(&variant.literal)
                .map_err(|error| Error::InferenceConstant(variant.identifier.location, error))?;
            variants_bigint.push((variant.identifier, value.value));
        }
        let bigints: Vec<&BigInt> = variants_bigint.iter().map(|variant| &variant.1).collect();
        let minimal_bitlength = IntegerConstant::minimal_bitlength_bigints(bigints.as_slice(), false)
            .map_err(|error| Error::InferenceConstant(identifier.location, error))?;

        for (identifier, value) in variants_bigint.into_iter() {
            let location = identifier.location;
            let constant = IntegerConstant::new(value, false, minimal_bitlength);
            scope
                .borrow_mut()
                .declare_constant(identifier.name, Constant::Integer(constant))
                .map_err(|error| Error::Scope(location, error))?;
        }

        let enumeration = Self::Enumeration {
            identifier: identifier.name,
            unique_id,
            bitlength: minimal_bitlength,
            scope: scope.clone(),
        };
        scope
            .borrow_mut()
            .declare_type("Self".to_owned(), enumeration.clone())
            .expect(crate::semantic::PANIC_SELF_ALIAS_DECLARATION);

        Ok(enumeration)
    }

    pub fn new_assert_function() -> Self {
        Self::Function(Function::new_assert())
    }

    pub fn new_dbg_function() -> Self {
        Self::Function(Function::new_dbg())
    }

    pub fn new_std_function(builtin_identifier: BuiltinIdentifier) -> Self {
        Self::Function(Function::new_std(builtin_identifier))
    }

    pub fn new_user_defined_function(
        identifier: String,
        unique_id: usize,
        arguments: Vec<(String, Self)>,
        return_type: Self,
    ) -> Self {
        Self::Function(Function::new_user_defined(
            identifier,
            unique_id,
            arguments,
            return_type,
        ))
    }

    pub fn size(&self) -> usize {
        match self {
            Self::Unit => 0,
            Self::Boolean => 1,
            Self::IntegerUnsigned { .. } => 1,
            Self::IntegerSigned { .. } => 1,
            Self::Field => 1,
            Self::String { .. } => 0,
            Self::Range { .. } => 0,
            Self::RangeInclusive { .. } => 0,
            Self::Array { r#type, size } => r#type.size() * size,
            Self::Tuple { types } => types.iter().map(|r#type| r#type.size()).sum(),
            Self::Structure(structure) => structure
                .fields
                .iter()
                .map(|(_name, r#type)| r#type.size())
                .sum(),
            Self::Enumeration { .. } => 1,
            Self::Function { .. } => 0,
        }
    }

    pub fn is_scalar(&self) -> bool {
        match self {
            Self::Boolean => true,
            Self::IntegerUnsigned { .. } => true,
            Self::IntegerSigned { .. } => true,
            Self::Field => true,
            _ => false,
        }
    }

    pub fn is_scalar_unsigned(&self) -> bool {
        match self {
            Self::IntegerUnsigned { .. } => true,
            Self::Field => true,
            _ => false,
        }
    }

    pub fn is_scalar_signed(&self) -> bool {
        match self {
            Self::IntegerSigned { .. } => true,
            _ => false,
        }
    }

    pub fn is_bit_array(&self) -> bool {
        match self {
            Self::Array { r#type, .. } => **r#type == Self::boolean(),
            _ => false,
        }
    }

    pub fn is_byte_array(&self) -> bool {
        match self {
            Self::Array { r#type, .. } => **r#type == Self::integer_unsigned(crate::BITLENGTH_BYTE),
            _ => false,
        }
    }

    pub fn is_scalar_array(&self) -> bool {
        match self {
            Self::Array { r#type, .. } => r#type.is_scalar(),
            _ => false,
        }
    }

    pub fn from_type_variant(
        type_variant: &TypeVariant,
        scope: Rc<RefCell<Scope>>,
    ) -> Result<Self, Error> {
        Ok(match type_variant {
            TypeVariant::Unit => Self::unit(),
            TypeVariant::Boolean => Self::boolean(),
            TypeVariant::IntegerUnsigned { bitlength } => Self::integer_unsigned(*bitlength),
            TypeVariant::IntegerSigned { bitlength } => Self::integer_signed(*bitlength),
            TypeVariant::Field => Self::field(),
            TypeVariant::Array { inner, size } => {
                Self::array(Self::from_type_variant(&*inner, scope)?, {
                    let location = size.location;
                    IntegerConstant::try_from(size)
                        .map_err(|error| Error::InferenceConstant(location, error))?
                        .to_usize()
                        .map_err(|error| Error::InferenceConstant(location, error))?
                })
            }
            TypeVariant::Tuple { inners } => {
                let mut types = Vec::with_capacity(inners.len());
                for inner in inners.iter() {
                    types.push(Self::from_type_variant(inner, scope.clone())?);
                }
                Self::tuple(types)
            }
            TypeVariant::Alias { path } => {
                let location = path.location;
                match ExpressionAnalyzer::new_without_bytecode(scope)
                    .expression(path.to_owned(), TranslationHint::TypeExpression)?
                {
                    Element::Type(r#type) => r#type,
                    element => {
                        return Err(Error::TypeAliasDoesNotPointToType(
                            location,
                            element.to_string(),
                        ))
                    }
                }
            }
        })
    }

    pub fn from_element(element: &Element, scope: Rc<RefCell<Scope>>) -> Result<Self, Error> {
        Ok(match element {
            Element::Value(value) => value.r#type(),
            Element::Constant(constant) => constant.r#type(),
            Element::Type(r#type) => r#type.to_owned(),
            Element::Path(path) => match Scope::resolve_path(scope, &path)? {
                ScopeItem::Variable(variable) => variable.r#type,
                ScopeItem::Constant(constant) => constant.r#type(),
                ScopeItem::Static(r#static) => r#static.data.r#type(),
                _ => panic!(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS),
            },
            Element::Place(place) => place.r#type.to_owned(),

            _ => panic!(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS),
        })
    }
}

impl PartialEq<Type> for Type {
    fn eq(&self, other: &Type) -> bool {
        match (self, other) {
            (Self::Unit, Self::Unit) => true,
            (Self::Boolean, Self::Boolean) => true,
            (Self::IntegerUnsigned { bitlength: b1 }, Self::IntegerUnsigned { bitlength: b2 }) => {
                b1 == b2
            }
            (Self::IntegerSigned { bitlength: b1 }, Self::IntegerSigned { bitlength: b2 }) => {
                b1 == b2
            }
            (Self::Field, Self::Field) => true,
            (Self::String, Self::String) => true,
            (Self::Range { r#type: type_1 }, Self::Range { r#type: type_2 }) => type_1 == type_2,
            (Self::RangeInclusive { r#type: type_1 }, Self::RangeInclusive { r#type: type_2 }) => {
                type_1 == type_2
            }
            (
                Self::Array {
                    r#type: type_1,
                    size: size_1,
                },
                Self::Array {
                    r#type: type_2,
                    size: size_2,
                },
            ) => type_1 == type_2 && size_1 == size_2,
            (Self::Tuple { types: types_1 }, Self::Tuple { types: types_2 }) => types_1 == types_2,
            (Self::Structure(structure_1), Self::Structure(structure_2)) => {
                structure_1.unique_id == structure_2.unique_id
            }
            (
                Self::Enumeration {
                    unique_id: unique_id_1,
                    ..
                },
                Self::Enumeration {
                    unique_id: unique_id_2,
                    ..
                },
            ) => unique_id_1 == unique_id_2,
            _ => false,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "()"),
            Self::Boolean => write!(f, "bool"),
            Self::IntegerUnsigned { bitlength } => write!(f, "u{}", bitlength),
            Self::IntegerSigned { bitlength } => write!(f, "i{}", bitlength),
            Self::Field => write!(f, "field"),
            Self::String => write!(f, "str"),
            Self::Range { r#type } => write!(f, "{0} .. {0}", r#type),
            Self::RangeInclusive { r#type } => write!(f, "{0} ..= {0}", r#type),
            Self::Array { r#type, size } => write!(f, "[{}; {}]", r#type, size),
            Self::Tuple { types } => write!(
                f,
                "({})",
                types
                    .iter()
                    .map(|r#type| r#type.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Structure(inner) => write!(f, "{}", inner),
            Self::Enumeration {
                identifier,
                unique_id,
                ..
            } => write!(f, "enum <{}> {}", unique_id, identifier),
            Self::Function(inner) => write!(f, "{}", inner),
        }
    }
}
