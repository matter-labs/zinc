//!
//! The semantic analyzer user-defined function type element.
//!

use std::fmt;

use crate::semantic::element::r#type::Type;

#[derive(Debug, Clone)]
pub struct Function {
    pub identifier: String,
    pub unique_id: usize,
    pub arguments: Vec<(String, Type)>,
    pub return_type: Box<Type>,
}

impl Function {
    pub fn new(
        identifier: String,
        unique_id: usize,
        arguments: Vec<(String, Type)>,
        return_type: Type,
    ) -> Self {
        Self {
            identifier,
            arguments,
            return_type: Box::new(return_type),
            unique_id,
        }
    }
}

impl fmt::Display for Function {
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
