//!
//! The semantic analyzer type element.
//!

use std::fmt;

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
        variants: Vec<(String, usize)>,
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

    pub fn new_enumeration(identifier: String, variants: Vec<(String, usize)>) -> Self {
        Self::Enumeration {
            identifier,
            variants,
        }
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
            Self::String { .. } => 1,
        }
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
            Self::Enumeration {
                identifier,
                variants,
            } => write!(
                f,
                "enum {} {{ {} }}",
                identifier,
                variants
                    .iter()
                    .map(|(name, value)| format!("{} = {}", name, value))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
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
