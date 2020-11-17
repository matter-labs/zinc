//!
//! The block expression semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_syntax::BlockExpression;
use zinc_syntax::FunctionLocalStatement;

use crate::generator::expression::operand::block::builder::Builder as GeneratorBlockExpressionBuilder;
use crate::generator::expression::operand::block::Expression as GeneratorBlockExpression;
use crate::generator::statement::Statement as GeneratorStatement;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::analyzer::statement::r#const::Analyzer as ConstStatementAnalyzer;
use crate::semantic::analyzer::statement::r#for::Analyzer as ForStatementAnalyzer;
use crate::semantic::analyzer::statement::r#let::Analyzer as LetStatementAnalyzer;
use crate::semantic::element::value::unit::Unit as UnitValue;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::r#type::Type as ScopeType;
use crate::semantic::scope::stack::Stack as ScopeStack;
use crate::semantic::scope::Scope;

///
/// The block expression semantic analyzer.
///
pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes the block expression.
    ///
    /// Returns the semantic element and the intermediate representation.
    ///
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        block: BlockExpression,
        rule: TranslationRule,
    ) -> Result<(Element, GeneratorBlockExpression), Error> {
        let mut builder = GeneratorBlockExpressionBuilder::default();

        let mut scope_stack = ScopeStack::new(scope);
        scope_stack.push(None, ScopeType::Block);

        for statement in block.statements.into_iter() {
            let intermediate = match statement {
                FunctionLocalStatement::Let(statement) => {
                    LetStatementAnalyzer::define(scope_stack.top(), statement)?
                        .map(GeneratorStatement::Let)
                }
                FunctionLocalStatement::Const(statement) => {
                    let identifier = statement.identifier.clone();
                    let constant = ConstStatementAnalyzer::define(scope_stack.top(), statement)?;
                    Scope::define_constant(scope_stack.top(), identifier, constant)?;
                    None
                }
                FunctionLocalStatement::For(statement) => Some(GeneratorStatement::For(
                    ForStatementAnalyzer::define(scope_stack.top(), statement)?,
                )),
                FunctionLocalStatement::Expression(expression) => {
                    let (_result, expression) =
                        ExpressionAnalyzer::new(scope_stack.top(), rule).analyze(expression)?;
                    let intermediate = GeneratorStatement::Expression(expression);
                    Some(intermediate)
                }
                FunctionLocalStatement::Empty(_location) => None,
            };

            if let Some(intermediate) = intermediate {
                builder.push_statement(intermediate);
            }
        }

        let element = match block.expression {
            Some(expression) => {
                let (element, expression) =
                    ExpressionAnalyzer::new(scope_stack.top(), rule).analyze(*expression)?;
                builder.set_expression(expression);
                element
            }
            None => Element::Value(Value::Unit(UnitValue::new(Some(block.location)))),
        };

        scope_stack.pop();

        Ok((element, builder.finish()))
    }
}
