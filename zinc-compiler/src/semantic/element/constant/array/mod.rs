//!
//! The semantic analyzer constant array element.
//!

mod tests;

pub mod error;

use std::fmt;

use num_bigint::BigInt;
use num_traits::Signed;
use num_traits::ToPrimitive;

use crate::lexical::token::location::Location;
use crate::semantic::element::access::Index as IndexAccess;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::Type;

use self::error::Error;

///
/// Arrays are collections of elements of the same type.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    pub location: Location,
    pub r#type: Type,
    pub values: Vec<Constant>,
}

impl Array {
    pub fn new(location: Location) -> Self {
        Self {
            location,
            r#type: Type::Unit,
            values: vec![],
        }
    }

    pub fn new_with_values(location: Location, r#type: Type, values: Vec<Constant>) -> Self {
        Self {
            location,
            r#type,
            values,
        }
    }

    pub fn r#type(&self) -> Type {
        Type::array(self.r#type.to_owned(), self.values.len())
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.len() == other.len() && self.r#type == other.r#type
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, value: Constant) -> Result<(), Error> {
        let r#type = value.r#type();
        if self.is_empty() {
            self.r#type = r#type;
        } else if r#type != self.r#type {
            return Err(Error::PushingInvalidType {
                expected: self.r#type.to_string(),
                found: r#type.to_string(),
            });
        }
        self.values.push(value);

        Ok(())
    }

    pub fn extend(&mut self, values: Vec<Constant>) -> Result<(), Error> {
        for value in values.into_iter() {
            self.push(value)?;
        }

        Ok(())
    }

    pub fn slice_single(mut self, index: BigInt) -> Result<(Constant, IndexAccess), Error> {
        let index = index.to_usize().ok_or_else(|| Error::IndexOutOfRange {
            index: index.to_string(),
            size: self.values.len(),
        })?;

        if index >= self.values.len() {
            return Err(Error::IndexOutOfRange {
                index: index.to_string(),
                size: self.values.len(),
            });
        }

        let access = IndexAccess::new(
            self.r#type.size(),
            self.r#type().size(),
            Some(self.r#type.size() * index),
        );

        Ok((self.values.remove(index), access))
    }

    pub fn slice_range(
        mut self,
        start: BigInt,
        end: BigInt,
    ) -> Result<(Constant, IndexAccess), Error> {
        if start.is_negative() {
            return Err(Error::SliceStartOutOfRange {
                start: start.to_string(),
            });
        }

        let start = start
            .to_usize()
            .ok_or_else(|| Error::SliceStartOutOfRange {
                start: start.to_string(),
            })?;

        let end = end.to_usize().ok_or_else(|| Error::SliceEndOutOfRange {
            end: end.to_string(),
            size: self.values.len(),
        })?;

        if end < start {
            return Err(Error::SliceEndLesserThanStart {
                start: start.to_string(),
                end: end.to_string(),
            });
        }

        if end > self.values.len() {
            return Err(Error::SliceEndOutOfRange {
                end: end.to_string(),
                size: self.values.len(),
            });
        }

        let length = end - start;

        let access = IndexAccess::new(
            self.r#type.size() * length,
            self.r#type().size(),
            Some(self.r#type.size() * start),
        );

        Ok((
            Constant::Array(Self::new_with_values(
                self.location,
                self.r#type,
                self.values.drain(start..end).collect(),
            )),
            access,
        ))
    }

    pub fn slice_range_inclusive(
        mut self,
        start: BigInt,
        end: BigInt,
    ) -> Result<(Constant, IndexAccess), Error> {
        if start.is_negative() {
            return Err(Error::SliceStartOutOfRange {
                start: start.to_string(),
            });
        }

        let start = start
            .to_usize()
            .ok_or_else(|| Error::SliceStartOutOfRange {
                start: start.to_string(),
            })?;

        let end = end.to_usize().ok_or_else(|| Error::SliceEndOutOfRange {
            end: end.to_string(),
            size: self.values.len(),
        })?;

        if end < start {
            return Err(Error::SliceEndLesserThanStart {
                start: start.to_string(),
                end: end.to_string(),
            });
        }

        if end >= self.values.len() {
            return Err(Error::SliceEndOutOfRange {
                end: end.to_string(),
                size: self.values.len(),
            });
        }

        let length = end - start + 1;

        let access = IndexAccess::new(
            self.r#type.size() * length,
            self.r#type().size(),
            Some(self.r#type.size() * start),
        );

        Ok((
            Constant::Array(Self::new_with_values(
                self.location,
                self.r#type,
                self.values.drain(start..=end).collect(),
            )),
            access,
        ))
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[ {} ]",
            self.values
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
