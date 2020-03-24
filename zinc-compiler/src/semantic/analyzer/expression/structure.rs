//!
//! The structure semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::expression::operand::group::builder::Builder as GeneratorGroupExpressionBuilder;
use crate::generator::expression::operand::group::Expression as GeneratorGroupExpression;
use crate::semantic::analyzer::expression::hint::Hint as TranslationHint;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::element::value::structure::Structure;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::item::Variant as ScopeItemVariant;
use crate::semantic::scope::Scope;
use crate::syntax::StructureExpression;

pub struct Analyzer {}

impl Analyzer {
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        structure: StructureExpression,
    ) -> Result<(Element, GeneratorGroupExpression), Error> {
        let identifier_location = structure.identifier.location;

        let mut builder = GeneratorGroupExpressionBuilder::default();

        let structure_type = match Scope::resolve_item(scope.clone(), &structure.identifier.name)
            .map_err(|error| Error::Scope(identifier_location, error))?
            .variant
        {
            ScopeItemVariant::Type(Type::Structure(structure)) => structure,
            item => {
                return Err(Error::TypeAliasDoesNotPointToStructure {
                    location: identifier_location,
                    found: item.to_string(),
                });
            }
        };
        let mut result = Structure::new(structure_type);

        for (identifier, expression) in structure.fields.into_iter() {
            let identifier_location = identifier.location;

            let (element, expression) = ExpressionAnalyzer::new(scope.clone())
                .analyze(expression, TranslationHint::ValueExpression)?;
            let element_type = Type::from_element(&element, scope.clone())?;
            result
                .push(identifier.name.clone(), element_type)
                .map_err(|error| {
                    Error::Element(
                        identifier_location,
                        ElementError::Value(ValueError::Structure(error)),
                    )
                })?;

            builder.push_expression(expression);
        }

        let result = Element::Value(Value::Structure(result));

        Ok((result, builder.finish()))
    }
}
