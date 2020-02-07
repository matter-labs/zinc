use num_bigint::BigInt;
use num_traits::Num;
use serde_derive::{Deserialize, Serialize};
use serde_json as json;

use crate::data::types::{DataType, IntegerType, ScalarType};
use failure::Fail;
use std::collections::HashSet;
use std::fmt;

#[allow(dead_code)]
fn serialize_bigint_into_string<S>(bigint: &BigInt, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let s = bigint.to_string();
    serializer.serialize_str(&s)
}

#[allow(dead_code)]
fn deserialize_bigint_from_string<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{Deserialize, Error};

    let str = String::deserialize(deserializer)?;
    BigInt::from_str_radix(&str, 10)
        .map_err(|_| D::Error::invalid_value(serde::de::Unexpected::Str(&str), &"a decimal number"))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructField {
    pub field: String,
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScalarValue {
    Field(BigInt),
    Bool(bool),
    Integer(BigInt),
}

impl ScalarValue {
    pub fn to_bigint(&self) -> BigInt {
        match self {
            ScalarValue::Field(value) | ScalarValue::Integer(value) => value.clone(),
            ScalarValue::Bool(value) => {
                if *value {
                    BigInt::from(1)
                } else {
                    BigInt::from(0)
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Unit,
    Scalar(ScalarValue),
    Struct(Vec<StructField>),
    Array(Vec<Value>),
}

impl Value {
    pub fn default_from_type(data_type: &DataType) -> Self {
        match data_type {
            DataType::Unit => Value::Unit,
            DataType::Enum => Value::Scalar(ScalarValue::Field(0.into())),
            DataType::Scalar(scalar_type) => match scalar_type {
                ScalarType::Field => Value::Scalar(ScalarValue::Field(0.into())),
                ScalarType::Boolean => Value::Scalar(ScalarValue::Bool(false)),
                ScalarType::Integer(_) => Value::Scalar(ScalarValue::Integer(0.into())),
            },
            DataType::Struct(fields) => Value::Struct(
                fields
                    .iter()
                    .map(|(name, data_type)| StructField {
                        field: name.clone(),
                        value: Value::default_from_type(data_type),
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
            Value::Scalar(value) => flat_array.push(value.to_bigint()),
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
            Value::Scalar(scalar) => {
                match scalar {
                    ScalarValue::Field(value) | ScalarValue::Integer(value) => {
                        *value = flat_values.first()?.clone();
                    }
                    ScalarValue::Bool(value) => {
                        *value = flat_values.first()? != &BigInt::from(0);
                    }
                }
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

// Pretty json de/serialization
impl Value {
    pub fn to_json(&self) -> json::Value {
        match self {
            Value::Unit => json::Value::String("unit".into()),
            Value::Scalar(scalar) => match scalar {
                ScalarValue::Field(value) | ScalarValue::Integer(value) => {
                    if value <= &BigInt::from(std::u64::MAX) {
                        json::Value::String(value.to_str_radix(10))
                    } else {
                        json::Value::String(String::from("0x") + value.to_str_radix(16).as_str())
                    }
                }
                ScalarValue::Bool(value) => json::Value::Bool(*value),
            },
            Value::Struct(fields) => {
                let mut object = json::Map::<String, serde_json::Value>::new();
                for field in fields.iter() {
                    object.insert(field.field.clone(), field.value.to_json());
                }
                json::Value::Object(object)
            }
            Value::Array(values) => json::Value::Array(values.iter().map(Self::to_json).collect()),
        }
    }

    pub fn from_typed_json(value: &json::Value, dtype: &DataType) -> Result<Self, JsonValueError> {
        match dtype {
            DataType::Unit => Self::unit_from_json(value),
            DataType::Scalar(t) => Self::scalar_from_json(value, t),
            DataType::Enum => Self::field_from_json(value),
            DataType::Struct(fields) => Self::struct_from_json(value, fields),
            DataType::Tuple(dtype) => Self::tuple_from_json(value, dtype),
            DataType::Array(dtype, size) => Self::array_from_json(value, dtype, *size),
        }
    }

    fn unit_from_json(value: &json::Value) -> Result<Self, JsonValueError> {
        if let Some(s) = value.as_str() {
            if s == "unit" {
                return Ok(Value::Unit);
            }
        }
        Err(JsonValueErrorType::TypeError {
            expected: "\"unit\"".into(),
            actual: value.to_string(),
        }
        .into())
    }

    fn scalar_from_json(
        value: &json::Value,
        scalar_type: &ScalarType,
    ) -> Result<Self, JsonValueError> {
        match scalar_type {
            ScalarType::Field => Self::field_from_json(value),
            ScalarType::Boolean => Self::boolean_from_json(value),
            ScalarType::Integer(itype) => Self::integer_from_json(value, itype),
        }
    }

    fn field_from_json(value: &json::Value) -> Result<Self, JsonValueError> {
        let value_string = value
            .as_str()
            .ok_or_else(|| JsonValueErrorType::TypeError {
                expected: "field (number string)".into(),
                actual: value.to_string(),
            })?;

        let bigint_result = if value_string.starts_with("0x") {
            BigInt::from_str_radix(&value_string[2..], 16)
        } else {
            BigInt::from_str_radix(value_string, 10)
        };

        let bigint = bigint_result
            .map_err(|_| JsonValueErrorType::InvalidNumberFormat(value_string.into()))?;

        // TODO: overflow check.

        Ok(Value::Scalar(ScalarValue::Field(bigint)))
    }

    fn boolean_from_json(value: &json::Value) -> Result<Self, JsonValueError> {
        let value_bool = value
            .as_bool()
            .ok_or_else(|| JsonValueErrorType::TypeError {
                expected: "boolean (true or false)".into(),
                actual: value.to_string(),
            })?;

        Ok(Value::Scalar(ScalarValue::Bool(value_bool)))
    }

    fn integer_from_json(
        value: &json::Value,
        _itype: &IntegerType,
    ) -> Result<Self, JsonValueError> {
        // TODO: overflow check.
        Self::field_from_json(value)
    }

    fn struct_from_json(
        value: &json::Value,
        field_types: &[(String, DataType)],
    ) -> Result<Self, JsonValueError> {
        let object = value
            .as_object()
            .ok_or_else(|| JsonValueErrorType::type_error("structure", value))?;

        let mut used_fields = HashSet::<&str>::new();
        let mut field_values = Vec::with_capacity(field_types.len());
        for (name, dtype) in field_types {
            used_fields.insert(name.as_str());

            let json_value = object
                .get(name)
                .ok_or_else(|| JsonValueErrorType::MissingField(name.clone()))?;

            let typed_value = Self::from_typed_json(json_value, dtype).in_struct(name.as_str())?;

            field_values.push(StructField {
                field: name.clone(),
                value: typed_value,
            })
        }

        for field in object.keys() {
            if !used_fields.contains(field.as_str()) {
                return Err(JsonValueErrorType::UnexpectedField(field.clone()).into());
            }
        }

        Ok(Value::Struct(field_values))
    }

    fn tuple_from_json(value: &json::Value, types: &[DataType]) -> Result<Self, JsonValueError> {
        let array = value
            .as_array()
            .ok_or_else(|| JsonValueErrorType::type_error("tuple (json array)", value))?;

        if array.len() != types.len() {
            return Err(JsonValueErrorType::UnexpectedSize {
                expected: types.len(),
                actual: array.len(),
            }
            .into());
        }

        let mut values = Vec::with_capacity(types.len());
        for (index, (value, dtype)) in array.iter().zip(types).enumerate() {
            let typed_value = Self::from_typed_json(value, dtype).in_array(index)?;
            values.push(typed_value);
        }

        Ok(Value::Array(values))
    }

    fn array_from_json(
        value: &json::Value,
        dtype: &DataType,
        size: usize,
    ) -> Result<Self, JsonValueError> {
        let array = value
            .as_array()
            .ok_or_else(|| JsonValueErrorType::type_error("array", value))?;

        if array.len() != size {
            return Err(JsonValueErrorType::UnexpectedSize {
                expected: size,
                actual: array.len(),
            }
            .into());
        }

        let mut values = Vec::with_capacity(size);
        for (index, value) in array.iter().enumerate() {
            let typed_value = Self::from_typed_json(value, dtype).in_array(index)?;

            values.push(typed_value);
        }

        Ok(Value::Array(values))
    }
}

#[derive(Debug, Fail)]
pub struct JsonValueError {
    path: Vec<String>,
    error: JsonValueErrorType,
}

impl fmt::Display for JsonValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut path = self.path.clone();
        path.reverse();
        let p = path.as_slice().join(&String::from("."));
        write!(f, "{} at {}", self.error, p)
    }
}

trait JsonErrorContext {
    fn in_struct(self, name: &str) -> Self;
    fn in_array(self, index: usize) -> Self;
}

// Propagate error location
impl<T> JsonErrorContext for Result<T, JsonValueError> {
    fn in_struct(self, name: &str) -> Self {
        self.map_err(|mut e| {
            e.path.push(name.into());
            e
        })
    }

    fn in_array(self, index: usize) -> Self {
        self.map_err(|mut e| {
            e.path.push(format!("[{}]", index));
            e
        })
    }
}

impl From<JsonValueErrorType> for JsonValueError {
    fn from(error: JsonValueErrorType) -> Self {
        Self {
            path: Vec::new(),
            error,
        }
    }
}

#[derive(Debug, Fail)]
pub enum JsonValueErrorType {
    #[fail(display = "unexpected null value")]
    UnexpectedNull,

    #[fail(display = "type mismatch: expected {}, got {}", expected, actual)]
    TypeError { expected: String, actual: String },

    #[fail(
        display = "failed to parse number: expected decimal or hexadecimal string, got \"{}\"",
        _0
    )]
    InvalidNumberFormat(String),

    #[fail(display = "value for field \"{}\" is missing", _0)]
    MissingField(String),

    #[fail(display = "unexpected field \"{}\"", _0)]
    UnexpectedField(String),

    #[fail(
        display = "expected array/tuple of size {}, got {} elements",
        expected, actual
    )]
    UnexpectedSize { expected: usize, actual: usize },
}

impl JsonValueErrorType {
    fn type_error(expected: &str, actual: &json::Value) -> Self {
        let actual_string: String = match actual {
            json::Value::Null => "null".into(),
            json::Value::Bool(value) => format!("boolean ({})", value),
            json::Value::Number(value) => format!("number ({})", value),
            json::Value::String(value) => format!("string (\"{}\")", value),
            json::Value::Array(_) => "array".into(),
            json::Value::Object(_) => "structure".into(),
        };

        JsonValueErrorType::TypeError {
            expected: expected.into(),
            actual: actual_string,
        }
    }
}
