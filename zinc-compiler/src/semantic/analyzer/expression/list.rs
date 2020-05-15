//!
//! The list semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::expression::operand::list::builder::Builder as GeneratorListExpressionBuilder;
use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::element::argument_list::ArgumentList as ArgumentListElement;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::syntax::tree::expression::list::Expression as ListExpression;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes the function argument list.
    ///
    /// Returns the semantic element and the intermediate representation.
    ///
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        list: ListExpression,
        rule: TranslationRule,
    ) -> Result<(Element, GeneratorExpressionOperand), Error> {
        let location = list.location;

        let mut arguments = Vec::with_capacity(list.len());
        let mut builder = GeneratorListExpressionBuilder::default();

        for expression in list.elements.into_iter() {
            let (element, expression) =
                ExpressionAnalyzer::new(scope.clone(), rule).analyze(expression)?;
            arguments.push(element);

            builder.push_expression(expression);
        }

        let element = Element::ArgumentList(ArgumentListElement::new(location, arguments));
        let intermediate = GeneratorExpressionOperand::List(builder.finish());

        Ok((element, intermediate))
    }
}
