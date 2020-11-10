//!
//! The structure semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_syntax::StructureExpression;

use crate::generator::expression::operand::group::builder::Builder as GeneratorGroupExpressionBuilder;
use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::element::constant::structure::Structure as StructureConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::structure::Structure as StructureValue;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;

///
/// The structure semantic analyzer.
///
pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes the structure literal expression.
    ///
    /// Returns the semantic element and the intermediate representation.
    ///
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        structure: StructureExpression,
        rule: TranslationRule,
    ) -> Result<(Element, Option<GeneratorExpressionOperand>), Error> {
        match rule {
            TranslationRule::Constant => {
                Self::constant(scope, structure).map(|element| (element, None))
            }
            _rule => Self::runtime(scope, structure)
                .map(|(element, intermediate)| (element, Some(intermediate))),
        }
    }

    ///
    /// Returns the runtime structure value semantic element and intermediate representation.
    ///
    fn runtime(
        scope: Rc<RefCell<Scope>>,
        structure: StructureExpression,
    ) -> Result<(Element, GeneratorExpressionOperand), Error> {
        let location = structure.location;

        let mut builder = GeneratorGroupExpressionBuilder::default();

        let mut result = StructureValue::new(Some(location));

        for (identifier, expression) in structure.fields.into_iter() {
            let (element, expression) =
                ExpressionAnalyzer::new(scope.clone(), TranslationRule::Value)
                    .analyze(expression)?;
            let element_type = Type::from_element(&element, scope.clone())?;

            result.push(
                identifier.name,
                Some(identifier.location),
                element_type.clone(),
            );

            builder.push_expression(element_type, expression);
        }

        let element = Element::Value(Value::Structure(result));
        let intermediate = GeneratorExpressionOperand::Group(builder.finish());

        Ok((element, intermediate))
    }

    ///
    /// Returns the constant structure semantic element.
    ///
    fn constant(
        scope: Rc<RefCell<Scope>>,
        structure: StructureExpression,
    ) -> Result<Element, Error> {
        let mut result = StructureConstant::new(structure.location);

        for (identifier, expression) in structure.fields.into_iter() {
            let expression_location = expression.location;

            let (element, _) = ExpressionAnalyzer::new(scope.clone(), TranslationRule::Constant)
                .analyze(expression)?;

            match element {
                Element::Constant(constant) => {
                    result.push(identifier, constant);
                }
                element => {
                    return Err(Error::ExpressionNonConstantElement {
                        location: expression_location,
                        found: element.to_string(),
                    });
                }
            }
        }

        let element = Element::Constant(Constant::Structure(result));

        Ok(element)
    }
}
