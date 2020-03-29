//!
//! The list semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::expression::operand::list::builder::Builder as GeneratorListExpressionBuilder;
use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::semantic::analyzer::expression::hint::Hint as TranslationHint;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::syntax::ExpressionTree;

pub struct Analyzer {}

impl Analyzer {
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        list: Vec<ExpressionTree>,
    ) -> Result<(Element, GeneratorExpressionOperand), Error> {
        let mut expressions = Vec::with_capacity(list.len());
        let mut builder = GeneratorListExpressionBuilder::default();

        for expression in list.into_iter() {
            let (element, expression) = ExpressionAnalyzer::new(scope.clone())
                .analyze(expression, TranslationHint::ValueExpression)?;
            expressions.push(element);

            builder.push_expression(expression);
        }

        let element = Element::ArgumentList(expressions);
        let intermediate = GeneratorExpressionOperand::List(builder.finish());

        Ok((element, intermediate))
    }
}
