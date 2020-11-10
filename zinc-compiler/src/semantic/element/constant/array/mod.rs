//!
//! The semantic analyzer constant array element.
//!

#[cfg(test)]
mod tests;

use std::fmt;

use num::Signed;
use num::ToPrimitive;

use zinc_lexical::Location;

use crate::semantic::element::access::index::Index as IndexAccess;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::range::Range as RangeConstant;
use crate::semantic::element::constant::range_inclusive::RangeInclusive as RangeInclusiveConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;

///
/// Arrays are collections of elements of the same type.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    /// The location where the array appears in the code.
    pub location: Location,
    /// The array element type.
    pub r#type: Type,
    /// The array size.
    pub values: Vec<Constant>,
}

impl Array {
    ///
    /// A shortcut constructor, which is called before pushing the element values.
    ///
    /// At the beginning, the array has zero elements, and the array element type is defaulted to
    /// the `()` unit type.
    ///
    pub fn new(location: Location) -> Self {
        Self {
            location,
            r#type: Type::unit(Some(location)),
            values: vec![],
        }
    }

    ///
    /// A shortcut constructor, which is called when the type and values are already known.
    ///
    pub fn new_with_values(location: Location, r#type: Type, values: Vec<Constant>) -> Self {
        Self {
            location,
            r#type,
            values,
        }
    }

    ///
    /// The array size.
    ///
    pub fn len(&self) -> usize {
        self.values.len()
    }

    ///
    /// If the array has exactly zero elements.
    ///
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    ///
    /// Push an element to the array and check its type.
    ///
    /// The type is set when the first element is pushed. If a subsequent item type is not equal to
    /// the first element type, an error is returned.
    ///
    pub fn push(&mut self, value: Constant) -> Result<(), Error> {
        let r#type = value.r#type();
        if self.is_empty() {
            self.r#type = r#type;
        } else if r#type != self.r#type {
            return Err(Error::ArrayPushingInvalidType {
                location: value.location(),
                expected: self.r#type.to_string(),
                found: r#type.to_string(),
            });
        }
        self.values.push(value);

        Ok(())
    }

    ///
    /// Extends the array from `values`, pushing its elements one by one.
    ///
    pub fn extend(&mut self, values: Vec<Constant>) -> Result<(), Error> {
        for value in values.into_iter() {
            self.push(value)?;
        }

        Ok(())
    }

    ///
    /// Applies the index operator, getting a single element from the array.
    ///
    pub fn slice_single(
        mut self,
        index: Option<IntegerConstant>,
    ) -> Result<(Element, IndexAccess), Error> {
        match index {
            Some(index) => {
                let location = index.location;

                let index = index
                    .value
                    .to_usize()
                    .ok_or_else(|| Error::ArrayIndexOutOfRange {
                        location,
                        index: index.to_string(),
                        size: self.values.len(),
                    })?;

                if index >= self.values.len() {
                    return Err(Error::ArrayIndexOutOfRange {
                        location,
                        index: index.to_string(),
                        size: self.values.len(),
                    });
                }

                let access = IndexAccess::new(
                    self.r#type.size(),
                    1,
                    self.r#type().size(),
                    Some(self.r#type.size() * index),
                );
                let element = Element::Constant(self.values.remove(index));

                Ok((element, access))
            }
            None => {
                let access = IndexAccess::new(self.r#type.size(), 1, self.r#type().size(), None);
                let element = Value::try_from_type(&self.r#type, false, None)
                    .map(Element::Value)
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);

                Ok((element, access))
            }
        }
    }

    ///
    /// Applies the range operator, getting an array slice from the array.
    ///
    pub fn slice_range(mut self, range: RangeConstant) -> Result<(Constant, IndexAccess), Error> {
        if range.start.is_negative() {
            return Err(Error::ArraySliceStartOutOfRange {
                location: range.location,
                start: range.start.to_string(),
            });
        }

        let start = range
            .start
            .to_usize()
            .ok_or_else(|| Error::ArraySliceStartOutOfRange {
                location: range.location,
                start: range.start.to_string(),
            })?;

        let end = range
            .end
            .to_usize()
            .ok_or_else(|| Error::ArraySliceEndOutOfRange {
                location: range.location,
                end: range.end.to_string(),
                size: self.values.len(),
            })?;

        if end < start {
            return Err(Error::ArraySliceEndLesserThanStart {
                location: range.location,
                start: start.to_string(),
                end: end.to_string(),
            });
        }

        if end > self.values.len() {
            return Err(Error::ArraySliceEndOutOfRange {
                location: range.location,
                end: end.to_string(),
                size: self.values.len(),
            });
        }

        let slice_length = end - start;

        let access = IndexAccess::new(
            self.r#type.size(),
            slice_length,
            self.r#type().size(),
            Some(self.r#type.size() * start),
        );

        let result = Constant::Array(Self::new_with_values(
            self.location,
            self.r#type,
            self.values.drain(start..end).collect(),
        ));

        Ok((result, access))
    }

    ///
    /// Applies the inclusive range operator, getting an array slice from the array.
    ///
    pub fn slice_range_inclusive(
        mut self,
        range: RangeInclusiveConstant,
    ) -> Result<(Constant, IndexAccess), Error> {
        if range.start.is_negative() {
            return Err(Error::ArraySliceStartOutOfRange {
                location: range.location,
                start: range.start.to_string(),
            });
        }

        let start = range
            .start
            .to_usize()
            .ok_or_else(|| Error::ArraySliceStartOutOfRange {
                location: range.location,
                start: range.start.to_string(),
            })?;

        let end = range
            .end
            .to_usize()
            .ok_or_else(|| Error::ArraySliceEndOutOfRange {
                location: range.location,
                end: range.end.to_string(),
                size: self.values.len(),
            })?;

        if end < start {
            return Err(Error::ArraySliceEndLesserThanStart {
                location: range.location,
                start: start.to_string(),
                end: end.to_string(),
            });
        }

        if end >= self.values.len() {
            return Err(Error::ArraySliceEndOutOfRange {
                location: range.location,
                end: end.to_string(),
                size: self.values.len(),
            });
        }

        let slice_length = end - start + 1;

        let access = IndexAccess::new(
            self.r#type.size(),
            slice_length,
            self.r#type().size(),
            Some(self.r#type.size() * start),
        );

        let result = Constant::Array(Self::new_with_values(
            self.location,
            self.r#type,
            self.values.drain(start..=end).collect(),
        ));

        Ok((result, access))
    }
}

impl ITyped for Array {
    fn r#type(&self) -> Type {
        Type::array(
            Some(self.location),
            self.r#type.to_owned(),
            self.values.len(),
        )
    }

    fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.len() == other.len() && self.r#type == other.r#type
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
