use crate::data::types::DataType;
use num_bigint::BigInt;
use serde_derive::{Deserialize, Serialize};
use num_traits::Num;

fn serialize_bigint_into_string<S>(bigint: &BigInt, serializer: S)
                                   -> Result<S::Ok, S::Error>
    where S: serde::Serializer
{
    let s = bigint.to_string();
    serializer.serialize_str(&s)
}

fn deserialize_bigint_from_string<'de, D>(deserializer: D)
                                          -> Result<BigInt, D::Error>
    where D: serde::Deserializer<'de>
{
    use serde::de::{Deserialize, Error};

    let str = String::deserialize(deserializer)?;
    BigInt::from_str_radix(&str, 10).map_err(|_| D::Error::invalid_value(
        serde::de::Unexpected::Str(&str),
        &"a decimal number"
    ))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructField {
    pub field: String,
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Unit,
    Scalar(
        #[serde(serialize_with = "serialize_bigint_into_string")]
        #[serde(deserialize_with = "deserialize_bigint_from_string")]
        BigInt
    ),
    Struct(Vec<StructField>),
    Array(Vec<Value>),
}

impl Value {
    pub fn default_from_type(data_type: &DataType) -> Self {
        match data_type {
            DataType::Unit => Value::Unit,
            DataType::Enum => Value::Scalar(0.into()),
            DataType::Primitive(_) => Value::Scalar(0.into()),
            DataType::Struct(fields) => Value::Struct(
                fields
                    .iter()
                    .map(|(name, data_type)| StructField {
                        field: name.clone(),
                        value: Value::default_from_type(data_type)
                    })
                    .collect(),
            ),
            DataType::Array(data_type, len) => {
                Value::Array(vec![Value::default_from_type(data_type); *len])
            }
            DataType::Tuple(fields) => {
                Value::Array(fields.iter().map(|t| Value::default_from_type(t)).collect())
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
            Value::Unit => {}
            Value::Scalar(value) => flat_array.push(value.clone()),
            Value::Struct(fields) => {
                for StructField { value, .. } in fields.iter() {
                    value.to_flat_values_recursive(flat_array);
                }
            }
            Value::Array(values) => {
                for value in values.iter() {
                    value.to_flat_values_recursive(flat_array);
                }
            }
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
            Value::Unit => Some(0),
            Value::Scalar(value) => {
                *value = flat_values.first()?.clone();
                Some(1)
            }
            Value::Struct(fields) => {
                let mut offset = 0;
                for StructField { value, .. } in fields.iter_mut() {
                    let slice = &flat_values[offset..];
                    offset += value.fill_from_flat_values(slice)?;
                }
                Some(offset)
            }
            Value::Array(values) => {
                let mut offset = 0;
                for value in values.iter_mut() {
                    let slice = &flat_values[offset..];
                    offset += value.fill_from_flat_values(slice)?;
                }
                Some(offset)
            }
        }
    }
}
