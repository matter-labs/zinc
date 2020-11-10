//!
//! The array semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_syntax::ArrayExpression;
use zinc_syntax::ArrayExpressionVariant;

use crate::generator::expression::operand::array::builder::Builder as GeneratorArrayExpressionBuilder;
use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::element::constant::array::Array as ArrayConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::array::Array as ArrayValue;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;

///
/// The array semantic analyzer.
///
pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes the array literal expression.
    ///
    /// Returns the semantic element and the intermediate representation.
    ///
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        array: ArrayExpression,
        rule: TranslationRule,
    ) -> Result<(Element, Option<GeneratorExpressionOperand>), Error> {
        match rule {
            TranslationRule::Constant => {
                Self::constant(scope, array).map(|element| (element, None))
            }
            _rule => Self::runtime(scope, array)
                .map(|(element, intermediate)| (element, Some(intermediate))),
        }
    }

    ///
    /// Returns the runtime array value semantic element and intermediate representation.
    ///
    fn runtime(
        scope: Rc<RefCell<Scope>>,
        array: ArrayExpression,
    ) -> Result<(Element, GeneratorExpressionOperand), Error> {
        let location = array.location;

        let mut result = ArrayValue::new(Some(location));

        let mut builder = GeneratorArrayExpressionBuilder::default();

        match array.variant {
            ArrayExpressionVariant::List { elements } => {
                for expression in elements.into_iter() {
                    let (element, expression) =
                        ExpressionAnalyzer::new(scope.clone(), TranslationRule::Value)
                            .analyze(expression)?;
                    let element_type = Type::from_element(&element, scope.clone())?;
                    result.push(element_type, element.location())?;

                    builder.push_expression(expression);
                }
            }
            ArrayExpressionVariant::Repeated {
                expression,
                size_expression,
            } => {
                let size_expression_location = size_expression.location;

                let size = match ExpressionAnalyzer::new(scope.clone(), TranslationRule::Constant)
                    .analyze(size_expression)?
                {
                    (Element::Constant(Constant::Integer(integer)), _intermediate) => {
                        integer.to_usize()?
                    }
                    (element, _intermediate) => {
                        return Err(Error::ExpressionNonConstantElement {
                            location: size_expression_location,
                            found: element.to_string(),
                        });
                    }
                };

                let (element, expression) =
                    ExpressionAnalyzer::new(scope.clone(), TranslationRule::Value)
                        .analyze(expression)?;
                let element_type = Type::from_element(&element, scope)?;
                result.extend(element_type, size, element.location())?;

                builder.push_expression(expression);
                builder.set_size(size);
            }
        }

        let intermediate = GeneratorExpressionOperand::Array(builder.finish());
        let element = Element::Value(Value::Array(result));

        Ok((element, intermediate))
    }

    ///
    /// Returns the constant array semantic element.
    ///
    fn constant(scope: Rc<RefCell<Scope>>, array: ArrayExpression) -> Result<Element, Error> {
        let mut result = ArrayConstant::new(array.location);

        match array.variant {
            ArrayExpressionVariant::List { elements } => {
                for expression in elements.into_iter() {
                    let expression_location = expression.location;

                    let (element, _) =
                        ExpressionAnalyzer::new(scope.clone(), TranslationRule::Constant)
                            .analyze(expression)?;
                    match element {
                        Element::Constant(constant) => result.push(constant)?,
                        element => {
                            return Err(Error::ExpressionNonConstantElement {
                                location: expression_location,
                                found: element.to_string(),
                            });
                        }
                    }
                }
            }
            ArrayExpressionVariant::Repeated {
                expression,
                size_expression,
            } => {
                let expression_location = expression.location;
                let size_expression_location = size_expression.location;

                let size = match ExpressionAnalyzer::new(scope.clone(), TranslationRule::Constant)
                    .analyze(size_expression)?
                {
                    (Element::Constant(Constant::Integer(integer)), _intermediate) => {
                        integer.to_usize()?
                    }
                    (element, _intermediate) => {
                        return Err(Error::ExpressionNonConstantElement {
                            location: size_expression_location,
                            found: element.to_string(),
                        });
                    }
                };

                let (element, _) = ExpressionAnalyzer::new(scope, TranslationRule::Constant)
                    .analyze(expression)?;
                match element {
                    Element::Constant(constant) => result.extend(vec![constant; size])?,
                    element => {
                        return Err(Error::ExpressionNonConstantElement {
                            location: expression_location,
                            found: element.to_string(),
                        });
                    }
                }
            }
        }

        let element = Element::Constant(Constant::Array(result));

        Ok(element)
    }
}
