//!
//! The semantic analyzer user-defined function element.
//!

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::function::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;

#[derive(Debug, Clone)]
pub struct Function {
    pub location: Location,
    pub identifier: String,
    pub unique_id: usize,
    pub formal_params: Vec<(String, Type)>,
    pub return_type: Box<Type>,
}

impl Function {
    pub fn new(
        location: Location,
        identifier: String,
        unique_id: usize,
        arguments: Vec<(String, Type)>,
        return_type: Type,
    ) -> Self {
        Self {
            location,
            identifier,
            formal_params: arguments,
            return_type: Box::new(return_type),
            unique_id,
        }
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
            let location = element.location();

            let (r#type, is_constant) = match element {
                Element::Value(value) => (value.r#type(), false),
                Element::Constant(constant) => (constant.r#type(), true),
                element => {
                    return Err(Error::ArgumentNotEvaluable {
                        location: location.unwrap(),
                        function: self.identifier.to_owned(),
                        position: index + 1,
                        found: element.to_string(),
                    })
                }
            };

            actual_params.push((r#type, is_constant, location));
        }

        if actual_params.len() != self.formal_params.len() {
            return Err(Error::ArgumentCount {
                location: self.location,
                function: self.identifier.to_owned(),
                expected: self.formal_params.len(),
                found: actual_params.len(),
            });
        }

        let formal_params_length = self.formal_params.len();
        for (index, (name, r#type)) in self.formal_params.into_iter().enumerate() {
            match actual_params.get(index) {
                Some((actual_type, _is_constant, _location)) if actual_type == &r#type => {}
                Some((actual_type, _is_constant, location)) => {
                    return Err(Error::ArgumentType {
                        location: location.unwrap(),
                        function: self.identifier.to_owned(),
                        name,
                        position: index + 1,
                        expected: r#type.to_string(),
                        found: actual_type.to_string(),
                    })
                }
                None => {
                    return Err(Error::ArgumentCount {
                        location: self.location,
                        function: self.identifier.to_owned(),
                        expected: formal_params_length,
                        found: actual_params.len(),
                    })
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
