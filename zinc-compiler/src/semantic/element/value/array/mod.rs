//!
//! The semantic analyzer array value element.
//!

mod tests;

pub mod error;

use std::fmt;

use num_traits::Signed;
use num_traits::ToPrimitive;

use crate::lexical::token::location::Location;
use crate::semantic::element::access::Index as IndexAccess;
use crate::semantic::element::constant::range::Range as RangeConstant;
use crate::semantic::element::constant::range_inclusive::RangeInclusive as RangeInclusiveConstant;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::Value;

use self::error::Error;

///
/// Arrays are collections of elements of the same type.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    pub location: Option<Location>,
    pub r#type: Type,
    pub size: usize,
}

impl Array {
    pub fn new(location: Option<Location>) -> Self {
        Self {
            location,
            r#type: Type::unit(location),
            size: 0,
        }
    }

    pub fn new_with_values(location: Option<Location>, r#type: Type, size: usize) -> Self {
        Self {
            location,
            r#type,
            size,
        }
    }

    pub fn r#type(&self) -> Type {
        Type::array(self.location, self.r#type.to_owned(), self.size)
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.len() == other.len() && self.r#type == other.r#type
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, r#type: Type, location: Option<Location>) -> Result<(), Error> {
        if self.is_empty() {
            self.r#type = r#type;
        } else if r#type != self.r#type {
            return Err(Error::PushingInvalidType {
                location: location.unwrap(),
                expected: self.r#type.to_string(),
                found: r#type.to_string(),
            });
        }
        self.size += 1;

        Ok(())
    }

    pub fn extend(
        &mut self,
        r#type: Type,
        count: usize,
        location: Option<Location>,
    ) -> Result<(), Error> {
        if self.is_empty() {
            self.r#type = r#type;
        } else if r#type != self.r#type {
            return Err(Error::PushingInvalidType {
                location: location.unwrap(),
                expected: self.r#type.to_string(),
                found: r#type.to_string(),
            });
        }
        self.size += count;

        Ok(())
    }

    pub fn slice_single(self) -> (Value, IndexAccess) {
        let access = IndexAccess::new(self.r#type.size(), self.r#type().size(), None);

        let result = Value::try_from_type(&self.r#type, self.location)
            .expect(crate::panic::VALIDATED_DURING_SYNTAX_ANALYSIS);

        (result, access)
    }

    pub fn slice_range(self, range: RangeConstant) -> Result<(Value, IndexAccess), Error> {
        if range.start.is_negative() {
            return Err(Error::SliceStartOutOfRange {
                location: range.location,
                start: range.start.to_string(),
            });
        }

        let start = range
            .start
            .to_usize()
            .ok_or_else(|| Error::SliceStartOutOfRange {
                location: range.location,
                start: range.start.to_string(),
            })?;

        let end = range
            .end
            .to_usize()
            .ok_or_else(|| Error::SliceEndOutOfRange {
                location: range.location,
                end: range.end.to_string(),
                size: self.size,
            })?;

        if end < start {
            return Err(Error::SliceEndLesserThanStart {
                location: range.location,
                start: start.to_string(),
                end: end.to_string(),
            });
        }

        if end > self.size {
            return Err(Error::SliceEndOutOfRange {
                location: range.location,
                end: end.to_string(),
                size: self.size,
            });
        }

        let length = end - start;

        let access = IndexAccess::new(self.r#type.size() * length, self.r#type().size(), None);

        let result = Value::Array(Self::new_with_values(self.location, self.r#type, length));

        Ok((result, access))
    }

    pub fn slice_range_inclusive(
        self,
        range: RangeInclusiveConstant,
    ) -> Result<(Value, IndexAccess), Error> {
        if range.start.is_negative() {
            return Err(Error::SliceStartOutOfRange {
                location: range.location,
                start: range.start.to_string(),
            });
        }

        let start = range
            .start
            .to_usize()
            .ok_or_else(|| Error::SliceStartOutOfRange {
                location: range.location,
                start: range.start.to_string(),
            })?;

        let end = range
            .end
            .to_usize()
            .ok_or_else(|| Error::SliceEndOutOfRange {
                location: range.location,
                end: range.end.to_string(),
                size: self.size,
            })?;

        if end < start {
            return Err(Error::SliceEndLesserThanStart {
                location: range.location,
                start: start.to_string(),
                end: end.to_string(),
            });
        }

        if end >= self.size {
            return Err(Error::SliceEndOutOfRange {
                location: range.location,
                end: end.to_string(),
                size: self.size,
            });
        }

        let length = end - start + 1;

        let access = IndexAccess::new(self.r#type.size() * length, self.r#type().size(), None);

        let result = Value::Array(Self::new_with_values(self.location, self.r#type, length));

        Ok((result, access))
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<array> of '{}'s", self.r#type)
    }
}
