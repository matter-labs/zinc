//!
//! The tuple semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::expression::operand::tuple::builder::Builder as GeneratorTupleExpressionBuilder;
use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::semantic::analyzer::expression::hint::Hint as TranslationHint;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::tuple::Tuple;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::syntax::TupleExpression;

pub struct Analyzer {}

impl Analyzer {
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        tuple: TupleExpression,
    ) -> Result<(Element, GeneratorExpressionOperand), Error> {
        let mut result = Tuple::default();
        let mut builder = GeneratorTupleExpressionBuilder::default();

        for expression in tuple.elements.into_iter() {
            let (element, expression) = ExpressionAnalyzer::new(scope.clone())
                .analyze(expression, TranslationHint::ValueExpression)?;
            let element_type = Type::from_element(&element, scope.clone())?;
            result.push(element_type.clone());

            builder.push_expression(element_type, expression);
        }

        let element = Element::Value(Value::Tuple(result));
        let intermediate = GeneratorExpressionOperand::Tuple(builder.finish());

        Ok((element, intermediate))
    }
}
