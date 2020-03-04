//!
//! The semantic analyzer user-defined function element.
//!

use std::fmt;
use std::ops::Deref;

use crate::semantic::element::r#type::function::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;

#[derive(Debug, Clone)]
pub struct Function {
    identifier: String,
    unique_id: usize,
    formal_params: Vec<(String, Type)>,
    return_type: Box<Type>,
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

    pub fn identifier(&self) -> &str {
        self.identifier.as_str()
    }

    pub fn unique_id(&self) -> usize {
        self.unique_id
    }

    pub fn formal_params(&self) -> &[(String, Type)] {
        self.formal_params.as_slice()
    }

    pub fn return_type(&self) -> &Type {
        self.return_type.deref()
    }

    pub fn input_size(&self) -> usize {
        self.formal_params
            .iter()
            .map(|(_name, r#type)| r#type.size())
            .sum()
    }

    pub fn output_size(&self) -> usize {
        self.return_type.size()
    }

    pub fn call(self, actual_elements: Vec<Element>) -> Result<Type, Error> {
        let mut actual_params = Vec::with_capacity(actual_elements.len());
        for (index, element) in actual_elements.into_iter().enumerate() {
            let (r#type, is_constant) = match element {
                Element::Value(value) => (value.r#type(), false),
                Element::Constant(constant) => (constant.r#type(), true),
                element => {
                    return Err(Error::argument_not_evaluable(
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
                    return Err(Error::argument_type(
                        self.identifier.to_owned(),
                        name,
                        index + 1,
                        r#type.to_string(),
                        actual_type.to_string(),
                    ))
                }
                None => {
                    return Err(Error::argument_count(
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
