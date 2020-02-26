//!
//! The semantic analyzer user-defined function type element.
//!

use std::fmt;

use crate::semantic::element::r#type::function::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;

#[derive(Debug, Clone)]
pub struct Function {
    pub identifier: String,
    pub unique_id: usize,
    pub formal_params: Vec<(String, Type)>,
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
            formal_params: arguments,
            return_type: Box::new(return_type),
            unique_id,
        }
    }

    pub fn call(self, actual_elements: Vec<Element>) -> Result<Type, Error> {
        let mut actual_params = Vec::with_capacity(actual_elements.len());
        for (index, element) in actual_elements.into_iter().enumerate() {
            let (r#type, is_constant) = match element {
                Element::Value(value) => (value.r#type(), false),
                Element::Constant(constant) => (constant.r#type(), true),
                element => {
                    return Err(Error::ArgumentNotEvaluable(
                        self.identifier.to_owned(),
                        index + 1,
                        element.to_string(),
                    ))
                }
            };
            actual_params.push((r#type, is_constant));
        }

        let formal_params_length = self.formal_params.len();
        for (index, (name, r#type)) in self.formal_params.into_iter().enumerate() {
            match actual_params.get(index) {
                Some((actual_type, _is_constant)) if actual_type == &r#type => {}
                Some((actual_type, _is_constant)) => {
                    return Err(Error::ArgumentType(
                        self.identifier.to_owned(),
                        r#type.to_string(),
                        index + 1,
                        name,
                        actual_type.to_string(),
                    ))
                }
                None => {
                    return Err(Error::ArgumentCount(
                        self.identifier.to_owned(),
                        formal_params_length,
                        actual_params.len(),
                    ))
                }
            }
        }

        Ok(*self.return_type)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "fn {}({}) -> {}",
            self.identifier,
            self.formal_params
                .iter()
                .map(|(name, r#type)| format!("{}: {}", name, r#type))
                .collect::<Vec<String>>()
                .join(", "),
            self.return_type,
        )
    }
}
