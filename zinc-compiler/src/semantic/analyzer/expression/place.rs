//!
//! The place expression translator.
//!

use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::element::place::Place;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;

///
/// The place expression translator.
///
pub struct Translator {}

impl Translator {
    ///
    /// Translates the place expression to a semantic expression type specified in `rule`.
    ///
    pub fn translate(
        place: Place,
        rule: TranslationRule,
    ) -> Result<(Element, Option<GeneratorExpressionOperand>), Error> {
        match rule {
            TranslationRule::Value => {
                let element = Value::try_from_place(&place).map(Element::Value)?;

                Ok((
                    element,
                    Some(GeneratorExpressionOperand::Place(place.into())),
                ))
            }
            _ => Ok((Element::Place(place), None)),
        }
    }
}
