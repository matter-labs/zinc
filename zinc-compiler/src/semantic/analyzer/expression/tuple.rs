//!
//! The tuple semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_syntax::TupleExpression;

use crate::generator::expression::operand::group::builder::Builder as GeneratorGroupExpressionBuilder;
use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::element::constant::tuple::Tuple as TupleConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::tuple::Tuple as TupleValue;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;

///
/// The tuple semantic analyzer.
///
pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes the tuple literal expression.
    ///
    /// Returns the semantic element and the intermediate representation.
    ///
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        tuple: TupleExpression,
        rule: TranslationRule,
    ) -> Result<(Element, Option<GeneratorExpressionOperand>), Error> {
        match rule {
            TranslationRule::Constant => {
                Self::constant(scope, tuple).map(|element| (element, None))
            }
            _rule => Self::runtime(scope, tuple)
                .map(|(element, intermediate)| (element, Some(intermediate))),
        }
    }

    ///
    /// Returns the runtime tuple value semantic element and intermediate representation.
    ///
    fn runtime(
        scope: Rc<RefCell<Scope>>,
        tuple: TupleExpression,
    ) -> Result<(Element, GeneratorExpressionOperand), Error> {
        let mut result = TupleValue::new(Some(tuple.location));

        let mut builder = GeneratorGroupExpressionBuilder::default();

        for expression in tuple.elements.into_iter() {
            let (element, expression) =
                ExpressionAnalyzer::new(scope.clone(), TranslationRule::Value)
                    .analyze(expression)?;
            let element_type = Type::from_element(&element, scope.clone())?;

            result.push(element_type.clone());

            builder.push_expression(element_type, expression);
        }

        let element = Element::Value(Value::Tuple(result));
        let intermediate = GeneratorExpressionOperand::Group(builder.finish());

        Ok((element, intermediate))
    }

    ///
    /// Returns the constant tuple semantic element.
    ///
    fn constant(scope: Rc<RefCell<Scope>>, tuple: TupleExpression) -> Result<Element, Error> {
        let mut result = TupleConstant::with_capacity(tuple.location, tuple.elements.len());

        for expression in tuple.elements.into_iter() {
            let expression_location = expression.location;

            let (element, _) = ExpressionAnalyzer::new(scope.clone(), TranslationRule::Constant)
                .analyze(expression)?;

            match element {
                Element::Constant(constant) => result.push(constant),
                element => {
                    return Err(Error::ExpressionNonConstantElement {
                        location: expression_location,
                        found: element.to_string(),
                    });
                }
            }
        }

        let element = Element::Constant(Constant::Tuple(result));

        Ok(element)
    }
}
