//!
//! The place expression translator.
//!

use std::convert::TryFrom;

use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::semantic::analyzer::expression::hint::Hint as TranslationHint;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::place::Place;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;

pub struct Translator {}

impl Translator {
    ///
    /// Translates the place expression to a semantic expression type specified in `hint`.
    ///
    pub fn translate(
        place: Place,
        hint: TranslationHint,
    ) -> Result<(Element, Option<GeneratorExpressionOperand>), Error> {
        match hint {
            TranslationHint::Value => {
                let element = Value::try_from(&place.r#type)
                    .map(Element::Value)
                    .map_err(ElementError::Value)
                    .map_err(|error| Error::Element(place.identifier.location, error))?;
                Ok((
                    element,
                    Some(GeneratorExpressionOperand::Place(place.into())),
                ))
            }
            _ => Ok((Element::Place(place), None)),
        }
    }
}
