//!
//! The semantic analyzer runtime function element.
//!

#[cfg(test)]
mod tests;

use std::fmt;

use zinc_lexical::Location;

use crate::semantic::binding::Binding;
use crate::semantic::element::argument_list::ArgumentList;
use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error;

///
/// The semantic analyzer runtime function element.
///
#[derive(Debug, Clone)]
pub struct Function {
    /// The location where the function is called.
    pub location: Location,
    /// The function identifier.
    pub identifier: String,
    /// The unique function type ID.
    pub type_id: usize,
    /// The function formal parameters list.
    pub bindings: Vec<Binding>,
    /// The function return type.
    pub return_type: Box<Type>,
}

impl Function {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        location: Location,
        identifier: String,
        type_id: usize,
        bindings: Vec<Binding>,
        return_type: Type,
    ) -> Self {
        Self {
            location,
            identifier,
            bindings,
            return_type: Box::new(return_type),
            type_id,
        }
    }

    ///
    /// The function input arguments total size in the abstract data stack.
    ///
    pub fn input_size(&self) -> usize {
        self.bindings
            .iter()
            .map(|binding| binding.r#type.size())
            .sum()
    }

    ///
    /// The function result type size in the abstract data stack.
    ///
    pub fn output_size(&self) -> usize {
        self.return_type.size()
    }

    ///
    /// Whether the function must be called from mutable context.
    ///
    pub fn is_mutable(&self) -> bool {
        self.bindings
            .first()
            .map(|instance| instance.is_mutable)
            .unwrap_or_default()
    }

    ///
    /// Calls the function with the `argument_list`, validating the call.
    ///
    pub fn call(self, argument_list: ArgumentList) -> Result<Type, Error> {
        let mut actual_params = Vec::with_capacity(argument_list.arguments.len());
        for (index, element) in argument_list.arguments.into_iter().enumerate() {
            let location = element.location();

            let r#type = match element {
                Element::Value(value) => value.r#type(),
                Element::Constant(constant) => constant.r#type(),
                element => {
                    return Err(Error::FunctionArgumentNotEvaluable {
                        location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                        function: self.identifier.to_owned(),
                        position: index + 1,
                        found: element.to_string(),
                    })
                }
            };

            actual_params.push((r#type, location));
        }

        if actual_params.len() != self.bindings.len() {
            return Err(Error::FunctionArgumentCount {
                location: self.location,
                function: self.identifier.to_owned(),
                expected: self.bindings.len(),
                found: actual_params.len(),
                reference: Some(argument_list.location),
            });
        }

        let bindings_length = self.bindings.len();
        for (index, binding) in self.bindings.into_iter().enumerate() {
            match actual_params.get(index) {
                Some((actual_type, _location)) if actual_type == &binding.r#type => {}
                Some((actual_type, location)) => {
                    return Err(Error::FunctionArgumentType {
                        location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                        function: self.identifier.to_owned(),
                        name: binding.identifier.name,
                        position: index + 1,
                        expected: binding.r#type.to_string(),
                        found: actual_type.to_string(),
                    });
                }
                None => {
                    return Err(Error::FunctionArgumentCount {
                        location: self.location,
                        function: self.identifier.to_owned(),
                        expected: bindings_length,
                        found: actual_params.len(),
                        reference: Some(argument_list.location),
                    })
                }
            }
        }

        Ok(*self.return_type)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "fn {}({}) -> {}",
            self.identifier,
            self.bindings
                .iter()
                .map(|binding| format!(
                    "{}{}: {}",
                    if binding.is_mutable { "mut " } else { "" },
                    binding.identifier.name,
                    binding.r#type
                ))
                .collect::<Vec<String>>()
                .join(", "),
            self.return_type,
        )
    }
}
