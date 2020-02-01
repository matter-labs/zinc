//!
//! The semantic analyzer place element.
//!

mod error;

pub use self::error::Error;

use std::fmt;
use std::ops::Deref;

use num_bigint::BigInt;
use num_traits::One;
use num_traits::Signed;
use num_traits::ToPrimitive;

use crate::lexical::Location;
use crate::semantic::Constant;
use crate::semantic::Element;
use crate::semantic::FieldAccessResult;
use crate::semantic::IndexAccessResult;
use crate::semantic::RangeConstant;
use crate::semantic::RangeInclusiveConstant;
use crate::semantic::Type;
use crate::semantic::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Place {
    pub location: Location,
    pub r#type: Type,
    pub address: usize,
    pub total_size: usize,
    pub is_mutable: bool,
    pub is_global: bool,
    pub is_indexed: bool,
}

impl Place {
    pub fn new(
        location: Location,
        r#type: Type,
        address: usize,
        is_mutable: bool,
        is_global: bool,
    ) -> Self {
        let total_size = r#type.size();
        Self {
            location,
            r#type,
            address,
            total_size,
            is_mutable,
            is_global,
            is_indexed: false,
        }
    }

    pub fn index(&mut self, index_value: &Element) -> Result<IndexAccessResult, Error> {
        self.is_indexed = true;

        let (inner_type, array_size) = match self.r#type {
            Type::Array { ref r#type, size } => (r#type.deref().to_owned(), r#type.size() * size),
            ref r#type => {
                return Err(Error::OperatorIndexFirstOperandExpectedArray(
                    r#type.to_string(),
                ))
            }
        };

        let inner_type_size = inner_type.size();
        match index_value {
            Element::Value(Value::Integer(..)) | Element::Constant(Constant::Integer(..)) => {
                self.r#type = inner_type;
                Ok(IndexAccessResult::new(0, inner_type_size, array_size, None))
            }
            Element::Constant(Constant::Range(RangeConstant { start, end, .. })) => {
                if start.is_negative() {
                    return Err(Error::IndexSliceStartOutOfRange(start.to_string()));
                }
                if end > &BigInt::from(array_size) {
                    return Err(Error::IndexSliceEndOutOfRange(
                        end.to_string(),
                        array_size.to_string(),
                    ));
                }
                if end < start {
                    return Err(Error::IndexSliceEndLesserThanStart(
                        end.to_string(),
                        start.to_string(),
                    ));
                }
                let start = start
                    .to_usize()
                    .ok_or_else(|| Error::IndexSliceStartOutOfRange(start.to_string()))?;
                let length = (end - start).to_usize().ok_or_else(|| {
                    Error::IndexSliceEndLesserThanStart(end.to_string(), start.to_string())
                })?;
                self.r#type = Type::new_array(inner_type, length);
                Ok(IndexAccessResult::new(
                    start,
                    inner_type_size,
                    array_size,
                    None,
                ))
            }
            Element::Constant(Constant::RangeInclusive(RangeInclusiveConstant {
                start,
                end,
                ..
            })) => {
                if start.is_negative() {
                    return Err(Error::IndexSliceStartOutOfRange(start.to_string()));
                }
                if end >= &BigInt::from(array_size) {
                    return Err(Error::IndexSliceEndOutOfRange(
                        end.to_string(),
                        array_size.to_string(),
                    ));
                }
                if end < start {
                    return Err(Error::IndexSliceEndLesserThanStart(
                        end.to_string(),
                        start.to_string(),
                    ));
                }
                let start = start
                    .to_usize()
                    .ok_or_else(|| Error::IndexSliceStartOutOfRange(start.to_string()))?;
                let length = (end - start + BigInt::one()).to_usize().ok_or_else(|| {
                    Error::IndexSliceEndLesserThanStart(end.to_string(), start.to_string())
                })?;
                self.r#type = Type::new_array(inner_type, length);
                Ok(IndexAccessResult::new(
                    start,
                    inner_type_size,
                    array_size,
                    None,
                ))
            }
            value => Err(Error::OperatorIndexSecondOperandExpectedIntegerOrRange(
                value.to_string(),
            )),
        }
    }

    pub fn field_tuple(&mut self, field_index: usize) -> Result<FieldAccessResult, Error> {
        self.is_indexed = true;

        let mut offset = 0;
        let total_size = self.r#type.size();
        match self.r#type {
            Type::Tuple { ref types } => {
                if field_index >= types.len() {
                    return Err(Error::FieldDoesNotExistInTuple(
                        field_index,
                        self.r#type.to_string(),
                    ));
                }
                let mut tuple_index = 0;
                while tuple_index < field_index {
                    offset += types[tuple_index].size();
                    tuple_index += 1;
                }
                self.r#type = types[tuple_index].to_owned();

                Ok(FieldAccessResult::new(
                    offset,
                    self.r#type.size(),
                    total_size,
                    None,
                ))
            }
            ref r#type => Err(Error::OperatorFieldFirstOperandExpectedTuple(
                r#type.to_string(),
            )),
        }
    }

    pub fn field_structure(&mut self, field_name: &str) -> Result<FieldAccessResult, Error> {
        self.is_indexed = true;

        let mut offset = 0;
        let total_size = self.r#type.size();
        match self.r#type {
            Type::Structure {
                ref identifier,
                ref fields,
                ..
            } => {
                for structure_field in fields.iter() {
                    if structure_field.0 == field_name {
                        self.r#type = structure_field.1.to_owned();

                        return Ok(FieldAccessResult::new(
                            offset,
                            self.r#type.size(),
                            total_size,
                            None,
                        ));
                    }
                    offset += structure_field.1.size();
                }
                Err(Error::FieldDoesNotExistInStructure(
                    field_name.to_owned(),
                    identifier.to_string(),
                ))
            }
            ref r#type => Err(Error::OperatorFieldFirstOperandExpectedStructure(
                r#type.to_string(),
            )),
        }
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0")
    }
}
