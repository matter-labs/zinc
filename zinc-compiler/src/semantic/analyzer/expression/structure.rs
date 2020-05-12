//!
//! The structure semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::expression::operand::group::builder::Builder as GeneratorGroupExpressionBuilder;
use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::semantic::analyzer::expression::error::Error as ExpressionError;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::structure::Structure as StructureConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::structure::Structure as StructureType;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::element::value::structure::Structure as StructureValue;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::Scope;
use crate::syntax::tree::expression::structure::Expression as StructureExpression;

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
        let identifier_location = structure.identifier.location;

        let r#type = match Scope::resolve_item(scope.clone(), &structure.identifier, true)? {
            ScopeItem::Type(r#type) => {
                let r#type = r#type.resolve()?;
                match r#type {
                    Type::Structure(mut structure) => {
                        structure.location = Some(identifier_location);
                        structure
                    }
                    _type => {
                        return Err(Error::Element(ElementError::Type(
                            TypeError::AliasDoesNotPointToStructure {
                                location: identifier_location,
                                found: structure.identifier.name,
                            },
                        )))
                    }
                }
            }
            _item => {
                return Err(Error::Element(ElementError::Type(
                    TypeError::AliasDoesNotPointToStructure {
                        location: identifier_location,
                        found: structure.identifier.name,
                    },
                )));
            }
        };

        match rule {
            TranslationRule::Constant => {
                Self::constant(scope, structure, r#type).map(|element| (element, None))
            }
            _rule => Self::value(scope, structure, r#type)
                .map(|(element, intermediate)| (element, Some(intermediate))),
        }
    }

    ///
    /// Returns the runtime structure value semantic element and intermediate representation.
    ///
    fn value(
        scope: Rc<RefCell<Scope>>,
        structure: StructureExpression,
        r#type: StructureType,
    ) -> Result<(Element, GeneratorExpressionOperand), Error> {
        let location = structure.location;

        let mut builder = GeneratorGroupExpressionBuilder::default();

        let mut result = StructureValue::new(Some(location), r#type);

        for (identifier, expression) in structure.fields.into_iter() {
            let (element, expression) =
                ExpressionAnalyzer::new(scope.clone(), TranslationRule::Value)
                    .analyze(expression)?;
            let element_type = Type::from_element(&element, scope.clone())?;

            result
                .push(identifier, element_type.clone(), element.location())
                .map_err(|error| {
                    Error::Element(ElementError::Value(ValueError::Structure(error)))
                })?;

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
        r#type: StructureType,
    ) -> Result<Element, Error> {
        let mut result = StructureConstant::new(structure.location, r#type);

        for (identifier, expression) in structure.fields.into_iter() {
            let expression_location = expression.location;

            let (element, _) = ExpressionAnalyzer::new(scope.clone(), TranslationRule::Constant)
                .analyze(expression)?;

            match element {
                Element::Constant(constant) => {
                    result.push(identifier, constant).map_err(|error| {
                        Error::Element(ElementError::Constant(ConstantError::Structure(error)))
                    })?
                }
                element => {
                    return Err(Error::Expression(ExpressionError::NonConstantElement {
                        location: expression_location,
                        found: element.to_string(),
                    }))
                }
            }
        }

        let element = Element::Constant(Constant::Structure(result));

        Ok(element)
    }
}
