//!
//! Transpiler element.
//!

mod descriptor;
mod permanent;
mod temporary;
mod r#type;

pub use self::descriptor::Descriptor;
pub use self::permanent::Element as PermanentElement;
pub use self::r#type::Element as TypeElement;
pub use self::temporary::Element as TemporaryElement;

use std::fmt;

use parser::TypeVariant;

#[derive(Debug)]
pub enum Element {
    Temporary(TemporaryElement),
    Permanent(PermanentElement),
    Unit,
    Type(TypeElement),
    ConstantBoolean(bool),
    ConstantNumeric(usize),
    ConstantString(String),
}

impl Element {
    pub fn type_variant(&self) -> TypeVariant {
        match self {
            Self::Temporary(element) => element.type_variant.clone(),
            Self::Permanent(element) => element.type_variant(),
            Self::Unit => TypeVariant::Unit,
            Self::Type(element) => element.type_variant.clone(),
            _ => panic!("Always checked by some branches above"),
        }
    }
}

impl Into<String> for Element {
    fn into(self) -> String {
        match self {
            Self::Temporary(element) => element.into(),
            Self::Permanent(element) => element.into(),
            Self::Unit => "()".to_owned(),
            Self::Type(element) => element.into(),
            Self::ConstantBoolean(element) => element.to_string(),
            Self::ConstantNumeric(element) => element.to_string(),
            Self::ConstantString(element) => element,
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Temporary(element) => write!(f, "{}", element),
            Self::Permanent(element) => write!(f, "{}", element),
            Self::Unit => write!(f, "()"),
            Self::Type(element) => write!(f, "{}", element),
            Self::ConstantBoolean(element) => write!(f, "{}", element),
            Self::ConstantNumeric(element) => write!(f, "{}", element),
            Self::ConstantString(element) => write!(f, "{}", element),
        }
    }
}
