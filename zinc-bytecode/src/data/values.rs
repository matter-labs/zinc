use num_bigint::BigInt;
use serde_derive::{Serialize, Deserialize};
use crate::data::types::DataType;

#[derive(Clone, Serialize, Deserialize)]
pub struct PrimitiveValue {
    pub value: BigInt,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Enum {
    pub value: BigInt,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Struct {
    pub fields: Vec<(String, Value)>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Tuple {
    pub fields: Vec<Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Array {
    pub values: Vec<Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Value {
    Unit,
    Primitive(PrimitiveValue),
    Enum(Enum),
    Struct(Struct),
    Tuple(Tuple),
    Array(Array),
}

impl Value {
    fn default_from_type(data_type: &DataType) -> Self {
        match data_type {
            DataType::Unit => {
                Value::Unit
            },
            DataType::Enum => {
                Value::Enum(Enum { value: 0.into() })
            }
            DataType::Primitive(_) => {
                Value::Primitive(PrimitiveValue { value: 0.into() })
            },
            DataType::Struct(fields) => {
                Value::Struct(Struct {
                    fields: fields
                        .iter()
                        .map(|(name, data_type)| {
                            (name.clone(), Value::default_from_type(data_type))
                        })
                        .collect()
                })
            },
            DataType::Array(data_type, len) => {
                Value::Array(Array {
                    values: vec![Value::default_from_type(data_type); *len]
                })
            },
            DataType::Tuple(fields) => {
                Value::Tuple(Tuple {
                    fields: fields
                        .iter()
                        .map(|t| Value::default_from_type(t))
                        .collect()
                })
            }
        }
    }
}
