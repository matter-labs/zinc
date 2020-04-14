//!
//! The semantic analyzer place element.
//!

mod tests;

pub mod element;
pub mod error;

use std::fmt;
use std::ops::Deref;

use num_bigint::BigInt;
use num_traits::One;
use num_traits::Signed;
use num_traits::ToPrimitive;

use crate::semantic::element::access::Field as FieldAccess;
use crate::semantic::element::access::Index as IndexAccess;
use crate::semantic::element::constant::range::Range;
use crate::semantic::element::constant::range_inclusive::RangeInclusive;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::syntax::tree::identifier::Identifier;

use self::element::Element as PlaceElement;
use self::error::Error;

#[derive(Debug, Clone)]
pub struct Place {
    pub identifier: Identifier,
    pub r#type: Type,
    pub total_size: usize,
    pub is_mutable: bool,
    pub elements: Vec<PlaceElement>,
}

impl Place {
    pub fn new(identifier: Identifier, r#type: Type, is_mutable: bool) -> Self {
        let total_size = r#type.size();
        Self {
            identifier,
            r#type,
            total_size,
            is_mutable,

            elements: vec![],
        }
    }

    pub fn index(mut self, index_value: Element) -> Result<(Self, IndexAccess), Error> {
        let (inner_type, array_size) = match self.r#type {
            Type::Array { ref r#type, size } => (r#type.deref().to_owned(), r#type.size() * size),
            ref r#type => {
                return Err(Error::OperatorIndexFirstOperandExpectedArray {
                    found: r#type.to_string(),
                })
            }
        };

        let inner_type_size = inner_type.size();
        match index_value {
            Element::Value(Value::Integer(..)) => {
                self.r#type = inner_type;

                let access = IndexAccess::new(inner_type_size, array_size);

                Ok((self, access))
            }
            Element::Constant(Constant::Integer(_integer)) => {
                self.r#type = inner_type;

                let access = IndexAccess::new(inner_type_size, array_size);

                Ok((self, access))
            }
            Element::Constant(Constant::Range(Range { start, end, .. })) => {
                if start.is_negative() {
                    return Err(Error::ArraySliceStartOutOfRange {
                        start: start.to_string(),
                    });
                }

                if end > BigInt::from(array_size) {
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

                let length = (end.to_owned() - start).to_usize().ok_or_else(|| {
                    Error::ArraySliceEndLesserThanStart {
                        start: start.to_string(),
                        end: end.to_string(),
                    }
                })?;

                self.r#type = Type::array(inner_type, length);

                let access = IndexAccess::new(inner_type_size, array_size);

                Ok((self, access))
            }
            Element::Constant(Constant::RangeInclusive(RangeInclusive { start, end, .. })) => {
                if start.is_negative() {
                    return Err(Error::ArraySliceStartOutOfRange {
                        start: start.to_string(),
                    });
                }

                if end >= BigInt::from(array_size) {
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

                let length = (end.to_owned() - start + BigInt::one())
                    .to_usize()
                    .ok_or_else(|| Error::ArraySliceEndLesserThanStart {
                        start: start.to_string(),
                        end: end.to_string(),
                    })?;

                self.r#type = Type::array(inner_type, length);

                let access = IndexAccess::new(inner_type_size, array_size);

                Ok((self, access))
            }
            value => Err(Error::OperatorIndexSecondOperandExpectedIntegerOrRange {
                found: value.to_string(),
            }),
        }
    }

    pub fn field_tuple(mut self, field_index: usize) -> Result<(Self, FieldAccess), Error> {
        let mut offset = 0;
        let total_size = self.r#type.size();
        match self.r#type {
            Type::Tuple { ref types } => {
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

                let access = FieldAccess::new(field_index, offset, self.r#type.size(), total_size);

                Ok((self, access))
            }
            ref r#type => Err(Error::OperatorFieldFirstOperandExpectedTuple {
                found: r#type.to_string(),
            }),
        }
    }

    pub fn field_structure(mut self, field_name: String) -> Result<(Self, FieldAccess), Error> {
        let mut offset = 0;
        let total_size = self.r#type.size();
        match self.r#type {
            Type::Structure(ref structure) => {
                for (index, structure_field) in structure.fields.iter().enumerate() {
                    if structure_field.0 == field_name {
                        self.r#type = structure_field.1.to_owned();

                        let access =
                            FieldAccess::new(index, offset, self.r#type.size(), total_size);

                        return Ok((self, access));
                    }
                    offset += structure_field.1.size();
                }

                Err(Error::StructureFieldDoesNotExist {
                    type_identifier: structure.identifier.to_owned(),
                    field_name,
                })
            }
            ref r#type => Err(Error::OperatorFieldFirstOperandExpectedStructure {
                found: r#type.to_string(),
            }),
        }
    }

    pub fn push_element(&mut self, element: PlaceElement) {
        self.elements.push(element);
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.identifier.name,
            self.elements
                .iter()
                .map(|element| element.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}
