//!
//! The list semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_syntax::ListExpression;

use crate::generator::expression::operand::list::builder::Builder as GeneratorListExpressionBuilder;
use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::element::argument_list::ArgumentList as ArgumentListElement;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;

///
/// The list semantic analyzer.
///
pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes the function argument list expression.
    ///
    /// Returns the semantic element and the intermediate representation.
    ///
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        list: ListExpression,
        rule: TranslationRule,
    ) -> Result<(Element, Option<GeneratorExpressionOperand>), Error> {
        match rule {
            TranslationRule::Constant => Self::constant(scope, list).map(|element| (element, None)),
            _rule => Self::runtime(scope, list)
                .map(|(element, intermediate)| (element, Some(intermediate))),
        }
    }

    ///
    /// Returns the runtime function argument list semantic element and intermediate representation.
    ///
    fn runtime(
        scope: Rc<RefCell<Scope>>,
        list: ListExpression,
    ) -> Result<(Element, GeneratorExpressionOperand), Error> {
        let location = list.location;

        let mut arguments = Vec::with_capacity(list.len());
        let mut builder = GeneratorListExpressionBuilder::default();

        for expression in list.elements.into_iter() {
            let (element, expression) =
                ExpressionAnalyzer::new(scope.clone(), TranslationRule::Value)
                    .analyze(expression)?;
            arguments.push(element);

            builder.push_expression(expression);
        }

        let element = Element::ArgumentList(ArgumentListElement::new(location, arguments));
        let intermediate = GeneratorExpressionOperand::List(builder.finish());

        Ok((element, intermediate))
    }

    ///
    /// Returns the constant function argument list semantic element.
    ///
    fn constant(scope: Rc<RefCell<Scope>>, list: ListExpression) -> Result<Element, Error> {
        let location = list.location;

        let mut arguments = Vec::with_capacity(list.len());

        for expression in list.elements.into_iter() {
            let (element, _intermediate) =
                ExpressionAnalyzer::new(scope.clone(), TranslationRule::Constant)
                    .analyze(expression)?;
            arguments.push(element);
        }

        let element = Element::ArgumentList(ArgumentListElement::new(location, arguments));

        Ok(element)
    }
}
