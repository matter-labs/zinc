//!
//! The tuple index semantic analyzer.
//!

use std::convert::TryFrom;

use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::syntax::tree::tuple_index::TupleIndex;

///
/// Analyzes the tuple field index, structure field identifier, or a path element.
///
/// Returns the semantic element and the intermediate representation.
///
pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes the field integer, which is a tuple field index.
    ///
    pub fn integer(integer: TupleIndex) -> Result<Element, Error> {
        let location = integer.location;

        let integer = IntegerConstant::try_from(&integer.literal)
            .map_err(|error| {
                Error::Element(
                    location,
                    ElementError::Constant(ConstantError::Integer(error)),
                )
            })?
            .to_usize()
            .map_err(|error| {
                Error::Element(
                    location,
                    ElementError::Constant(ConstantError::Integer(error)),
                )
            })?;

        Ok(Element::TupleIndex(integer))
    }
}
