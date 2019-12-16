//!
//! The semantic analyzer type element.
//!

use std::cell::RefCell;
use std::convert::TryFrom;
use std::fmt;
use std::rc::Rc;

use crate::semantic::Constant;
use crate::semantic::Element;
use crate::semantic::Error;
use crate::semantic::ExpressionAnalyzer;
use crate::semantic::IntegerConstant;
use crate::semantic::ResolutionHint;
use crate::semantic::Scope;
use crate::semantic::ScopeItem;
use crate::syntax::Identifier;
use crate::syntax::IntegerLiteral;
use crate::syntax::TypeVariant;
use crate::syntax::Variant;

#[derive(Clone, PartialEq)]
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
    Array {
        r#type: Box<Self>,
        size: usize,
    },
    Tuple {
        types: Vec<Self>,
    },
    Structure {
        identifier: String,
        fields: Vec<(String, Self)>,
    },
    Enumeration {
        identifier: String,
        scope: Scope,
    },
    Function {
        identifier: String,
        arguments: Vec<(String, Self)>,
        return_type: Box<Self>,
    },
    String,
}

impl Type {
    pub fn new_integer(is_signed: bool, bitlength: usize) -> Self {
        if is_signed {
            Self::new_integer_signed(bitlength)
        } else {
            Self::new_integer_unsigned(bitlength)
        }
    }

    pub fn new_numeric(is_signed: bool, bitlength: usize) -> Self {
        if is_signed {
            Self::new_integer_signed(bitlength)
        } else {
            match bitlength {
                crate::BITLENGTH_BOOLEAN => Self::Boolean,
                crate::BITLENGTH_FIELD => Self::Field,
                bitlength => Self::new_integer_unsigned(bitlength),
            }
        }
    }

    pub fn new_unit() -> Self {
        Self::Unit
    }

    pub fn new_boolean() -> Self {
        Self::Boolean
    }

    pub fn new_integer_unsigned(bitlength: usize) -> Self {
        Self::IntegerUnsigned { bitlength }
    }

    pub fn new_integer_signed(bitlength: usize) -> Self {
        Self::IntegerSigned { bitlength }
    }

    pub fn new_field() -> Self {
        Self::Field
    }

    pub fn new_array(r#type: Self, size: usize) -> Self {
        Self::Array {
            r#type: Box::new(r#type),
            size,
        }
    }

    pub fn new_tuple(types: Vec<Self>) -> Self {
        Self::Tuple { types }
    }

    pub fn new_structure(identifier: String, fields: Vec<(String, Self)>) -> Self {
        Self::Structure { identifier, fields }
    }

    pub fn new_enumeration(identifier: Identifier, variants: Vec<Variant>) -> Result<Self, Error> {
        let mut scope = Scope::new(None);

        let literals: Vec<&IntegerLiteral> =
            variants.iter().map(|variant| &variant.literal).collect();
        let enough_bitlength = IntegerConstant::infer_enough_bitlength(literals.as_slice())
            .map_err(|error| Error::InferenceConstant(identifier.location, error))?;

        for variant in variants.into_iter() {
            let location = variant.literal.location;
            let mut constant = IntegerConstant::try_from(&variant.literal)
                .map_err(|error| Error::InferenceConstant(location, error))?;
            constant.cast(false, enough_bitlength);
            scope
                .declare_constant(variant.identifier.name, Constant::Integer(constant))
                .map_err(|error| Error::Scope(location, error))?;
        }
        Ok(Self::Enumeration {
            identifier: identifier.name,
            scope,
        })
    }

    pub fn new_function(
        identifier: String,
        arguments: Vec<(String, Self)>,
        return_type: Self,
    ) -> Self {
        Self::Function {
            identifier,
            arguments,
            return_type: Box::new(return_type),
        }
    }

    pub fn new_string() -> Self {
        Self::String
    }

    pub fn size(&self) -> usize {
        match self {
            Self::Unit => 0,
            Self::Boolean => 1,
            Self::IntegerUnsigned { .. } => 1,
            Self::IntegerSigned { .. } => 1,
            Self::Field => 1,
            Self::Array { r#type, size } => r#type.size() * size,
            Self::Tuple { types } => types.iter().map(|r#type| r#type.size()).sum(),
            Self::Structure { fields, .. } => {
                fields.iter().map(|(_name, r#type)| r#type.size()).sum()
            }
            Self::Enumeration { .. } => 1,
            Self::Function { .. } => 0,
            Self::String { .. } => 0,
        }
    }

    pub fn from_type_variant(
        type_variant: TypeVariant,
        scope: Rc<RefCell<Scope>>,
    ) -> Result<Self, Error> {
        Ok(match type_variant {
            TypeVariant::Unit => Type::Unit,
            TypeVariant::Boolean => Type::Boolean,
            TypeVariant::IntegerUnsigned { bitlength } => Type::IntegerUnsigned { bitlength },
            TypeVariant::IntegerSigned { bitlength } => Type::IntegerSigned { bitlength },
            TypeVariant::Field => Type::Field,
            TypeVariant::Array { type_variant, size } => Type::Array {
                r#type: Self::from_type_variant(*type_variant, scope).map(Box::new)?,
                size: size.into(),
            },
            TypeVariant::Tuple { type_variants } => {
                let mut types = Vec::with_capacity(type_variants.len());
                for type_variant in type_variants.into_iter() {
                    types.push(Self::from_type_variant(type_variant, scope.clone())?);
                }
                Type::Tuple { types }
            }
            TypeVariant::Alias { path } => {
                let location = path.location;
                match ExpressionAnalyzer::new_without_bytecode(scope)
                    .expression(path, ResolutionHint::TypeExpression)?
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
            Element::Place(place) => match Scope::resolve_place(scope, &place)? {
                ScopeItem::Variable(variable) => variable.r#type,
                ScopeItem::Constant(constant) => constant.r#type(),
                ScopeItem::Static(r#static) => r#static.data.r#type(),
                _ => panic!(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS),
            },
            Element::Value(value) => value.r#type(),
            Element::Constant(constant) => constant.r#type(),
            Element::Type(r#type) => r#type.to_owned(),
            _ => panic!(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS),
        })
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "()"),
            Self::Boolean => write!(f, "bool"),
            Self::IntegerUnsigned { bitlength } => write!(f, "u{}", bitlength),
            Self::IntegerSigned { bitlength } => write!(f, "i{}", bitlength),
            Self::Field => write!(f, "field"),
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
            Self::Structure { identifier, fields } => write!(
                f,
                "struct {} {{ {} }}",
                identifier,
                fields
                    .iter()
                    .map(|(name, r#type)| format!("{}: {}", name, r#type))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Enumeration { identifier, scope } => write!(f, "enum {} {:?}", identifier, scope),
            Self::Function {
                identifier,
                arguments,
                return_type,
            } => write!(
                f,
                "fn {}({}) -> {}",
                identifier,
                arguments
                    .iter()
                    .map(|(name, r#type)| format!("{}: {}", name, r#type))
                    .collect::<Vec<String>>()
                    .join(", "),
                return_type,
            ),
            Self::String => write!(f, "&str"),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}
