//!
//! The member semantic analyzer.
//!

use std::convert::TryFrom;

use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::syntax::MemberInteger;
use crate::syntax::MemberString;

pub struct Analyzer {}

impl Analyzer {
    pub fn integer(integer: MemberInteger) -> Result<Element, Error> {
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

        Ok(Element::MemberInteger(integer))
    }

    pub fn string(member_string: MemberString) -> Result<Element, Error> {
        Ok(Element::MemberString(member_string))
    }
}
