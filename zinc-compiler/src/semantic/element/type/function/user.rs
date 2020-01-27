//!
//! The semantic analyzer user-defined function type element.
//!

use std::fmt;

use crate::semantic::Type;

#[derive(Debug, Clone)]
pub struct UserDefinedFunction {
    pub identifier: String,
    pub arguments: Vec<(String, Type)>,
    pub return_type: Box<Type>,
}

impl UserDefinedFunction {
    pub fn new(identifier: String, arguments: Vec<(String, Type)>, return_type: Type) -> Self {
        Self {
            identifier,
            arguments,
            return_type: Box::new(return_type),
        }
    }
}

impl fmt::Display for UserDefinedFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "fn {}({}) -> {}",
            self.identifier,
            self.arguments
                .iter()
                .map(|(name, r#type)| format!("{}: {}", name, r#type))
                .collect::<Vec<String>>()
                .join(", "),
            self.return_type,
        )
    }
}
