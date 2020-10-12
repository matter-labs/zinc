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
use crate::semantic::analyzer::expression::error::Error as ExpressionError;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::element::argument_list::ArgumentList;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::function::error::Error;
use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error as SemanticError;
use crate::semantic::scope::item::constant::Constant as ScopeConstantItem;
use crate::semantic::scope::item::Item as ScopeItem;
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
    pub formal_params: Vec<(String, Type)>,
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
        arguments: Vec<(String, Type)>,
        return_type: Type,
        body: BlockExpression,
    ) -> Self {
        Self {
            location,
            identifier,
            formal_params: arguments,
            return_type: Box::new(return_type),
            type_id,
            body,
        }
    }

    ///
    /// The function input arguments total size in the virtual compiler data stack.
    ///
    pub fn input_size(&self) -> usize {
        self.formal_params
            .iter()
            .map(|(_name, r#type)| r#type.size())
            .sum()
    }

    ///
    /// The function result type size in the virtual compiler data stack.
    ///
    pub fn output_size(&self) -> usize {
        self.return_type.size()
    }

    ///
    /// Validates the function call with the `argument_list`.
    ///
    pub fn validate(&self, argument_list: ArgumentList) -> Result<Vec<(String, Constant)>, Error> {
        if argument_list.arguments.len() != self.formal_params.len() {
            return Err(Error::ArgumentCount {
                location: self.location,
                function: self.identifier.to_owned(),
                expected: self.formal_params.len(),
                found: argument_list.arguments.len(),
                reference: Some(argument_list.location),
            });
        }

        let mut actual_params = Vec::with_capacity(argument_list.arguments.len());
        for (index, element) in argument_list.arguments.into_iter().enumerate() {
            let name = self.formal_params[index].0.to_owned();

            let constant = match element {
                Element::Constant(constant) => constant,
                Element::Value(value) => {
                    return Err(Error::ArgumentConstantness {
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
                    return Err(Error::ArgumentNotEvaluable {
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

        let formal_params_length = self.formal_params.len();
        for (index, (name, r#type)) in self.formal_params.iter().enumerate() {
            match actual_params.get(index) {
                Some((_name, constant)) if &constant.r#type() == r#type => {}
                Some((_name, constant)) => {
                    return Err(Error::ArgumentType {
                        location: constant.location(),
                        function: self.identifier.to_owned(),
                        name: name.to_owned(),
                        position: index + 1,
                        expected: r#type.to_string(),
                        found: constant.r#type().to_string(),
                    })
                }
                None => {
                    return Err(Error::ArgumentCount {
                        location: self.location,
                        function: self.identifier.to_owned(),
                        expected: formal_params_length,
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
    ) -> Result<Constant, SemanticError> {
        let location = self.location;

        let scope = Scope::new_child(self.identifier, scope);

        for (name, constant) in arguments.into_iter() {
            Scope::insert_item(
                scope.clone(),
                name,
                ScopeItem::Constant(ScopeConstantItem::new_defined(
                    constant.location(),
                    constant,
                    false,
                ))
                .wrap(),
            );
        }

        let (element, _intermediate) =
            BlockExpressionAnalyzer::analyze(scope, self.body, TranslationRule::Constant)?;
        match element {
            Element::Constant(constant) => Ok(constant),
            element => Err(SemanticError::Expression(
                ExpressionError::NonConstantElement {
                    location: element.location().unwrap_or(location),
                    found: element.to_string(),
                },
            )),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "const fn {}({}) -> {}",
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
