//!
//! The semantic analyzer array element value.
//!

mod tests;

pub mod error;

use std::fmt;

use num_bigint::BigInt;
use num_traits::One;
use num_traits::Signed;
use num_traits::ToPrimitive;

use crate::semantic::element::access::AccessData;
use crate::semantic::element::r#type::Type;

use self::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    r#type: Type,
    size: usize,
}

impl Default for Array {
    fn default() -> Self {
        Self {
            r#type: Type::Unit,
            size: 0,
        }
    }
}

impl Array {
    pub fn new(r#type: Type, size: usize) -> Self {
        Self { r#type, size }
    }

    pub fn r#type(&self) -> Type {
        Type::array(self.r#type.to_owned(), self.size)
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

    pub fn push(&mut self, r#type: Type) -> Result<(), Error> {
        if self.size == 0 {
            self.r#type = r#type;
        } else if r#type != self.r#type {
            return Err(Error::PushingInvalidType {
                expected: self.r#type.to_string(),
                found: r#type.to_string(),
            });
        }
        self.size += 1;

        Ok(())
    }

    pub fn extend(&mut self, r#type: Type, count: usize) -> Result<(), Error> {
        if self.size == 0 {
            self.r#type = r#type;
        } else if r#type != self.r#type {
            return Err(Error::PushingInvalidType {
                expected: self.r#type.to_string(),
                found: r#type.to_string(),
            });
        }
        self.size += count;

        Ok(())
    }

    pub fn slice_single(self) -> (Self, AccessData) {
        let access = AccessData::new(
            0,
            0,
            self.r#type.size(),
            self.r#type().size(),
            self.r#type.to_owned(),
        );

        (self, access)
    }

    pub fn slice_range(self, start: BigInt, end: BigInt) -> Result<(Self, AccessData), Error> {
        if start.is_negative() {
            return Err(Error::SliceStartOutOfRange {
                start: start.to_string(),
            });
        }

        if end > BigInt::from(self.size) {
            return Err(Error::SliceEndOutOfRange {
                end: end.to_string(),
                size: self.size,
            });
        }

        if end < start {
            return Err(Error::SliceEndLesserThanStart {
                start: start.to_string(),
                end: end.to_string(),
            });
        }

        let start = start
            .to_usize()
            .ok_or_else(|| Error::SliceStartOutOfRange {
                start: start.to_string(),
            })?;

        let length =
            (end.clone() - start)
                .to_usize()
                .ok_or_else(|| Error::SliceEndLesserThanStart {
                    start: start.to_string(),
                    end: end.to_string(),
                })?;

        let access = AccessData::new(
            start,
            self.r#type.size() * start,
            self.r#type.size() * length,
            self.r#type().size(),
            Type::array(self.r#type.to_owned(), length),
        );

        Ok((self, access))
    }

    pub fn slice_range_inclusive(
        self,
        start: BigInt,
        end: BigInt,
    ) -> Result<(Self, AccessData), Error> {
        if start.is_negative() {
            return Err(Error::SliceStartOutOfRange {
                start: start.to_string(),
            });
        }

        if end >= BigInt::from(self.size) {
            return Err(Error::SliceEndOutOfRange {
                end: end.to_string(),
                size: self.size,
            });
        }

        if end < start {
            return Err(Error::SliceEndLesserThanStart {
                start: start.to_string(),
                end: end.to_string(),
            });
        }

        let start = start
            .to_usize()
            .ok_or_else(|| Error::SliceStartOutOfRange {
                start: start.to_string(),
            })?;

        let length = (end.clone() - start + BigInt::one())
            .to_usize()
            .ok_or_else(|| Error::SliceEndLesserThanStart {
                start: start.to_string(),
                end: end.to_string(),
            })?;

        let access = AccessData::new(
            start,
            self.r#type.size() * start,
            self.r#type.size() * length,
            self.r#type().size(),
            Type::array(self.r#type.to_owned(), length),
        );

        Ok((self, access))
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "array of '{}'s", self.r#type)
    }
}
