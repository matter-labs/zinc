//!
//! The structure semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::expression::operand::group::builder::Builder as GeneratorGroupExpressionBuilder;
use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::semantic::analyzer::expression::hint::Hint as TranslationHint;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::element::value::structure::Structure;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::item::variant::Variant as ScopeItemVariant;
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
    ) -> Result<(Element, GeneratorExpressionOperand), Error> {
        let identifier_location = structure.identifier.location;

        let mut builder = GeneratorGroupExpressionBuilder::default();

        let structure_type_item = Scope::resolve_item(scope.clone(), &structure.identifier, true)?;
        let structure_type = match structure_type_item.variant {
            ScopeItemVariant::Type(Type::Structure(structure)) => structure,
            item => {
                return Err(Error::Element(
                    identifier_location,
                    ElementError::Type(TypeError::AliasDoesNotPointToStructure {
                        found: item.to_string(),
                    }),
                ));
            }
        };
        let mut result = Structure::new(structure_type);

        for (identifier, expression) in structure.fields.into_iter() {
            let identifier_location = identifier.location;

            let (element, expression) = ExpressionAnalyzer::new(scope.clone())
                .analyze(expression, TranslationHint::Value)?;
            let element_type = Type::from_element(&element, scope.clone())?;
            result
                .push(identifier.name, element_type.clone())
                .map_err(|error| {
                    Error::Element(
                        identifier_location,
                        ElementError::Value(ValueError::Structure(error)),
                    )
                })?;

            builder.push_expression(element_type, expression);
        }

        let element = Element::Value(Value::Structure(result));
        let intermediate = GeneratorExpressionOperand::Group(builder.finish());

        Ok((element, intermediate))
    }
}
