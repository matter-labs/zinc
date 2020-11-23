//!
//! The semantic analyzer memory place element.
//!

#[cfg(test)]
mod tests;

pub mod element;
pub mod memory_type;

use std::fmt;
use std::ops::Deref;

use num::BigInt;
use num::Signed;
use num::ToPrimitive;

use zinc_syntax::Identifier;

use crate::semantic::element::access::dot::contract_field::ContractField as ContractFieldAccess;
use crate::semantic::element::access::dot::stack_field::StackField as StackFieldAccess;
use crate::semantic::element::access::dot::Dot as DotAccessVariant;
use crate::semantic::element::access::index::Index as IndexAccess;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::tuple_index::TupleIndex;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::item::Item as ScopeItem;

use self::element::Element as PlaceElement;
use self::memory_type::MemoryType;

///
/// The semantic analyzer memory place element.
///
#[derive(Debug, Clone)]
pub struct Place {
    /// The memory place identifier, which is usually a variable name.
    pub identifier: Identifier,
    /// The memory place type, which is changed each time we access an item deeper into the data structure.
    pub r#type: Type,
    /// The variable total size, which is not changed during indexing.
    pub total_size: usize,
    /// If the memory place, usually a variable, is declared as mutable.
    pub is_mutable: bool,
    /// The memory type, which the memory place is part of.
    pub memory_type: MemoryType,
    /// The memory place path, which consists of array indexes and fields accesses.
    pub elements: Vec<PlaceElement>,
}

impl Place {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        identifier: Identifier,
        mut r#type: Type,
        is_mutable: bool,
        memory_type: MemoryType,
    ) -> Self {
        r#type.set_location(identifier.location);
        let total_size = r#type.size();

        Self {
            identifier,
            r#type,
            total_size,
            is_mutable,
            memory_type,
            elements: vec![],
        }
    }

    ///
    /// Validates the array index or slice operator and changes the internal state.
    ///
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
                let access = IndexAccess::new(inner_type_size, 1, array_size, None);

                self.r#type = inner_type;

                Ok((self, access))
            }
            Element::Constant(Constant::Integer(_integer)) => {
                let access = IndexAccess::new(inner_type_size, 1, array_size, None);

                self.r#type = inner_type;

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

                let slice_length = (end - start).to_usize().ok_or_else(|| {
                    Error::ArraySliceEndLesserThanStart {
                        location: range.location,
                        start: start.to_string(),
                        end: end.to_string(),
                    }
                })?;

                let access = IndexAccess::new(inner_type_size, slice_length, array_size, None);

                self.r#type = Type::array(Some(self.identifier.location), inner_type, slice_length);

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

                let slice_length = (end - start + 1).to_usize().ok_or_else(|| {
                    Error::ArraySliceEndLesserThanStart {
                        location: range.location,
                        start: start.to_string(),
                        end: end.to_string(),
                    }
                })?;

                let access = IndexAccess::new(inner_type_size, slice_length, array_size, None);

                self.r#type = Type::array(Some(self.identifier.location), inner_type, slice_length);

                Ok((self, access))
            }
            value => Err(Error::OperatorIndexSecondOperandExpectedIntegerOrRange {
                location: value
                    .location()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                found: value.to_string(),
            }),
        }
    }

    ///
    /// Validates the tuple field access operator and changes the internal state.
    ///
    pub fn tuple_field(mut self, index: TupleIndex) -> Result<(Self, DotAccessVariant), Error> {
        let TupleIndex {
            location,
            value: index,
        } = index;

        let mut offset = 0;
        let total_size = self.r#type.size();
        match self.r#type {
            Type::Tuple(ref tuple) => {
                if index >= tuple.types.len() {
                    return Err(Error::TupleFieldOutOfRange {
                        location,
                        r#type: self.r#type.to_string(),
                        field_index: index,
                    });
                }

                let mut tuple_index = 0;
                while tuple_index < index {
                    offset += tuple.types[tuple_index].size();
                    tuple_index += 1;
                }

                let element_size = tuple.types[tuple_index].size();
                let access = DotAccessVariant::StackField(StackFieldAccess::new(
                    index.to_string(),
                    index,
                    offset,
                    element_size,
                    total_size,
                ));

                self.r#type = tuple.types[tuple_index].to_owned();

                Ok((self, access))
            }
            ref r#type => Err(Error::OperatorDotFirstOperandExpectedTuple {
                location: self.identifier.location,
                found: r#type.to_string(),
            }),
        }
    }

    ///
    /// Validates the structure field access operator and changes the internal state.
    ///
    pub fn structure_field(
        mut self,
        identifier: Identifier,
    ) -> Result<(Self, DotAccessVariant), Error> {
        let mut offset = 0;
        let total_size = self.r#type.size();

        match self.r#type {
            Type::Structure(ref structure) => {
                for (index, (field_name, field_type)) in structure.fields.iter().enumerate() {
                    let element_size = field_type.size();

                    if field_name == &identifier.name {
                        let access = DotAccessVariant::StackField(StackFieldAccess::new(
                            field_name.to_owned(),
                            index,
                            offset,
                            element_size,
                            total_size,
                        ));

                        self.r#type = field_type.to_owned();

                        return Ok((self, access));
                    }
                    offset += element_size;
                }

                Err(Error::StructureFieldDoesNotExist {
                    location: identifier.location,
                    r#type: structure.identifier.to_owned(),
                    field_name: identifier.name,
                })
            }
            Type::Contract(ref contract) => {
                let item = contract.scope.borrow().resolve_item(&identifier, false);

                if let Ok(item) = item {
                    if let ScopeItem::Field(ref field) = *item.borrow() {
                        let element_size = field.r#type.size();

                        let access = DotAccessVariant::ContractField(ContractFieldAccess::new(
                            identifier.name,
                            field.index,
                            offset,
                            element_size,
                            total_size,
                            field.is_immutable,
                            field.r#type.is_mtreemap(),
                        ));

                        self.r#type = field.r#type.to_owned();
                        self.total_size = self.r#type.size();
                        if field.is_immutable {
                            self.is_mutable = false;
                        }
                        self.memory_type = MemoryType::ContractStorage { index: field.index };

                        return Ok((self, access));
                    }
                }

                Err(Error::StructureFieldDoesNotExist {
                    location: identifier.location,
                    r#type: contract.identifier.to_owned(),
                    field_name: identifier.name,
                })
            }
            ref r#type => Err(Error::OperatorDotFirstOperandExpectedInstance {
                location: self.identifier.location,
                found: r#type.to_string(),
            }),
        }
    }

    ///
    /// Push a place path `element`, if it is already known and validated.
    ///
    pub fn push_element(&mut self, element: PlaceElement) {
        self.elements.push(element);
    }

    ///
    /// Whether the place path contains an immutable contract storage field.
    ///
    pub fn check_immutable_field(&self) -> Option<String> {
        for element in self.elements.iter() {
            match element {
                PlaceElement::ContractField { access } if access.is_immutable => {
                    return Some(access.name.to_owned())
                }
                _ => {}
            }
        }
        None
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
