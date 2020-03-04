//!
//! The semantic analyzer place element.
//!

mod tests;

pub mod error;

use std::fmt;
use std::ops::Deref;

use num_bigint::BigInt;
use num_traits::One;
use num_traits::Signed;
use num_traits::ToPrimitive;

use crate::lexical::Location;
use crate::semantic::element::access::AccessData;
use crate::semantic::element::constant::range::Range;
use crate::semantic::element::constant::range_inclusive::RangeInclusive;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;

use self::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Place {
    pub location: Location,
    pub identifier: String,
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
        identifier: String,
        r#type: Type,
        address: usize,
        is_mutable: bool,
        is_global: bool,
    ) -> Self {
        let total_size = r#type.size();
        Self {
            location,
            identifier,
            r#type,
            address,
            total_size,
            is_mutable,
            is_global,
            is_indexed: false,
        }
    }

    pub fn index(&mut self, index_value: &Element) -> Result<AccessData, Error> {
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

                self.identifier
                    .push_str(format!("[{}]", self.r#type).as_str());

                Ok(AccessData::new(
                    0,
                    inner_type_size,
                    array_size,
                    self.r#type.to_owned(),
                ))
            }
            Element::Constant(Constant::Range(Range { start, end, .. })) => {
                self.identifier
                    .push_str(format!("[{}..{}]", start, end).as_str());

                if start.is_negative() {
                    return Err(Error::ArraySliceStartOutOfRange {
                        start: start.to_string(),
                    });
                }
                if end > &BigInt::from(array_size) {
                    return Err(Error::ArraySliceEndOutOfRange {
                        end: end.to_string(),
                        size: array_size,
                    });
                }
                if end < start {
                    return Err(Error::ArraySliceEndLesserThanStart {
                        start: start.to_string(),
                        end: end.to_string(),
                    });
                }
                let start = start
                    .to_usize()
                    .ok_or_else(|| Error::ArraySliceStartOutOfRange {
                        start: start.to_string(),
                    })?;
                let length = (end - start).to_usize().ok_or_else(|| {
                    Error::ArraySliceEndLesserThanStart {
                        start: start.to_string(),
                        end: end.to_string(),
                    }
                })?;
                self.r#type = Type::array(inner_type, length);
                Ok(AccessData::new(
                    start,
                    inner_type_size,
                    array_size,
                    self.r#type.to_owned(),
                ))
            }
            Element::Constant(Constant::RangeInclusive(RangeInclusive { start, end, .. })) => {
                self.identifier
                    .push_str(format!("[{}..={}]", start, end).as_str());

                if start.is_negative() {
                    return Err(Error::ArraySliceStartOutOfRange {
                        start: start.to_string(),
                    });
                }
                if end >= &BigInt::from(array_size) {
                    return Err(Error::ArraySliceEndOutOfRange {
                        end: end.to_string(),
                        size: array_size,
                    });
                }
                if end < start {
                    return Err(Error::ArraySliceEndLesserThanStart {
                        start: start.to_string(),
                        end: end.to_string(),
                    });
                }
                let start = start
                    .to_usize()
                    .ok_or_else(|| Error::ArraySliceStartOutOfRange {
                        start: start.to_string(),
                    })?;
                let length = (end - start + BigInt::one()).to_usize().ok_or_else(|| {
                    Error::ArraySliceEndLesserThanStart {
                        start: start.to_string(),
                        end: end.to_string(),
                    }
                })?;
                self.r#type = Type::array(inner_type, length);
                Ok(AccessData::new(
                    start,
                    inner_type_size,
                    array_size,
                    self.r#type.to_owned(),
                ))
            }
            value => Err(Error::OperatorIndexSecondOperandExpectedIntegerOrRange(
                value.to_string(),
            )),
        }
    }

    pub fn field_tuple(&mut self, field_index: usize) -> Result<AccessData, Error> {
        self.is_indexed = true;

        let mut offset = 0;
        let total_size = self.r#type.size();
        match self.r#type {
            Type::Tuple { ref types } => {
                self.identifier
                    .push_str(format!(".{}", field_index).as_str());

                if field_index >= types.len() {
                    return Err(Error::TupleFieldDoesNotExist {
                        type_identifier: self.r#type.to_string(),
                        field_index,
                    });
                }
                let mut tuple_index = 0;
                while tuple_index < field_index {
                    offset += types[tuple_index].size();
                    tuple_index += 1;
                }
                self.r#type = types[tuple_index].to_owned();

                Ok(AccessData::new(
                    offset,
                    self.r#type.size(),
                    total_size,
                    self.r#type.to_owned(),
                ))
            }
            ref r#type => Err(Error::OperatorFieldFirstOperandExpectedTuple(
                r#type.to_string(),
            )),
        }
    }

    pub fn field_structure(&mut self, field_name: &str) -> Result<AccessData, Error> {
        self.is_indexed = true;

        let mut offset = 0;
        let total_size = self.r#type.size();
        match self.r#type {
            Type::Structure(ref structure) => {
                self.identifier
                    .push_str(format!(".{}", field_name).as_str());

                for structure_field in structure.fields.iter() {
                    if structure_field.0 == field_name {
                        self.r#type = structure_field.1.to_owned();

                        return Ok(AccessData::new(
                            offset,
                            self.r#type.size(),
                            total_size,
                            self.r#type.to_owned(),
                        ));
                    }
                    offset += structure_field.1.size();
                }
                Err(Error::StructureFieldDoesNotExist {
                    type_identifier: structure.identifier.to_owned(),
                    field_name: field_name.to_owned(),
                })
            }
            ref r#type => Err(Error::OperatorFieldFirstOperandExpectedStructure(
                r#type.to_string(),
            )),
        }
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}
