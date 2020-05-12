//!
//! The path expression translator.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::expression::operand::constant::Constant as GeneratorConstant;
use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::generator::r#type::Type as GeneratorType;
use crate::semantic::analyzer::expression::error::Error as ExpressionError;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::path::Path;
use crate::semantic::element::place::Place;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::Scope;

pub struct Translator {}

impl Translator {
    ///
    /// Translates the path expression to a semantic expression type specified in `rule`.
    ///
    pub fn translate(
        scope: Rc<RefCell<Scope>>,
        path: Path,
        rule: TranslationRule,
    ) -> Result<(Element, Option<GeneratorExpressionOperand>), Error> {
        let location = path.location;
        let path_last_identifier = path.last().to_owned();

        match rule {
            TranslationRule::Place => match Scope::resolve_path(scope, &path)? {
                ScopeItem::Variable(variable) => Ok((
                    Element::Place(Place::new(
                        path_last_identifier,
                        variable.r#type,
                        variable.is_mutable,
                    )),
                    None,
                )),
                ScopeItem::Constant(constant) => {
                    let mut constant = constant.resolve()?;
                    constant.set_location(location);

                    let intermediate = GeneratorConstant::try_from_semantic(&constant);

                    Ok((
                        Element::Constant(constant),
                        intermediate.map(GeneratorExpressionOperand::Constant),
                    ))
                }
                ScopeItem::Type(r#type) => {
                    let mut r#type = r#type.resolve()?;
                    r#type.set_location(location);

                    Ok((Element::Type(r#type), None))
                }
                ScopeItem::Module(_) => Ok((Element::Module(path_last_identifier), None)),
            },
            TranslationRule::Value => match Scope::resolve_path(scope, &path)? {
                ScopeItem::Variable(variable) => {
                    let value = Value::try_from_type(&variable.r#type, Some(location))
                        .map_err(ElementError::Value)
                        .map_err(Error::Element)?;
                    let r#type = value.r#type();
                    let element = Element::Value(value);

                    let intermediate = GeneratorType::try_from_semantic(&r#type)
                        .map(|_| {
                            Place::new(path_last_identifier, r#type, variable.is_mutable).into()
                        })
                        .map(GeneratorExpressionOperand::Place);

                    Ok((element, intermediate))
                }
                ScopeItem::Constant(constant) => {
                    let mut constant = constant.resolve()?;
                    constant.set_location(location);

                    let intermediate = GeneratorConstant::try_from_semantic(&constant)
                        .map(GeneratorExpressionOperand::Constant);

                    let element = Element::Constant(constant);
                    Ok((element, intermediate))
                }
                ScopeItem::Type(r#type) => {
                    let mut r#type = r#type.resolve()?;
                    r#type.set_location(location);

                    Ok((Element::Type(r#type), None))
                }
                ScopeItem::Module(_) => Ok((Element::Module(path_last_identifier), None)),
            },
            TranslationRule::Constant => match Scope::resolve_path(scope, &path)? {
                ScopeItem::Constant(constant) => {
                    let mut constant = constant.resolve()?;
                    constant.set_location(location);

                    let intermediate = GeneratorConstant::try_from_semantic(&constant)
                        .map(GeneratorExpressionOperand::Constant);

                    let element = Element::Constant(constant);
                    Ok((element, intermediate))
                }
                item => Err(Error::Expression(ExpressionError::NonConstantElement {
                    location: path.location,
                    found: item.to_string(),
                })),
            },

            TranslationRule::Type => match Scope::resolve_path(scope, &path)? {
                ScopeItem::Type(r#type) => {
                    let mut r#type = r#type.resolve()?;
                    r#type.set_location(location);

                    Ok((Element::Type(r#type), None))
                }
                _ => Ok((Element::Path(path), None)),
            },
            TranslationRule::Path => Ok((Element::Path(path), None)),
            TranslationRule::Field => Ok((Element::Identifier(path_last_identifier), None)),
        }
    }
}
