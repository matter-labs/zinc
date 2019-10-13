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

use parser::ExpressionOperand;

pub enum Element {
    Unit,
    Constant(String),
    Operand(ExpressionOperand),
    Temporary(TemporaryElement),
    Permanent(PermanentElement),
    Type(TypeElement),
}

impl Element {
    pub fn is_unit(&self) -> bool {
        match self {
            Self::Unit => true,
            _ => false,
        }
    }
}

impl Into<String> for Element {
    fn into(self) -> String {
        match self {
            Self::Unit => "()".to_owned(),
            Self::Constant(element) => element,
            Self::Operand(element) => element.to_string(),
            Self::Temporary(element) => element.into(),
            Self::Permanent(element) => element.into(),
            Self::Type(element) => element.into(),
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "()"),
            Self::Constant(element) => write!(f, "{}", element),
            Self::Operand(element) => write!(f, "{}", element),
            Self::Temporary(element) => write!(f, "{}", element),
            Self::Permanent(element) => write!(f, "{}", element),
            Self::Type(element) => write!(f, "{}", element),
        }
    }
}
