//!
//! The semantic analyzer constant function element.
//!

#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use zinc_lexical::Location;
use zinc_syntax::BlockExpression;

use crate::semantic::analyzer::expression::block::Analyzer as BlockExpressionAnalyzer;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::binding::Binding;
use crate::semantic::element::argument_list::ArgumentList;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::item::constant::Constant as ScopeConstantItem;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::r#type::Type as ScopeType;
use crate::semantic::scope::Scope;

///
/// The semantic analyzer constant function element.
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
    /// The function body, which is executed each time the function is called.
    pub body: BlockExpression,
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
        body: BlockExpression,
    ) -> Self {
        Self {
            location,
            identifier,
            bindings,
            return_type: Box::new(return_type),
            type_id,
            body,
        }
    }

    ///
    /// The function input arguments total size in the virtual compiler data stack.
    ///
    pub fn input_size(&self) -> usize {
        self.bindings
            .iter()
            .map(|binding| binding.r#type.size())
            .sum()
    }

    ///
    /// The function result type size in the virtual compiler data stack.
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
    /// Validates the function call with the `argument_list`.
    ///
    pub fn validate(&self, argument_list: ArgumentList) -> Result<Vec<(String, Constant)>, Error> {
        if argument_list.arguments.len() != self.bindings.len() {
            return Err(Error::FunctionArgumentCount {
                location: self.location,
                function: self.identifier.to_owned(),
                expected: self.bindings.len(),
                found: argument_list.arguments.len(),
                reference: Some(argument_list.location),
            });
        }

        let mut actual_params = Vec::with_capacity(argument_list.arguments.len());
        for (index, element) in argument_list.arguments.into_iter().enumerate() {
            let name = self.bindings[index].identifier.name.to_owned();

            let constant = match element {
                Element::Constant(constant) => constant,
                Element::Value(value) => {
                    return Err(Error::FunctionArgumentConstantness {
                        location: value
                            .location()
                            .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                        function: self.identifier.to_owned(),
                        name,
                        position: index + 1,
                        found: value.to_string(),
                    })
                }
                element => {
                    return Err(Error::FunctionArgumentNotEvaluable {
                        location: element
                            .location()
                            .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                        function: self.identifier.to_owned(),
                        position: index + 1,
                        found: element.to_string(),
                    })
                }
            };

            actual_params.push((name, constant));
        }

        let bindings_length = self.bindings.len();
        for (index, binding) in self.bindings.iter().enumerate() {
            match actual_params.get(index) {
                Some((_name, constant)) if constant.r#type() == binding.r#type => {}
                Some((_name, constant)) => {
                    return Err(Error::FunctionArgumentType {
                        location: constant.location(),
                        function: self.identifier.to_owned(),
                        name: binding.identifier.name.to_owned(),
                        position: index + 1,
                        expected: binding.r#type.to_string(),
                        found: constant.r#type().to_string(),
                    })
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

        Ok(actual_params)
    }

    ///
    /// Calls the constant function with a specific set of constant `arguments`, which are
    /// declared in their own `scope`, and then executes the function body in a constant context,
    /// where the result is calculated and checked for possible violations, like integer overflow.
    ///
    pub fn call(
        self,
        arguments: Vec<(String, Constant)>,
        scope: Rc<RefCell<Scope>>,
    ) -> Result<Constant, Error> {
        let location = self.location;

        let scope = Scope::new_child(self.identifier, ScopeType::Function, scope);

        for (name, constant) in arguments.into_iter() {
            Scope::insert_item(
                scope.clone(),
                name,
                ScopeItem::Constant(ScopeConstantItem::new_defined(
                    constant.location(),
                    constant,
                ))
                .wrap(),
            );
        }

        let (element, _intermediate) =
            BlockExpressionAnalyzer::analyze(scope, self.body, TranslationRule::Constant)?;
        match element {
            Element::Constant(constant) => Ok(constant),
            element => Err(Error::ExpressionNonConstantElement {
                location: element.location().unwrap_or(location),
                found: element.to_string(),
            }),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "const fn {}({}) -> {}",
            self.identifier,
            self.bindings
                .iter()
                .map(|binding| format!("{}: {}", binding.identifier.name, binding.r#type))
                .collect::<Vec<String>>()
                .join(", "),
            self.return_type,
        )
    }
}
