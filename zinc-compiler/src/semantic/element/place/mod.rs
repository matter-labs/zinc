//!
//! The semantic analyzer place element.
//!

mod tests;

pub mod element;
pub mod error;
pub mod memory_type;

use std::fmt;
use std::ops::Deref;

use num_bigint::BigInt;
use num_traits::Signed;
use num_traits::ToPrimitive;

use crate::semantic::element::access::dot::contract_field::ContractField as ContractFieldAccess;
use crate::semantic::element::access::dot::stack_field::StackField as StackFieldAccess;
use crate::semantic::element::access::dot::Dot as DotAccessVariant;
use crate::semantic::element::access::index::Index as IndexAccess;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::tuple_index::TupleIndex;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::scope::item::variable::memory_type::MemoryType as VariableItemMemoryType;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::Scope;
use crate::syntax::tree::identifier::Identifier;

use self::element::Element as PlaceElement;
use self::error::Error;
use self::memory_type::MemoryType;

#[derive(Debug, Clone)]
pub struct Place {
    pub identifier: Identifier,
    pub r#type: Type,
    pub total_size: usize,
    pub is_mutable: bool,
    pub memory_type: MemoryType,
    pub elements: Vec<PlaceElement>,
}

impl Place {
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
                let access = IndexAccess::new(inner_type_size, array_size, None);

                self.r#type = inner_type;

                Ok((self, access))
            }
            Element::Constant(Constant::Integer(_integer)) => {
                let access = IndexAccess::new(inner_type_size, array_size, None);

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

                let length = (end - start).to_usize().ok_or_else(|| {
                    Error::ArraySliceEndLesserThanStart {
                        location: range.location,
                        start: start.to_string(),
                        end: end.to_string(),
                    }
                })?;

                let access = IndexAccess::new(inner_type_size, array_size, None);

                self.r#type = Type::array(Some(self.identifier.location), inner_type, length);

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

                let access = IndexAccess::new(inner_type_size, array_size, None);

                self.r#type = Type::array(Some(self.identifier.location), inner_type, length);

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

                let element_size = tuple.types[tuple_index].size();
                let access = DotAccessVariant::StackField(StackFieldAccess::new(
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
                    type_identifier: structure.identifier.to_owned(),
                    field_name: identifier.name,
                })
            }
            Type::Contract(ref contract) => {
                if let Ok(ScopeItem::Variable(variable)) =
                    Scope::resolve_item(contract.scope.clone(), &identifier, false)
                {
                    let position = match variable.memory_type {
                        VariableItemMemoryType::ContractStorage { index } => index,
                        _ => panic!(crate::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS),
                    };
                    let element_size = variable.r#type.size();
                    let access = DotAccessVariant::ContractField(ContractFieldAccess::new(
                        position,
                        element_size,
                    ));

                    self.r#type = variable.r#type;
                    self.total_size = self.r#type.size();

                    return Ok((self, access));
                }

                Err(Error::ContractFieldDoesNotExist {
                    location: identifier.location,
                    type_identifier: contract.identifier.to_owned(),
                    field_name: identifier.name,
                })
            }
            ref r#type => Err(Error::OperatorDotFirstOperandExpectedStructure {
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
