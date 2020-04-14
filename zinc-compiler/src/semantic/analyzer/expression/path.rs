//!
//! The path expression translator.
//!

use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

use crate::generator::expression::operand::constant::Constant as GeneratorConstant;
use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::generator::r#type::Type as GeneratorType;
use crate::semantic::analyzer::expression::hint::Hint as TranslationHint;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::path::Path;
use crate::semantic::element::place::Place;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::item::variant::Variant as ScopeItemVariant;
use crate::semantic::scope::Scope;

pub struct Translator {}

impl Translator {
    ///
    /// Translates the path expression to a semantic expression type specified in `hint`.
    ///
    pub fn translate(
        scope: Rc<RefCell<Scope>>,
        path: Path,
        hint: TranslationHint,
    ) -> Result<(Element, Option<GeneratorExpressionOperand>), Error> {
        let location = path.location;

        let path_last_identifier = path.last().to_owned();

        match hint {
            TranslationHint::Place => match Scope::resolve_path(scope, &path)?.variant {
                ScopeItemVariant::Variable(variable) => Ok((
                    Element::Place(Place::new(
                        path_last_identifier,
                        variable.r#type,
                        variable.is_mutable,
                    )),
                    None,
                )),
                ScopeItemVariant::Constant(constant) => Ok((Element::Constant(constant), None)),
                ScopeItemVariant::Type(r#type) => Ok((Element::Type(r#type), None)),
                ScopeItemVariant::Module(_) => {
                    Ok((Element::Module(path_last_identifier.name), None))
                }
            },
            TranslationHint::Value => match Scope::resolve_path(scope, &path)?.variant {
                ScopeItemVariant::Variable(variable) => {
                    let value = Value::try_from(&variable.r#type)
                        .map_err(ElementError::Value)
                        .map_err(|error| Error::Element(location, error))?;
                    let r#type = value.r#type();
                    let intermediate = GeneratorType::try_from_semantic(&r#type)
                        .map(|_| {
                            Place::new(path_last_identifier, r#type, variable.is_mutable).into()
                        })
                        .map(GeneratorExpressionOperand::Place);
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
                ScopeItemVariant::Module(_) => {
                    Ok((Element::Module(path_last_identifier.name), None))
                }
            },

            TranslationHint::Type => match Scope::resolve_path(scope, &path)?.variant {
                ScopeItemVariant::Type(r#type) => Ok((Element::Type(r#type), None)),
                _ => Ok((Element::Path(path), None)),
            },
            TranslationHint::Path => Ok((Element::Path(path), None)),
            TranslationHint::Field => Ok((Element::Identifier(path_last_identifier), None)),
        }
    }
}
