//!
//! The list semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::expression::operand::group::builder::Builder as GeneratorGroupExpressionBuilder;
use crate::generator::expression::operand::group::Expression as GeneratorGroupExpression;
use crate::semantic::analyzer::expression::hint::Hint as TranslationHint;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::syntax::Expression;

pub struct Analyzer {}

impl Analyzer {
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        list: Vec<Expression>,
    ) -> Result<(Element, GeneratorGroupExpression), Error> {
        let mut elements = Vec::with_capacity(list.len());
        let mut builder = GeneratorGroupExpressionBuilder::default();

        for expression in list.into_iter() {
            let (element, expression) = ExpressionAnalyzer::new(scope.clone())
                .analyze(expression, TranslationHint::ValueExpression)?;
            elements.push(element);

            builder.push_expression(expression);
        }

        let result = Element::ArgumentList(elements);

        Ok((result, builder.finish()))
    }
}
