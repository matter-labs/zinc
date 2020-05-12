//!
//! The semantic analyzer place element.
//!

mod tests;

pub mod element;
pub mod error;

use std::fmt;
use std::ops::Deref;

use num_bigint::BigInt;
use num_traits::Signed;
use num_traits::ToPrimitive;

use crate::semantic::element::access::field::Field as FieldAccess;
use crate::semantic::element::access::index::Index as IndexAccess;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::tuple_index::TupleIndex;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::Scope;
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
    pub fn new(identifier: Identifier, mut r#type: Type, is_mutable: bool) -> Self {
        r#type.set_location(identifier.location);
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
            Type::Array(ref array) => (
                array.r#type.deref().to_owned(),
                array.r#type.size() * array.size,
            ),
            ref r#type => {
                return Err(Error::OperatorIndexFirstOperandExpectedArray {
                    location: self.identifier.location,
                    found: r#type.to_string(),
                })
            }
        };

        let inner_type_size = inner_type.size();
        match index_value {
            Element::Value(Value::Integer(..)) => {
                self.r#type = inner_type;

                let access = IndexAccess::new(inner_type_size, array_size, None);

                Ok((self, access))
            }
            Element::Constant(Constant::Integer(_integer)) => {
                self.r#type = inner_type;

                let access = IndexAccess::new(inner_type_size, array_size, None);

                Ok((self, access))
            }
            Element::Constant(Constant::Range(range)) => {
                if range.start.is_negative() {
                    return Err(Error::ArraySliceStartOutOfRange {
                        location: range.location,
                        start: range.start.to_string(),
                    });
                }

                if range.end > BigInt::from(array_size) {
                    return Err(Error::ArraySliceEndOutOfRange {
                        location: range.location,
                        end: range.end.to_string(),
                        size: array_size,
                    });
                }

                let start =
                    range
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
                        size: array_size,
                    })?;

                if end < start {
                    return Err(Error::ArraySliceEndLesserThanStart {
                        location: range.location,
                        start: start.to_string(),
                        end: end.to_string(),
                    });
                }

                let length = (end - start).to_usize().ok_or_else(|| {
                    Error::ArraySliceEndLesserThanStart {
                        location: range.location,
                        start: start.to_string(),
                        end: end.to_string(),
                    }
                })?;

                self.r#type = Type::array(Some(self.identifier.location), inner_type, length);

                let access = IndexAccess::new(inner_type_size, array_size, None);

                Ok((self, access))
            }
            Element::Constant(Constant::RangeInclusive(range)) => {
                if range.start.is_negative() {
                    return Err(Error::ArraySliceStartOutOfRange {
                        location: range.location,
                        start: range.start.to_string(),
                    });
                }

                if range.end >= BigInt::from(array_size) {
                    return Err(Error::ArraySliceEndOutOfRange {
                        location: range.location,
                        end: range.end.to_string(),
                        size: array_size,
                    });
                }

                let start =
                    range
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
                        size: array_size,
                    })?;

                if end < start {
                    return Err(Error::ArraySliceEndLesserThanStart {
                        location: range.location,
                        start: start.to_string(),
                        end: end.to_string(),
                    });
                }

                let length = (end - start + 1).to_usize().ok_or_else(|| {
                    Error::ArraySliceEndLesserThanStart {
                        location: range.location,
                        start: start.to_string(),
                        end: end.to_string(),
                    }
                })?;

                self.r#type = Type::array(Some(self.identifier.location), inner_type, length);

                let access = IndexAccess::new(inner_type_size, array_size, None);

                Ok((self, access))
            }
            value => Err(Error::OperatorIndexSecondOperandExpectedIntegerOrRange {
                location: value
                    .location()
                    .expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                found: value.to_string(),
            }),
        }
    }

    pub fn field_tuple(mut self, index: TupleIndex) -> Result<(Self, FieldAccess), Error> {
        let TupleIndex {
            location,
            value: index,
        } = index;

        let mut offset = 0;
        let total_size = self.r#type.size();
        match self.r#type {
            Type::Tuple(ref tuple) => {
                if index >= tuple.types.len() {
                    return Err(Error::TupleFieldDoesNotExist {
                        location,
                        type_identifier: self.r#type.to_string(),
                        field_index: index,
                    });
                }

                let mut tuple_index = 0;
                while tuple_index < index {
                    offset += tuple.types[tuple_index].size();
                    tuple_index += 1;
                }

                self.r#type = tuple.types[tuple_index].to_owned();

                let access = FieldAccess::new(index, offset, self.r#type.size(), total_size);

                Ok((self, access))
            }
            ref r#type => Err(Error::OperatorFieldFirstOperandExpectedTuple {
                location: self.identifier.location,
                found: r#type.to_string(),
            }),
        }
    }

    pub fn field_structure(mut self, identifier: Identifier) -> Result<(Self, FieldAccess), Error> {
        let mut offset = 0;
        let total_size = self.r#type.size();
        match self.r#type {
            Type::Structure(ref structure) => {
                for (index, structure_field) in structure.fields.iter().enumerate() {
                    if structure_field.0 == identifier.name {
                        self.r#type = structure_field.1.to_owned();

                        let access =
                            FieldAccess::new(index, offset, self.r#type.size(), total_size);

                        return Ok((self, access));
                    }
                    offset += structure_field.1.size();
                }

                Err(Error::StructureFieldDoesNotExist {
                    location: identifier.location,
                    type_identifier: structure.identifier.to_owned(),
                    field_name: identifier.name,
                })
            }
            Type::Contract(ref contract) => {
                if let Ok(ScopeItem::Variable(variable)) =
                    Scope::resolve_item(contract.scope.clone(), &identifier, false)
                {
                    let element_size = variable.r#type.size();
                    self.r#type = variable.r#type;

                    let access = FieldAccess::new(0, 0, element_size, total_size);

                    return Ok((self, access));
                }

                Err(Error::ContractFieldDoesNotExist {
                    location: identifier.location,
                    type_identifier: contract.identifier.to_owned(),
                    field_name: identifier.name,
                })
            }
            ref r#type => Err(Error::OperatorFieldFirstOperandExpectedStructure {
                location: self.identifier.location,
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
