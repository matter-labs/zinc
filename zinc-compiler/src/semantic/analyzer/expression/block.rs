//!
//! The block expression semantic analyzer.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::generator::expression::operand::block::builder::Builder as GeneratorBlockExpressionBuilder;
use crate::generator::expression::operand::block::Expression as GeneratorBlockExpression;
use crate::semantic::analyzer::expression::hint::Hint as TranslationHint;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::analyzer::statement::Analyzer as StatementAnalyzer;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::stack::Stack as ScopeStack;
use crate::semantic::scope::Scope;
use crate::syntax::BlockExpression;

pub struct Analyzer {}

impl Analyzer {
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        block: BlockExpression,
    ) -> Result<(Element, GeneratorBlockExpression), Error> {
        let mut builder = GeneratorBlockExpressionBuilder::default();

        let mut scope_stack = ScopeStack::new(scope);
        scope_stack.push();

        for statement in block.statements.into_iter() {
            if let Some(statement) =
                StatementAnalyzer::new(scope_stack.top(), HashMap::new()).local_fn(statement)?
            {
                builder.push_statement(statement);
            }
        }

        let element = match block.expression {
            Some(expression) => {
                let (element, expression) = ExpressionAnalyzer::new(scope_stack.top())
                    .analyze(*expression, TranslationHint::ValueExpression)?;
                builder.set_expression(expression);
                element
            }
            None => Element::Value(Value::Unit),
        };

        scope_stack.pop();

        Ok((element, builder.finish()))
    }
}
