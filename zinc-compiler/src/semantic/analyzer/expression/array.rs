//!
//! The array semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::expression::operand::array::builder::Builder as GeneratorArrayExpressionBuilder;
use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::semantic::analyzer::expression::hint::Hint as TranslationHint;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::array::Array;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::syntax::tree::expression::array::variant::Variant as ArrayExpressionVariant;
use crate::syntax::tree::expression::array::Expression as ArrayExpression;

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
    ) -> Result<(Element, GeneratorExpressionOperand), Error> {
        let mut result = Array::default();
        let mut builder = GeneratorArrayExpressionBuilder::default();

        match array.variant {
            ArrayExpressionVariant::List { elements } => {
                for expression in elements.into_iter() {
                    let expression_location = expression.location;

                    let (element, expression) = ExpressionAnalyzer::new(scope.clone())
                        .analyze(expression, TranslationHint::Value)?;
                    let element_type = Type::from_element(&element, scope.clone())?;
                    result.push(element_type).map_err(|error| {
                        Error::Element(
                            expression_location,
                            ElementError::Value(ValueError::Array(error)),
                        )
                    })?;

                    builder.push_expression(expression);
                }
            }
            ArrayExpressionVariant::Repeated {
                expression,
                size_expression,
            } => {
                let expression_location = expression.location;
                let size_expression_location = size_expression.location;

                let size = match ExpressionAnalyzer::new(scope.clone())
                    .analyze(size_expression, TranslationHint::Value)?
                {
                    (Element::Constant(Constant::Integer(integer)), _intermediate) => {
                        integer.to_usize().map_err(|error| {
                            Error::Element(
                                size_expression_location,
                                ElementError::Constant(ConstantError::Integer(error)),
                            )
                        })?
                    }
                    (element, _intermediate) => {
                        return Err(Error::ConstantExpressionHasNonConstantElement {
                            location: size_expression_location,
                            found: element.to_string(),
                        });
                    }
                };

                let (element, expression) = ExpressionAnalyzer::new(scope.clone())
                    .analyze(expression, TranslationHint::Value)?;
                let element_type = Type::from_element(&element, scope)?;
                result.extend(element_type, size).map_err(|error| {
                    Error::Element(
                        expression_location,
                        ElementError::Value(ValueError::Array(error)),
                    )
                })?;

                builder.push_expression(expression);
                builder.set_size(size);
            }
        }

        let intermediate = GeneratorExpressionOperand::Array(builder.finish());
        let element = Element::Value(Value::Array(result));

        Ok((element, intermediate))
    }
}
