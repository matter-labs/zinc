use num_bigint::BigInt;
use serde_derive::{Serialize, Deserialize};
use crate::data::types::DataType;

#[derive(Clone, Serialize, Deserialize)]
pub enum Value {
    Unit,
    Primitive(BigInt),
    Enum(BigInt),
    Struct(Vec<(String, Value)>),
    Tuple(Vec<Value>),
    Array(Vec<Value>),
}

impl Value {
    pub fn default_from_type(data_type: &DataType) -> Self {
        match data_type {
            DataType::Unit => {
                Value::Unit
            },
            DataType::Enum => {
                Value::Enum(0.into())
            }
            DataType::Primitive(_) => {
                Value::Primitive(0.into())
            },
            DataType::Struct(fields) => {
                Value::Struct(
                    fields
                        .iter()
                        .map(|(name, data_type)| {
                            (name.clone(), Value::default_from_type(data_type))
                        })
                        .collect()
                )
            },
            DataType::Array(data_type, len) => {
                Value::Array(vec![Value::default_from_type(data_type); *len])
            },
            DataType::Tuple(fields) => {
                Value::Tuple(
                    fields
                        .iter()
                        .map(|t| Value::default_from_type(t))
                        .collect()
                )
            }
        }
    }

    pub fn to_flat_values(&self) -> Vec<BigInt> {
        let mut flat_array = Vec::new();
        self.to_flat_values_recursive(&mut flat_array);
        flat_array
    }

    fn to_flat_values_recursive(&self, flat_array: &mut Vec<BigInt>) {
        match self {
            Value::Unit => {},
            Value::Primitive(value) => {
                flat_array.push(value.clone())
            },
            Value::Enum(value) => {
                flat_array.push(value.clone())
            },
            Value::Struct(fields) => {
                for (_name, value) in fields.iter() {
                    value.to_flat_values_recursive(flat_array);
                }
            },
            Value::Tuple(fields) => {
                for value in fields.iter() {
                    value.to_flat_values_recursive(flat_array);
                }
            },
            Value::Array(values) => {
                for value in values.iter() {
                    value.to_flat_values_recursive(flat_array);
                }
            },
        }
    }

    /// Creates value from flat array and data type.
    pub fn from_flat_values(data_type: &DataType, flat_values: &[BigInt]) -> Option<Self> {
        let mut value = Self::default_from_type(data_type);
        let consumed = value.fill_from_flat_values(flat_values)?;
        if consumed == flat_values.len() {
            Some(value)
        } else {
            None
        }
    }

    /// Fills values from slice, returns number of used values or None if there is not enough.
    fn fill_from_flat_values(&mut self, flat_values: &[BigInt]) -> Option<usize> {
        match self {
            Value::Unit => { Some(0) },
            Value::Primitive(value) => {
                *value = flat_values.first()?.clone();
                Some(1)
            },
            Value::Enum(value) => {
                *value = flat_values.first()?.clone();
                Some(1)
            },
            Value::Struct(fields) => {
                let mut offset = 0;
                for (_name, value) in fields.iter_mut() {
                    let slice = &flat_values[offset..];
                    offset += value.fill_from_flat_values(slice)?;
                }
                Some(offset)
            },
            Value::Tuple(fields) => {
                let mut offset = 0;
                for value in fields.iter_mut() {
                    let slice = &flat_values[offset..];
                    offset += value.fill_from_flat_values(slice)?;
                }
                Some(offset)
            },
            Value::Array(values) => {
                let mut offset = 0;
                for value in values.iter_mut() {
                    let slice = &flat_values[offset..];
                    offset += value.fill_from_flat_values(slice)?;
                }
                Some(offset)
            },
        }
    }
}
