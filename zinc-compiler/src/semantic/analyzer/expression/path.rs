//!
//! The path expression translator.
//!

use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

use crate::generator::expression::operand::constant::Constant as GeneratorConstant;
use crate::generator::expression::operand::variable::Variable as GeneratorVariable;
use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::semantic::analyzer::expression::hint::Hint as TranslationHint;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::path::Path;
use crate::semantic::element::place::Place;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::item::Variant as ScopeItemVariant;
use crate::semantic::scope::Scope;
use crate::syntax::MemberString;

pub struct Translator {}

impl Translator {
    pub fn translate(
        scope: Rc<RefCell<Scope>>,
        path: &Path,
        translation_hint: TranslationHint,
    ) -> Result<(Element, Option<GeneratorExpressionOperand>), Error> {
        let location = path.location;

        let path_last_element_name = path.last().to_owned().name;

        match translation_hint {
            TranslationHint::PlaceExpression => match Scope::resolve_path(scope, path)?.variant {
                ScopeItemVariant::Variable(variable) => Ok((
                    Element::Place(Place::new(
                        location,
                        path_last_element_name,
                        variable.r#type,
                        variable.is_mutable,
                    )),
                    None,
                )),
                ScopeItemVariant::Constant(constant) => Ok((Element::Constant(constant), None)),
                ScopeItemVariant::Type(r#type) => Ok((Element::Type(r#type), None)),
                ScopeItemVariant::Module(_) => Ok((Element::Module(path_last_element_name), None)),
            },
            TranslationHint::ValueExpression => match Scope::resolve_path(scope, path)?.variant {
                ScopeItemVariant::Variable(variable) => {
                    let value = Value::try_from(&variable.r#type)
                        .map_err(ElementError::Value)
                        .map_err(|error| Error::Element(location, error))?;
                    let intermediate =
                        GeneratorVariable::try_from_semantic(path_last_element_name, &value)
                            .map(GeneratorExpressionOperand::Variable);
                    let element = Element::Value(value);
                    Ok((element, intermediate))
                }
                ScopeItemVariant::Constant(constant) => {
                    let intermediate = GeneratorConstant::try_from_semantic(&constant)
                        .map(GeneratorExpressionOperand::Constant);
                    let element = Element::Constant(constant);
                    Ok((element, intermediate))
                }
                ScopeItemVariant::Type(r#type) => Ok((Element::Type(r#type), None)),
                ScopeItemVariant::Module(_) => Ok((Element::Module(path_last_element_name), None)),
            },

            TranslationHint::TypeExpression => match Scope::resolve_path(scope, path)?.variant {
                ScopeItemVariant::Type(r#type) => Ok((Element::Type(r#type), None)),
                _ => Ok((Element::Path(path.to_owned()), None)),
            },
            TranslationHint::PathExpression => Ok((Element::Path(path.to_owned()), None)),
            TranslationHint::CompoundTypeMember => Ok((
                Element::MemberString(MemberString::new(location, path_last_element_name)),
                None,
            )),
        }
    }
}
